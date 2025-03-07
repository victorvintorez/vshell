use std::process::Command;

use serde::{Deserialize, Serialize};
use tracing::{info, warn};

struct Shell {
    path: &'static str,
    flag: &'static str,
}

const SH: Shell = Shell {
    path: "/usr/bin/sh",
    flag: "-c",
};

const BASH: Shell = Shell {
    path: "/usr/bin/bash",
    flag: "-c",
};

const ZSH: Shell = Shell {
    path: "/usr/bin/zsh",
    flag: "-c",
};

const FISH: Shell = Shell {
    path: "/usr/bin/fish",
    flag: "-c",
};

const NU: Shell = Shell {
    path: "/usr/bin/nu",
    flag: "-c",
};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default)]
pub enum ShellType {
    #[default]
    Sh,
    Bash,
    Zsh,
    Fish,
    Nu,
}

impl From<ShellType> for Shell {
    fn from(item: ShellType) -> Self {
        match item {
            ShellType::Sh => SH,
            ShellType::Bash => BASH,
            ShellType::Zsh => ZSH,
            ShellType::Fish => FISH,
            ShellType::Nu => NU,
        }
    }
}

pub fn run_shell_cmd(shelltype: &ShellType, command: &str) -> ShellResult {
    let shell: Shell = shelltype.clone().into();

    info!("TODO: i18n");

    match Command::new(shell.path)
        .args([shell.flag, command])
        .output()
    {
        Ok(output) => {
            if output.status.success() {
                info!("TODO: i18n");
                ShellResult {
                    command: command.to_string(),
                    success: true,
                    output: String::from_utf8(output.stdout).expect("TODO: i18n"),
                }
            } else {
                warn!("TODO: i18n");
                ShellResult {
                    command: command.to_string(),
                    success: false,
                    output: String::from_utf8(output.stdout).expect("TODO: i18n"),
                }
            }
        }
        Err(e) => {
            warn!("TODO: i18n");
            ShellResult {
                command: command.to_string(),
                success: false,
                output: e.to_string(),
            }
        }
    }
}

pub struct ShellResult {
    pub command: String,
    pub success: bool,
    pub output: String,
}
