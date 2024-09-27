use crate::request::EchoRequest;
use crate::response::{EchoResponse, Host, Http, Request};
use crate::{config, host};
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::rt::time::sleep;
use actix_web::web::{Bytes, Payload, Query};
use actix_web::{HttpRequest, HttpResponse};
use std::collections::HashMap;
use std::time::Duration;
use url::Url;

pub async fn echo(req: HttpRequest, mut body: Payload, param: Query<EchoRequest>) -> HttpResponse {
    let request_body = String::from_utf8(body.to_bytes().await.or::<Bytes>(Ok(Bytes::new())).unwrap().to_vec()).unwrap();

    if param.echo_time.is_some() {
        sleep(Duration::from_millis(param.echo_time.unwrap())).await;
    }

    let response = EchoResponse {
        host: config::APP_CONFIG.enable.host.then_some(Host {
            hostname: host::HOSTNAME.clone(),
            ip: req.peer_addr().unwrap().ip().to_string(),
            ips: exact_xff(&req),
        }),
        http: config::APP_CONFIG.enable.http.then_some(Http {
            method: req.method().to_string(),
            base_url: "".to_string(),
            original_url: req.uri().to_string(),
            protocol: req.connection_info().scheme().to_string(),
        }),
        request: config::APP_CONFIG.enable.request.then_some(Request {
            headers: exact_headers(&req),
            query: exact_params(&req),
            body: request_body.clone(),
        }),
        environment: config::APP_CONFIG.enable.environment.then_some(host::ALL_ENVS.clone()),
    };

    let mut builder = HttpResponse::build(build_status_code(param.echo_code));
    for kv in build_headers(&param) {
        builder.append_header(kv);
    }

    let body = build_body(&param);
    if body.is_some() {
        return builder.content_type(ContentType::plaintext())
            .body(body.unwrap());
    }

    builder.content_type(ContentType::json())
        .json(response)
}


fn build_headers(param: &EchoRequest) -> HashMap<String, String> {
    match param.echo_header.clone() {
        None => {
            Default::default()
        }
        Some(headers) => {
            headers.split(",")
                .map(|it| {
                    let mut spited: Vec<&str> = it.trim().split(":").collect();
                    if spited.len() != 2 {
                        return None;
                    }
                    let key = spited[0];
                    let value = spited[1];
                    Some((key.to_string(), value.to_string()))
                })
                .filter(|it| it.is_some())
                .map(|it| (it.unwrap()))
                .collect()
        }
    }
}

fn build_body(param: &EchoRequest) -> Option<String> {
    if param.echo_body.is_some() {
        return Some(param.echo_body.clone().unwrap_or("".to_string()));
    }
    if param.echo_env_body.is_some() {
        return host::ALL_ENVS.get(param.echo_env_body.as_ref().unwrap()).map(|x| x.to_string());
    }

    None
}

fn build_status_code(code: Option<u16>) -> StatusCode {
    match code {
        None => {
            StatusCode::OK
        }
        Some(code) => {
            match StatusCode::from_u16(code) {
                Ok(res) => {
                    res
                }
                Err(_) => {
                    StatusCode::OK
                }
            }
        }
    }
}


fn exact_params(req: &HttpRequest) -> HashMap<String, Vec<String>> {
    let url = Url::parse(req.full_url().as_str()).unwrap();
    url.query_pairs().fold(HashMap::<String, Vec<String>>::new(), |mut acc, (k, v)| {
        acc.entry(k.to_string())
            .and_modify(|e| e.push(v.to_string()))
            .or_insert(vec![v.to_string()]);
        acc
    })
}

fn exact_headers(req: &HttpRequest) -> HashMap<String, String> {
    let mut all_headers = HashMap::<String, String>::new();

    for (k, v) in req.headers() {
        all_headers.entry(k.to_string())
            .and_modify(|e| e.push_str(&format!(", {}", v.to_str().unwrap())))
            .or_insert(v.to_str().unwrap().to_string());
    }

    all_headers
}

fn exact_xff(req: &HttpRequest) -> Vec<String> {
    match req.headers().get("X-Forwarded-For") {
        None => {
            vec![]
        }
        Some(val) => {
            match val.to_str() {
                Ok(val) => {
                    val.split(",")
                        .map(|x| x.trim().to_string())
                        .into_iter()
                        .collect::<Vec<String>>()
                }
                Err(_) => {
                    vec![]
                }
            }
        }
    }
}
