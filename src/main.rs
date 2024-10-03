use actix_web::{App, HttpServer, web};
use actix_web::rt::Runtime;
use anyhow::Result;
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};
use tonic::transport::{Identity, Server, ServerTlsConfig};

use crate::grpc::echo;

mod config;
mod host;
mod http;
mod request;
mod response;
mod grpc;
mod middleware;

mod proto {
    tonic::include_proto!("echo");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("echo_descriptor");
}

fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let runtime = Runtime::new()?;

    runtime.spawn(async {
        HttpServer::new(|| App::new().service(web::resource("/metrics").to(http::metrics_handler)))
            .bind(format!("0.0.0.0:{}", config::APP_CONFIG.metrics.port))?
            .run()
            .await
    });

    if config::APP_CONFIG.enable.grpc {
        runtime.spawn(async {
            let echo_service = grpc::EchoServer::default();
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


            Server::builder()
                .layer(layer)
                .add_service(reflection_service)
                .add_service(echo::echo_server::EchoServer::new(echo_service))
                .serve(addr)
                .await
        });
    }

    if config::APP_CONFIG.enable.grpc_tls {
        let cert = std::fs::read_to_string(config::APP_CONFIG.grpc.tls.certificate_chain_file.clone())?;
        let key = std::fs::read_to_string(config::APP_CONFIG.grpc.tls.private_key_file.clone())?;

        let identity = Identity::from_pem(cert, key);

        runtime.spawn(async {
            let echo_service = grpc::EchoServer::default();
            let reflection_service = tonic_reflection::server::Builder::configure()
                .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
                .build_v1alpha()
                .unwrap();

            let addr = format!("[::1]:{}", config::APP_CONFIG.grpc.tls.port).parse().unwrap();
            log::info!("GRPC Over TLS listening on {}", addr);


            // The stack of middleware that our service will be wrapped in
            let layer = tower::ServiceBuilder::new()
                // Apply our own middleware
                .layer(middleware::grpc::MetricsMiddlewareLayer::default())
                .into_inner();


            Server::builder()
                .tls_config(ServerTlsConfig::new().identity(identity))?
                .layer(layer)
                .add_service(reflection_service)
                .add_service(echo::echo_server::EchoServer::new(echo_service))
                .serve(addr)
                .await
        });
    }


    if config::APP_CONFIG.enable.https {
        match init_ssl() {
            Ok(builder) => {
                runtime.spawn(async {
                    HttpServer::new(|| {
                        App::new()
                            .wrap(middleware::http::Metrics)
                            .service(web::resource("/{any:.*}").to(http::echo))
                    })
                        .bind_openssl(
                            format!("0.0.0.0:{}", config::APP_CONFIG.https.port),
                            builder,
                        )?
                        .run()
                        .await
                });
            }
            Err(e) => {
                log::warn!("starting HTTPS server error: {}, skip", e);
            }
        }
    }

    runtime.block_on(async {
        HttpServer::new(|| {
            App::new()
                .wrap(middleware::http::Metrics)
                .service(web::resource("/{any:.*}").to(http::echo))
        })
            .bind(format!("0.0.0.0:{}", config::APP_CONFIG.http.port))?
            .run()
            .await
    })
}

fn init_ssl() -> Result<SslAcceptorBuilder> {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;
    builder.set_private_key_file(
        config::APP_CONFIG.https.private_key_file.clone(),
        SslFiletype::PEM,
    )?;
    builder.set_certificate_chain_file(config::APP_CONFIG.https.certificate_chain_file.clone())?;
    Ok(builder)
}
