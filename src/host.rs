use std::collections::HashMap;
use std::env;
use lazy_static::lazy_static;


lazy_static! {
   pub static ref ALL_ENVS: HashMap<String, String> = all_envs();
   pub static ref HOSTNAME: Option<String> = hostname();
}

pub fn hostname() -> Option<String> {
    env::var("HOSTNAME").ok()
}

pub fn all_envs() -> HashMap<String, String> {
    env::vars().collect()
}