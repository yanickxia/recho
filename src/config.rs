use config::Config;
use lazy_static::lazy_static;
use serde::Deserialize;


lazy_static! {
   pub static ref APP_CONFIG: EchoConfig = load_config();
}

#[derive(Deserialize)]
pub struct EchoConfig {
    pub http: Http,
    pub enable: Enable,
}

#[derive(Deserialize)]
pub struct Http {
    pub port: u32,
}

#[derive(Deserialize, Default)]
pub struct Enable {
    pub host: bool,
    pub http: bool,
    pub request: bool,
    pub header: bool,
    pub environment: bool,
    pub file: bool,
}

pub fn load_config() -> EchoConfig {
    let settings = Config::builder()
        .add_source(config::File::with_name("config/settings"))
        .add_source(config::Environment::default().separator("_"))
        .build()
        .unwrap();
    settings.try_deserialize::<EchoConfig>().unwrap()
}
