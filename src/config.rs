use serde_derive::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use toml;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
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
    pub fn database_url(&self) -> &String {
        &self.database_url
    }
    pub fn log_level(&self) -> &String {
        &self.log_level
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
}

impl Default for Config {
    fn default() -> Config {
        Config {
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
                f.write(tml.as_bytes());
                f.sync_all();
                tml
            }
        }
    }
}
