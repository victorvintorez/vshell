use clap::Subcommand;
use serde::{Deserialize, Serialize};

#[derive(Subcommand, Debug, Serialize, Deserialize)]
#[serde(tag = "request", rename_all = "snake_case")]
pub enum Request {
    #[command(subcommand)]
    Debug(DebugRequest),
    #[command(subcommand)]
    Wallpaper(WallpaperRequest),
}

#[derive(Subcommand, Debug, Serialize, Deserialize)]
#[serde(tag = "request", rename_all = "snake_case")]
pub enum DebugRequest {
    Ping,
    Inspector,
}

#[derive(Subcommand, Debug, Serialize, Deserialize)]
#[serde(tag = "request", rename_all = "snake_case")]
pub enum WallpaperRequest {
    Set { path: String },
    Default,
    Show,
}
