use crate::fl;
use color_eyre::Report;
use serde::Deserialize;
use std::path::PathBuf;
use std::{collections::HashMap, env};
use tracing::{debug, error, warn};
use universal_config::ConfigLoader;

pub fn load_config(config_dir: Option<PathBuf>) -> (Config, PathBuf) {
    let (config, dir) = match config_dir {
        Some(conf_dir) => (
            ConfigLoader::load(&conf_dir),
            conf_dir
                .parent()
                .map(PathBuf::from)
                .ok_or_else(|| Report::msg(fl!("config_error_config-parent-dir"))),
        ),
        None => {
            let config_loader = ConfigLoader::new("vshell");
            (
                config_loader.find_and_load(),
                config_loader.config_dir().map_err(Report::new),
            )
        }
    };

    let config = config.unwrap_or_else(|err| {
        error!(
            "{}",
            fl!(
                "config_error_config-file-fail",
                error = format!("{:?}", err)
            )
        );
        warn!("{}", fl!("config_warn_using-default-config"));

        Config::default()
    });

    let dir = dir
        .and_then(|dir| dir.canonicalize().map_err(Report::new))
        .unwrap_or_else(|_| env::current_dir().expect("to have current directory"));

    debug!(
        "{}",
        fl!(
            "config_debug_config-file-load",
            path = dir.clone().into_os_string().into_string().unwrap()
        )
    );

    (config, dir)
}

#[derive(Deserialize, Default, Debug, Clone)]
pub struct Config {
    #[serde(default)]
    pub monitor: String,
    pub templates: Option<HashMap<String, TemplateConfig>>,
}

#[derive(Deserialize, Default, Debug, Clone)]
pub struct TemplateConfig {
    #[serde(alias = "tmpl", alias = "from")]
    pub template: String,
    #[serde(alias = "dest", alias = "to")]
    pub target: String,
    #[serde(alias = "pre_cmd")]
    pub pre: Option<String>,
    #[serde(alias = "post_cmd", alias = "reload")]
    pub post: Option<String>,
}
