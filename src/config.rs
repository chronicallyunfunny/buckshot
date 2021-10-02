use anyhow::{bail, Result};
use serde::Deserialize;
use std::fs::read_to_string;
use std::path::Path;

#[derive(Deserialize)]
pub struct Config {
    pub account_entry: Vec<Account>,
    pub offset: Option<i64>,
    pub spread: usize,
    pub microsoft_auth: bool,
    pub gc_snipe: bool,
    pub skin: Option<Skin>,
    pub name_queue: Option<Vec<String>>,
}

#[derive(Deserialize)]
pub struct Skin {
    pub is_file: bool,
    pub path: String,
    pub slim: bool,
}

#[derive(Clone, Deserialize)]
pub struct Account {
    pub email: String,
    pub password: String,
    pub sq_ans: Option<[String; 3]>,
    pub giftcode: Option<String>,
}

impl Config {
    pub fn new(config_path: &Path) -> Result<Self> {
        let cfg_str = read_to_string(&config_path)?;
        let cfg: Self = toml::from_str(&cfg_str)?;
        if cfg.account_entry.is_empty() {
            bail!("No accounts provided in config file");
        }
        if cfg.account_entry.len() > 10 {
            bail!("Only a max of 10 accounts is allowed when GC sniping");
        }
        if let Some(count) = &cfg.name_queue {
            if count.is_empty() {
                bail!("No name provided in name queue");
            }
        }
        Ok(cfg)
    }
}
