use serde::Deserialize;

#[derive(Deserialize)]
pub struct EchoRequest {
    pub echo_code: Option<u16>,
    pub echo_body: Option<String>,
    pub echo_env_body: Option<String>,
    pub echo_header: Option<String>,
    pub echo_time: Option<String>,
    pub echo_file: Option<String>,
}

impl EchoRequest {
    pub fn only_echo_body(&self) -> bool {
        self.echo_body.is_some()
    }
}