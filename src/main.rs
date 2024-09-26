mod http;
mod response;
mod request;
mod host;
mod config;

use actix_web::rt::Runtime;
use actix_web::{web, App, HttpServer};


fn main() -> std::io::Result<()> {
    let config = config::load_config();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let runtime = Runtime::new()?;
    runtime.block_on(async {
        HttpServer::new(||
            App::new().service(web::resource("/{any:.*}").to(http::echo)))
            .bind(format!("127.0.0.1:{}", config.http.port))?
            .run()
            .await
    })
}
