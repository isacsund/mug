// std imports {{{
use std::fs;
use std::io::ErrorKind::NotFound;
// }}}

// 3rd party imports {{{
use serde_derive::{
    Deserialize,
    Serialize,
};
use url::Url;
// }}}

// Own imports {{{
use crate::error::Error;
// }}}

/// The default URL used for the AUR.
const AUR_URL: &str = "https://aur.archlinux.org/";

/// Configuration for mug.
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
pub struct Config {
    pub aur_url: Url,
}

impl Default for Config {
    fn default() -> Self {
        let aur_url = Url::parse(AUR_URL).expect("Failed to parse URL");

        Config {
            aur_url,
        }
    }
}

impl Config {
    pub fn load() -> Result<Self, Error> {
        let config_dir = match dirs::config_dir(){
            Some(d) => d,
            None => return Err(Error::Config("Failed to get config directory".to_string())),
        };

        let config_dir = config_dir.join(env!("CARGO_PKG_NAME"));
        let config_file = config_dir.join("config.toml");

        let config = match fs::read_to_string(&config_file) {
            Ok(c) => {
                toml::from_str(&c).expect("Failed to parse toml in configuration file")
            }

            Err(ref e) if e.kind() == NotFound => {
                let config = Config::default();

                let toml = toml::to_string_pretty(&config).expect("Failed to convert config to toml");

                fs::create_dir_all(config_dir).expect("Failed to create config directory");
                fs::write(&config_file, toml).expect("Failed to write config file");

                config
            }

            Err(_) => {
                return Err(Error::Config("Failed to load configuration file".to_string()))
            }
        };

        Ok(config)
    }
}
