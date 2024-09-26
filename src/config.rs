use config::Config;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct EchoConfig {
    pub http: Http,
}

#[derive(Deserialize)]
pub struct Http {
    pub port: u32,
}

pub fn load_config() -> EchoConfig {
    let settings = Config::builder()
        .add_source(config::File::with_name("config/settings"))
        .add_source(config::Environment::default().separator("_"))
        .build()
        .unwrap();
    settings.try_deserialize::<EchoConfig>().unwrap()
}