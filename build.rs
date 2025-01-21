#![allow(unused, dead_code)]

#[path = "src/architecture"]
pub mod architecture {
    #[path = "cli/mod.rs"]
    pub mod cli;
    
    pub use cli::Args;
    
    #[path = "ipc"]
    pub mod ipc {
        #[path = "request.rs"]
        pub mod request;
        
        #[path = "response.rs"]
        pub mod response;
        
        pub use request::Request;
        pub use response::Response;
    }
}

use std::fs;
use std::path::PathBuf;
use clap::{Command, CommandFactory};
use clap_complete::generate_to;
use clap_complete::Shell::{Bash, Fish, Zsh};
use crate::architecture::Args;
use crate::architecture::ipc::{Request, Response};

pub fn generate_shell_completions(mut cmd: Command) -> std::io::Result<()> {
    let completion_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/completions");

    fs::create_dir_all(&completion_dir)?;

    for shell in [Bash, Fish, Zsh] {
        generate_to(shell, &mut cmd, "vshell", &completion_dir)?;
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let mut cmd = Args::command();
    cmd.set_bin_name("vshell");
    
    generate_shell_completions(cmd)?;

    Ok(())
}