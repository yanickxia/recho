mod http;
mod response;
mod request;
mod host;
mod config;

use actix_web::rt::Runtime;
use actix_web::{web, App, HttpServer};


fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let runtime = Runtime::new()?;
    runtime.block_on(async {
        HttpServer::new(||
            App::new().service(web::resource("/{any:.*}").to(http::echo)))
            .bind(format!("0.0.0.0:{}", config::APP_CONFIG.http.port))?
            .run()
            .await
    })
}
