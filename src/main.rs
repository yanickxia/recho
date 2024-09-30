mod config;
mod host;
mod http;
mod middleware;
mod request;
mod response;

use actix_web::rt::Runtime;
use actix_web::{web, App, HttpServer};
use anyhow::Result;
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};

fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let runtime = Runtime::new()?;

    runtime.spawn(async {
        HttpServer::new(|| App::new().service(web::resource("/metrics").to(http::metrics_handler)))
            .bind(format!("0.0.0.0:{}", config::APP_CONFIG.metrics.port))?
            .run()
            .await
    });

    if config::APP_CONFIG.enable.https {
        match init_ssl() {
            Ok(builder) => {
                runtime.spawn(async {
                    HttpServer::new(|| {
                        App::new()
                            .wrap(middleware::Metrics)
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
                .wrap(middleware::Metrics)
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
