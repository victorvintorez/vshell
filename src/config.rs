use std::env;
use std::path::PathBuf;
use color_eyre::Report;
use serde::Deserialize;
use tracing::{debug, error, warn};
use universal_config::ConfigLoader;

pub fn load_config(config_dir: Option<PathBuf>) -> (Config, PathBuf) {
    let (config, dir) = match config_dir {
        Some(conf_dir) => {
            (ConfigLoader::load(&conf_dir), conf_dir.parent().map(PathBuf::from).ok_or_else(|| Report::msg("Failed to get parent directory of config directory")))
        }
        None => {
            let config_loader = ConfigLoader::new("vshell");
            (
                config_loader.find_and_load(),
                config_loader.config_dir().map_err(Report::new),
            )
        }
    };

    let config = config.unwrap_or_else(|err| {
        error!("Failed to load config: {}", err);
        warn!("Using default config");
        
        Config::default()
    });
    
    let dir = dir.and_then(|dir| dir.canonicalize().map_err(Report::new)).unwrap_or_else(|_| env::current_dir().expect("to have current directory"));
    
    debug!("Using config.toml from: {}/", dir.display());
    
    (config, dir)
}

#[derive(Deserialize, Default, Debug)]
pub struct Config {
    #[serde(default)]
    pub monitor: String,
}