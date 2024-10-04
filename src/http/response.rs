use std::collections::HashMap;

use serde::Serialize;

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
    pub headers: Option<HashMap<String, String>>,
    pub query: HashMap<String, Vec<String>>,
    pub body: String,
}

#[derive(Serialize)]
pub struct EchoResponse {
    pub host: Option<Host>,
    pub http: Option<Http>,
    pub request: Option<Request>,
    pub environment: Option<HashMap<String, String>>,
}
