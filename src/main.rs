mod config;
mod contants;
mod grpc;
mod host;
mod http;
mod middleware;

#[tokio::main]
async fn main() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let mut handles = vec![];
    handles.push(http::server::run_http());
    handles.push(http::server::run_metrics());

    if config::APP_CONFIG.enable.https {
        handles.push(http::server::run_https());
    }

    if config::APP_CONFIG.enable.grpc {
        grpc::server::run_grpc();
    }

    if config::APP_CONFIG.enable.grpc_tls {
        grpc::server::run_grpc_tls();
    }
    futures::future::join_all(handles).await;
}
