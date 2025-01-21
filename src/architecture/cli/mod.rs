use crate::architecture::ipc::{request::Request, response::Response};
use clap::Parser;
use serde;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tracing::{error, info};

#[derive(Parser, Debug, Serialize, Deserialize)]
#[command(version)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Request>,

    #[arg(short, long, value_name = "FILE")]
    pub config_path: Option<PathBuf>,

    #[arg(short, long, value_name = "FILE")]
    pub style_path: Option<PathBuf>,

    #[arg(short, long)]
    pub debug: bool,
}

pub fn handle_response(response: Response) {
    let is_err = matches!(response, Response::Error { .. });

    match response {
        Response::Ok { message } => {
            if let Some(message) = message {
                info!("{}", message);
            }
        }
        Response::Error { message } => {
            if let Some(message) = message {
                error!("{}", message);
            }
        }
    }

    if is_err {
        std::process::exit(1);
    }
}
