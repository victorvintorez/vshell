use clap::Subcommand;
use serde::{Deserialize, Serialize};

#[derive(Subcommand, Debug, Serialize, Deserialize)]
#[serde(tag = "request", rename_all = "snake_case")]
pub enum Request {
    #[command(subcommand)]
    Debug(DebugRequest),
}

#[derive(Subcommand, Debug, Serialize, Deserialize)]
#[serde(tag = "request", rename_all = "snake_case")]
pub enum DebugRequest {
    Ping,
    Inspector,
}
