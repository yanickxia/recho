use serde::Deserialize;
use validator::Validate;


#[derive(Deserialize, Validate)]
pub struct EchoRequest {
    pub echo_code: Option<u16>,
    pub echo_body: Option<String>,
    pub echo_env_body: Option<String>,
    pub echo_header: Option<String>,
    #[validate(range(min = 1, max = 30000))]
    pub echo_time: Option<u64>,
    pub echo_file: Option<String>,
}
