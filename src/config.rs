use std::process::exit;
use serde::Deserialize;
use tracing::error;
use universal_config::ConfigLoader;

pub fn load_config() -> Config {
    let config_loader = ConfigLoader::new("vshell");
    config_loader.find_and_load().unwrap_or_else(|err| {
        error!("Failed to load config: {}", err);
        exit(1);
    })
}

#[derive(Deserialize)]
pub struct Config {
    pub focused_window: FocusedWindow,
    pub ollama: Ollama,
    pub workspaces: Workspaces,
    pub programs: Programs,
    pub music: Music,
    pub wallpapers: Wallpapers,
    pub search: Search,
    pub weather: Weather,
}

#[derive(Deserialize)]
pub struct FocusedWindow {
    pub title_rewrites: Option<Vec<(String, String)>>, // initialClass regex, title replacement
    pub icon_rewrites: Option<Vec<(String, String)>> // initialClass regex, icon replacement
}

#[derive(Deserialize)]
pub struct Workspaces {
    #[serde(default)]
    pub count: u8,
}
impl Default for Workspaces {
    fn default() -> Self {
        Workspaces {
            count: 4,
        }
    }
}

#[derive(Deserialize)]
pub struct Ollama {
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub port: u16,
    #[serde(default)]
    pub default_model: String,
}
impl Default for Ollama {
    fn default() -> Self {
        Ollama {
            url: "http://localhost".to_string(),
            port: 11434,
            default_model: "llama3.3:latest".to_string(),
        }
    }
}

#[derive(Deserialize)]
pub struct Programs {
    pub terminal: String,
}
impl Default for Programs {
    fn default() -> Self {
        Programs {
            terminal: "foot".to_string(),
        }
    }
}

#[derive(Deserialize)]
pub struct Music {
    pub player: String,
}
impl Default for Music {
    fn default() -> Self {
        Music {
            player: "mpv".to_string(),
        }
    }
}

#[derive(Deserialize)]
pub struct Wallpapers {
    pub path: String,
    pub allow_nsfw: bool,
}
impl Default for Wallpapers {
    fn default() -> Self {
        Wallpapers {
            path: "Pictures/Wallpapers".to_string(),
            allow_nsfw: false,
        }
    }
}

#[derive(Deserialize)]
pub struct Search {
    pub engine: String,
}
impl Default for Search {
    fn default() -> Self {
        Search {
            engine: "https://duckduckgo.com/?q=".to_string(),
        }
    }
}

#[derive(Deserialize)]
pub struct Weather {
    pub location: String,
    pub unit: String,
}
impl Default for Weather {
    fn default() -> Self {
        Weather {
            location: "New York".to_string(),
            unit: "celcius".to_string(),
        }
    }
}