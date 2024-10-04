use std::sync::Arc;
use std::time::Duration;

use tonic::{Request, Response, Status};

use echo::{EchoReply, EchoRequest};

use crate::config;
use crate::contants::*;
use crate::grpc::method::echo::echo_server::Echo;
use crate::grpc::method::echo::{Grpc, Host};
use crate::host;

pub mod echo {
    tonic::include_proto!("echo");
}

#[derive(Debug, Default)]
pub struct EchoServer {}

#[tonic::async_trait]
impl Echo for EchoServer {
    async fn echo(&self, request: Request<EchoRequest>) -> Result<Response<EchoReply>, Status> {
        let delay = request.get_ref().delay;
        if delay > 0 {
            tokio::time::sleep(Duration::from_millis(delay)).await;
        }

        let protocol = match request
            .extensions()
            .get::<Arc<crate::grpc::server::ConnInfo>>()
        {
            None => PLAINTEXT,
            Some(_) => TLS,
        };

        let method = request.extensions().get::<Arc<String>>().unwrap();

        let reply = EchoReply {
            message: request.get_ref().message.clone(),
            environment: config::APP_CONFIG
                .enable
                .environment
                .then_some(host::ALL_ENVS.clone())
                .unwrap_or_default(),
            grpc: Some(Grpc {
                protocol: protocol.to_string(),
                method: method.as_str().to_string(),
            }),
            host: config::APP_CONFIG.enable.host.then_some(Host {
                hostname: host::hostname(),
                ip: request.remote_addr().map(|x| x.ip().to_string()),
            }),
        };
        Ok(Response::new(reply))
    }
}
