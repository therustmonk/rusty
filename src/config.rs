use std::path::Path;
use std::io::{Read, Write};
use std::fs::File;
use failure::Error;
use toml;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub message: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            message: "❤️ Rust".into(),
        }
    }
}

impl Config {
    pub fn write(&self, path: &Path) -> Result<(), Error> {
        let toml = toml::to_string(self)?;
        let mut file = File::create(path)?;
        file.write_all(toml.as_ref())?;
        Ok(())
    }

    pub fn read_or_create(path: &Path) -> Result<Self, Error> {
        if !path.exists() {
            let config = Config::default();
            config.write(path)?;
            Ok(config)
        } else {
            let mut file = File::open(path)?;
            let mut buffer = String::new();
            file.read_to_string(&mut buffer)?;
            let config: Config = toml::from_str(&buffer)?;
            Ok(config)
        }
    }
}
