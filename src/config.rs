use crate::cli::pretty_panic;
use crate::constants::CONFIG_PATH;
use serde::Deserialize;
use std::io::ErrorKind::NotFound;
use std::path::Path;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Deserialize)]
pub struct Config {
    pub account: Account,
    pub config: SubConfig,
}

#[derive(Deserialize)]
pub struct Account {
    pub username: String,
    pub password: String,
    pub sq1: String,
    pub sq2: String,
    pub sq3: String,
}

#[derive(Deserialize)]
pub struct SubConfig {
    pub offset: i32,
    pub auto_offset: bool,
    pub spread: u32,
    pub microsoft_auth: bool,
    pub gc_snipe: bool,
    pub change_skin: bool,
    pub skin_model: String,
    pub skin_filename: String,
    pub name_queue: Vec<String>,
}

impl Config {
    pub async fn new(config_name: &Option<String>) -> Self {
        let config_path = match config_name {
            Some(x) => x,
            None => CONFIG_PATH,
        };
        match File::open(&config_path).await {
            Ok(mut f) => {
                let mut s = String::new();
                f.read_to_string(&mut s).await.unwrap();
                let config: Result<Self, _> = toml::from_str(&s);
                let config = match config {
                    Ok(c) => c,
                    Err(_) => pretty_panic(&format!(
                        "Error parsing {}, please check the formatting of the file.",
                        config_path
                    )),
                };
                if !(config.config.skin_model.to_lowercase() == "slim"
                    || config.config.skin_model.to_lowercase() == "classic")
                {
                    pretty_panic("Invalid skin type.");
                }
                config
            }
            Err(e) if e.kind() == NotFound => {
                let path = Path::new(config_path);
                let mut file = File::create(path).await.unwrap();
                file.write_all(&get_default_config().into_bytes())
                    .await
                    .unwrap();
                pretty_panic(&format!(
                    "File {} not found, creating a new config file. Please enter any relevant information to the file.",
                    config_path
                ));
            }
            Err(e) => pretty_panic(&format!(
                "File {} not found, a new config file cannot be created. Reason: {}.",
                config_path, e
            )),
        }
    }
}

fn get_default_config() -> String {
    [
        r#"# Leave the account section empty if you are using a Microsoft account"#,
        r#"[account]"#,
        r#"username = "test@example.com""#,
        r#"password = "test""#,
        r#"# Leave the rest empty if you do not have security questions"#,
        r#"sq1 = "Foo""#,
        r#"sq2 = "Bar""#,
        r#"sq3 = "Baz""#,
        r#""#,
        r#"[config]"#,
        r#"offset = 0"#,
        r#"auto_offset = false"#,
        r#"spread = 0"#,
        r#"microsoft_auth = false"#,
        r#"gc_snipe = false"#,
        r#"change_skin = false"#,
        r#"skin_model = "slim""#,
        r#"skin_filename = "example.png""#,
        r#"# Name queueing example:"#,
        r#"# name_queue = ["Marc", "Dream"]"#,
        r#"name_queue = []"#,
        r#""#,
    ]
    .join("\r\n")
}
