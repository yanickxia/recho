mod config;
mod host;
mod http;
mod middleware;
mod request;
mod response;

use actix_web::rt::Runtime;
use actix_web::{web, App, HttpServer};

fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let runtime = Runtime::new()?;

    runtime.spawn(async {
        HttpServer::new(|| App::new().service(web::resource("/metrics").to(http::metrics_handler)))
            .bind(format!("0.0.0.0:{}", config::APP_CONFIG.metrics.port))?
            .run()
            .await
    });

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
