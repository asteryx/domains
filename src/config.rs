use serde_derive::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use toml;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub database_url: String,
    pub log_level: String,
    pub ping_interval: u64, //in seconds
}

impl Default for Config {
    fn default() -> Config {
        Config {
            database_url: "db.sqlite".to_string(),
            log_level: "ERROR".to_string(),
            ping_interval: 60,
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
