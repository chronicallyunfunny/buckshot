use crate::constants::CONFIG_PATH;
use serde_derive::Deserialize;
use std::fs::File;
use std::io::Read;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub account: Account,
    pub config: SubConfig,
}

#[derive(Deserialize, Clone)]
pub struct Account {
    pub username: String,
    pub password: String,
    pub sq1: String,
    pub sq2: String,
    pub sq3: String,
}

#[derive(Deserialize, Clone)]
pub struct SubConfig {
    pub auto_offset: bool,
    pub spread: u32,
    pub microsoft_auth: bool,
    pub gc_snipe: bool,
    pub change_skin: bool,
    pub skin_model: String,
    pub skin_filename: String,
}

impl Config {
    // Opens and deserialises config.toml and maps the options to Config struct
    pub fn new() -> Self {
        match File::open(CONFIG_PATH) {
            Ok(mut f) => {
                let mut s = String::new();
                f.read_to_string(&mut s).unwrap();
                let config: Self = toml::from_str(&s).unwrap();
                if !(config.config.skin_model.to_lowercase() == "slim"
                    || config.config.skin_model.to_lowercase() == "classic")
                {
                    panic!("[ConfigParser] Invalid skin type.");
                }
                config
            }
            Err(_) => panic!("File {} not found.", CONFIG_PATH),
        }
    }
}
