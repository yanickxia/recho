use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct Host {
    pub hostname: Option<String>,
    pub ip: String,
    pub ips: Vec<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Http {
    pub method: String,
    pub base_url: String,
    pub original_url: String,
    pub protocol: String,
}

#[derive(Serialize)]
pub struct Request {
    pub headers: HashMap<String, String>,
    pub query: HashMap<String, Vec<String>>,
    pub body: String,
}

#[derive(Serialize)]
pub struct EchoResponse {
    pub host: Host,
    pub http: Http,
    pub request: Request,
    pub environment: HashMap<String, String>,
}
