use std::time::Duration;

use tonic::{Request, Response, Status};

use echo::{EchoReply, EchoRequest};

use crate::grpc::method::echo::echo_server::Echo;

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

        let reply = EchoReply {
            message: request.get_ref().message.clone()
        };
        Ok(Response::new(reply))
    }
}
