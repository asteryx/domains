use serde_derive::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use toml;

const ENV_PREFIX: &str = "DOMAINS";
lazy_static! {
    pub static ref CONFIG: Config = Config::from_file();
}
fn get_env_name(local_env: &str) -> String {
    format!("{}_{}", ENV_PREFIX, local_env).to_ascii_uppercase()
}

pub fn get_env(name: &str, default: &str) -> String {
    let env_var = std::env::var(&get_env_name(name));
    match env_var {
        Ok(var) => var,
        _ => default.to_string(),
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    server_addr: String,
    server_port: String,
    database_url: String,
    log_level: String,
    ping_interval: u64, //in seconds
    media_root: String,
    image_format: String,
    domain_statuses_rotation: bool,
    domain_statuses_rotate_days: i32,
    jwt_expiration_hours: u16,
    jwt_secret_key: String,
}

impl Config {
    pub fn server_addr(&self) -> String {
        get_env("server_addr", &self.server_addr)
    }
    pub fn server_port(&self) -> String {
        get_env("server_port", &self.server_port)
    }
    pub fn database_url(&self) -> String {
        get_env("database_url", &self.database_url)
    }
    pub fn log_level(&self) -> String {
        get_env("log_level", &self.log_level)
    }
    pub fn log_level_env_name(&self) -> String {
        get_env_name("log_level")
    }
    pub fn ping_interval(&self) -> u64 {
        self.ping_interval
    }
    pub fn media_root(&self) -> &String {
        &self.media_root
    }
    pub fn image_format(&self) -> &String {
        &self.image_format
    }
    pub fn domain_statuses_rotation(&self) -> bool {
        self.domain_statuses_rotation
    }
    pub fn domain_statuses_rotate_days(&self) -> i32 {
        self.domain_statuses_rotate_days
    }
    pub fn jwt_expiration_hours(&self) -> u16 {
        self.jwt_expiration_hours
    }
    pub fn jwt_secret_key(&self) -> &String {
        &self.jwt_secret_key
    }
    pub fn media(&self, filename: Option<String>) -> Option<String> {
        if let Some(file) = filename {
            Some(format!("{}{}", self.media_root(), file))
        } else {
            None
        }
    }
}

impl Default for Config {
    fn default() -> Config {
        Config {
            server_addr: "127.0.0.1".to_string(),
            server_port: "8000".to_string(),
            database_url: "postgres://domains:defaultpassword@localhost:5432/domains".to_string(),
            log_level: "ERROR".to_string(),
            ping_interval: 60,
            media_root: "./media/".to_string(),
            image_format: "png".to_string(),
            domain_statuses_rotation: true,
            domain_statuses_rotate_days: 2,
            jwt_expiration_hours: 300,
            jwt_secret_key: "Super test secret key".to_string(),
        }
    }
}

impl Config {
    pub fn new() -> Config {
        Config::default()
    }
    pub fn from_file() -> Config {
        let config_toml = Config::open_config_file("config.toml".to_string());

        toml::from_str(&config_toml).expect("Config file parsing error")
    }

    fn open_config_file(path: String) -> String {
        match File::open(&path) {
            Ok(mut file) => {
                let mut res = String::new();
                file.read_to_string(&mut res)
                    .expect("Error read file 'config.toml'");
                res
            }
            Err(_) => {
                let mut f: File =
                    File::create(&path).expect(&"Cannot create config file 'config.toml'");
                let config = Config::default();
                let tml = toml::to_string(&config).unwrap();
                f.write(tml.as_bytes()).ok();
                f.sync_all().ok();
                tml
            }
        }
    }
}
