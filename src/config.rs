use serde_derive::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use toml;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub database_url: String,
    pub log_level: String,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            database_url: "db.sqlite".to_string(),
            log_level: "ERROR".to_string(),
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

// Attempt to load and parse the config file into our Config struct.
// If a file cannot be found, return a default Config.
// If we find a file but cannot parse it, panic
//pub fn parse(path: String) -> Config {
//    let mut config_toml = String::new();
//
//    file.read_to_string(&mut config_toml)
//        .unwrap_or_else(|err| panic!("Error while reading config: [{}]", err));
//
//    let mut parser = Parser::new(&config_toml);
//    let toml = parser.parse();
//
//    if toml.is_none() {
//        for err in &parser.errors {
//            let (loline, locol) = parser.to_linecol(err.lo);
//            let (hiline, hicol) = parser.to_linecol(err.hi);
//            println!(
//                "{}:{}:{}-{}:{} error: {}",
//                path, loline, locol, hiline, hicol, err.desc
//            );
//        }
//        panic!("Exiting server");
//    }
//
//    let config = Value::Table(toml.unwrap());
//    match toml::decode(config) {
//        Some(t) => t,
//        None => panic!("Error while deserializing config"),
//    }
//}
