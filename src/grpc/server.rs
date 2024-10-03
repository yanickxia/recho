use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;

use anyhow::Result;
use hyper::http;
use hyper::server::conn::http2::Builder;
use hyper_util::{
    rt::{TokioExecutor, TokioIo},
    service::TowerToHyperService,
};
use rustls_pemfile::certs;
use tokio::net::TcpListener;
use tokio::task::JoinHandle;
use tokio_rustls::{
    rustls::{pki_types::CertificateDer, ServerConfig},
    TlsAcceptor,
};
use tonic::{body::boxed, service::Routes};
use tonic::transport::Server;
use tower::ServiceExt;
use tower_http::ServiceBuilderExt;

use crate::{config, middleware};
use crate::grpc::method;
use crate::grpc::method::echo;

mod proto {
    tonic::include_proto!("echo");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("echo_descriptor");
}

pub fn run_grpc_tls() -> JoinHandle<()> {
    let certs = {
        let fd = File::open(config::APP_CONFIG.grpc.tls.certificate_chain_file.clone()).unwrap();
        let mut buf = BufReader::new(&fd);
        certs(&mut buf).collect::<Result<Vec<_>, _>>().unwrap()
    };

    let key = {
        let fd = std::fs::File::open(config::APP_CONFIG.grpc.tls.private_key_file.clone()).unwrap();
        let mut buf = BufReader::new(&fd);
        rustls_pemfile::private_key(&mut buf)
            .unwrap()
            .unwrap()
    };

    let mut tls = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, key)
        .unwrap();
    tls.alpn_protocols = vec![b"h2".to_vec()];

    let server = method::EchoServer::default();

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build_v1alpha()
        .unwrap();


    let svc = Routes::new(echo::echo_server::EchoServer::new(server))
        .add_service(reflection_service)
        .prepare();

    let http = Builder::new(TokioExecutor::new());

    tokio::spawn(async move {
        let listener = TcpListener::bind(format!("[::1]:{}", config::APP_CONFIG.grpc.tls.port)).await.expect("can't listening tls");
        let tls_acceptor = TlsAcceptor::from(Arc::new(tls));

        loop {
            let (conn, addr) = match listener.accept().await {
                Ok(incoming) => incoming,
                Err(e) => {
                    log::error!("Error accepting connection: {}", e);
                    continue;
                }
            };

            let http = http.clone();
            let tls_acceptor = tls_acceptor.clone();
            let svc = svc.clone();

            tokio::spawn(async move {
                let mut certificates = Vec::new();

                let conn = tls_acceptor
                    .accept_with(conn, |info| {
                        if let Some(certs) = info.peer_certificates() {
                            for cert in certs {
                                certificates.push(cert.clone());
                            }
                        }
                    })
                    .await
                    .unwrap();

                let svc = tower::ServiceBuilder::new()
                    .add_extension(Arc::new(ConnInfo { addr, certificates }))
                    .layer(middleware::grpc::MetricsMiddlewareLayer::default())
                    .service(svc);

                http.serve_connection(
                    TokioIo::new(conn),
                    TowerToHyperService::new(svc.map_request(|req: http::Request<_>| req.map(boxed))),
                )
                    .await
                    .expect("unexpect exit");
            });
        }
    })
}
pub fn run_grpc() -> JoinHandle<()> {
    let echo_service = method::EchoServer::default();
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build_v1alpha()
        .unwrap();

    let addr = format!("[::1]:{}", config::APP_CONFIG.grpc.port).parse().unwrap();
    log::info!("GRPC listening on {}", addr);

    // The stack of middleware that our service will be wrapped in
    let layer = tower::ServiceBuilder::new()
        // Apply our own middleware
        .layer(middleware::grpc::MetricsMiddlewareLayer::default())
        .into_inner();


    tokio::spawn(async move {
        Server::builder()
            .layer(layer)
            .add_service(reflection_service)
            .add_service(echo::echo_server::EchoServer::new(echo_service))
            .serve(addr)
            .await
            .expect("unexpect exit");
    })
}

#[allow(dead_code)]
#[derive(Debug)]
struct ConnInfo {
    addr: std::net::SocketAddr,
    certificates: Vec<CertificateDer<'static>>,
}
