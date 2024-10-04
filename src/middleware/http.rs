use std::future::{ready, Ready};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures_util::future::LocalBoxFuture;
use lazy_static::lazy_static;
use prometheus_client::encoding::EncodeLabelSet;
use prometheus_client::metrics::counter::Counter;
use prometheus_client::metrics::family::Family;
use prometheus_client::registry::Registry;

use crate::middleware::grpc;

#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub struct RequestLabels {
    method: String,
    path: String,
    protocol: String,
    status_code: u32,
}

lazy_static! {
    pub static ref HTTP_REQUESTS_COUNTER: Family::<RequestLabels, Counter> =
        Family::<RequestLabels, Counter>::default();
    pub static ref REGISTRY: Registry = {
        let mut r = <Registry>::default();

        r.register(
            "http_requests",
            "Number of HTTP requests received",
            HTTP_REQUESTS_COUNTER.clone(),
        );

        r.register(
            "grpc_requests",
            "Number of GRPC requests received",
            grpc::GRPC_REQUESTS_COUNTER.clone(),
        );

        r
    };
}

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct Metrics;

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for Metrics
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = MetricsMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(MetricsMiddleware { service }))
    }
}

pub struct MetricsMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for MetricsMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let method = req.method().to_string();
        let path = req.path().to_string();
        let protocol = req.connection_info().scheme().to_string();

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            let labels = RequestLabels {
                method,
                path,
                protocol,
                status_code: res.status().as_u16() as u32,
            };
            HTTP_REQUESTS_COUNTER.get_or_create(&labels).inc();
            Ok(res)
        })
    }
}
