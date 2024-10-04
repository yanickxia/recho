use std::fs::File;
use std::io::BufReader;

use actix_web::{web, App, HttpServer};
use anyhow::Result;
use rustls::pki_types::PrivateKeyDer;
use rustls::ServerConfig;
use rustls_pemfile::{certs, pkcs8_private_keys};
use tokio::task::JoinHandle;

use crate::config;
use crate::http::method;
use crate::middleware;

pub fn run_https() -> JoinHandle<()> {
    tokio::spawn(async {
        HttpServer::new(|| {
            App::new()
                .wrap(middleware::http::Metrics)
                .service(web::resource("/{any:.*}").to(method::echo))
        })
        .bind_rustls_0_23(
            format!("0.0.0.0:{}", config::APP_CONFIG.https.port),
            load_rustls_config().unwrap(),
        )
        .unwrap()
        .run()
        .await
        .expect("unexpect exit");
    })
}

pub fn run_http() -> JoinHandle<()> {
    tokio::spawn(async {
        HttpServer::new(|| {
            App::new()
                .wrap(middleware::http::Metrics)
                .service(web::resource("/{any:.*}").to(method::echo))
        })
        .bind(format!("0.0.0.0:{}", config::APP_CONFIG.http.port))
        .unwrap()
        .run()
        .await
        .expect("unexpect exit");
    })
}

pub fn run_metrics() -> JoinHandle<()> {
    tokio::spawn(async {
        HttpServer::new(|| {
            App::new().service(web::resource("/metrics").to(method::metrics_handler))
        })
        .bind(format!("0.0.0.0:{}", config::APP_CONFIG.metrics.port))
        .unwrap()
        .run()
        .await
        .expect("unexpect exit");
    })
}

fn load_rustls_config() -> Result<ServerConfig> {
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .unwrap();

    // init server config builder with safe defaults
    let config = ServerConfig::builder().with_no_client_auth();

    // load TLS key/cert files
    let cert_file = &mut BufReader::new(
        File::open(
            crate::config::APP_CONFIG
                .https
                .certificate_chain_file
                .clone(),
        )
        .unwrap(),
    );
    let key_file = &mut BufReader::new(
        File::open(crate::config::APP_CONFIG.https.private_key_file.clone()).unwrap(),
    );

    // convert files to key/cert objects
    let cert_chain = certs(cert_file).collect::<Result<Vec<_>, _>>()?;
    let mut keys = pkcs8_private_keys(key_file)
        .map(|key| key.map(PrivateKeyDer::Pkcs8))
        .collect::<anyhow::Result<Vec<_>, _>>()
        .unwrap();

    // exit if no keys could be parsed
    if keys.is_empty() {
        eprintln!("Could not locate PKCS 8 private keys.");
        std::process::exit(1);
    }

    Ok(config.with_single_cert(cert_chain, keys.remove(0)).unwrap())
}
