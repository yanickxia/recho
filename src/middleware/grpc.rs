use std::sync::Arc;
use std::{
    pin::Pin,
    task::{Context, Poll},
};

use lazy_static::lazy_static;
use prometheus_client::encoding::EncodeLabelSet;
use prometheus_client::metrics::counter::Counter;
use prometheus_client::metrics::family::Family;
use tonic::body::BoxBody;
use tower::{Layer, Service};

use crate::contants::*;

#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub struct RequestLabels {
    method: String,
    protocol: String,
}

lazy_static! {
    pub static ref GRPC_REQUESTS_COUNTER: Family::<RequestLabels, Counter> =
        Family::<RequestLabels, Counter>::default();
}

#[derive(Debug, Clone, Default)]
pub(crate) struct MetricsMiddlewareLayer {}

impl<S> Layer<S> for MetricsMiddlewareLayer {
    type Service = MetricsMiddleware<S>;

    fn layer(&self, service: S) -> Self::Service {
        MetricsMiddleware { inner: service }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct MetricsMiddleware<S> {
    inner: S,
}

type BoxFuture<'a, T> = Pin<Box<dyn std::future::Future<Output = T> + Send + 'a>>;

impl<S> Service<hyper::Request<BoxBody>> for MetricsMiddleware<S>
where
    S: Service<hyper::Request<BoxBody>, Response = hyper::Response<BoxBody>>
        + Clone
        + Send
        + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: hyper::Request<BoxBody>) -> Self::Future {
        // This is necessary because tonic internally uses `tower::buffer::Buffer`.
        // See https://github.com/tower-rs/tower/issues/547#issuecomment-767629149
        // for details on why this is necessary
        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);

        // Do extra async work here...
        let protocol = match req.uri().scheme() {
            None => PLAINTEXT,
            Some(schema) => {
                let schema = schema.to_string().to_lowercase();
                if schema == HTTPS {
                    TLS
                } else {
                    PLAINTEXT
                }
            }
        };

        let method = req.uri().path().to_string();

        let labels = RequestLabels {
            method: method.clone(),
            protocol: protocol.to_string(),
        };
        GRPC_REQUESTS_COUNTER.get_or_create(&labels).inc();

        req.extensions_mut().insert(Arc::new(method));

        Box::pin(async move {
            let response = inner.call(req).await?;
            Ok(response)
        })
    }
}

// static PROTOCOL: &str = "protocol";
//
// pub fn protocol_append(mut req: Request<()>) -> Result<Request<()>, Status> {
//     req.metadata_mut().insert(PROTOCOL, "TLS".parse().unwrap())
//     Ok(req)
// }
//
// pub fn intercept(mut req: Request<()>) -> Result<Request<()>, Status> {
//     let protocol = req.metadata().get(PROTOCOL)
//         .unwrap_or("plaintext".parse().unwrap());
//
//     let labels = RequestLabels {
//         method: "".to_string(),
//         protocol: protocol.to_string(),
//     };
//
//     GRPC_REQUESTS_COUNTER.get_or_create(&labels).inc();
//     Ok(req)
// }
