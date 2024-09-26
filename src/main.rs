use actix_web::rt::Runtime;
use actix_web::{web, App, HttpServer};

mod http;
mod response;
mod request;
mod host;

fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let runtime = Runtime::new()?;
    runtime.block_on(async {
        HttpServer::new(||
            App::new().service(web::resource("/{any:.*}").to(http::echo)))
            .bind("127.0.0.1:8080")?
            .run()
            .await
    })
}
