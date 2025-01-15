use std::path::PathBuf;
use clap::{Parser, Subcommand};
use serde;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug, Serialize, Deserialize)]
#[command(version)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,

    #[arg(short, long, value_name = "FILE")]
    pub config_path: Option<PathBuf>,
    
    #[arg(short, long, value_name = "FILE")]
    pub style_path: Option<PathBuf>,

    #[arg(short, long, action = clap::ArgAction::Count)]
    pub debug: u8,
}

#[derive(Subcommand, Debug, Serialize, Deserialize)]
#[serde(tag = "command", rename_all = "snake_case")]
pub enum Commands {
    Init,
}