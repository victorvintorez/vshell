use crate::fl;
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
    let shell: Shell = (*shelltype).into();

    info!("{}", fl!("shell_info_running-command", cmd = command));

    match Command::new(shell.path)
        .args([shell.flag, command])
        .output()
    {
        Ok(output) => {
            if output.status.success() {
                let res = String::from_utf8(output.stdout)
                    .expect(&fl!("shell_expect_command-success-result"));
                info!(
                    "{}",
                    fl!("shell_info_command-success", cmd = command, result = *res)
                );
                ShellResult {
                    command: command.to_string(),
                    success: true,
                    output: res,
                }
            } else {
                let res = String::from_utf8(output.stdout)
                    .expect(&fl!("shell_expect_command-fail-result"));
                warn!(
                    "{}",
                    fl!("shell_warn_command-fail", cmd = command, result = *res)
                );
                ShellResult {
                    command: command.to_string(),
                    success: false,
                    output: res,
                }
            }
        }
        Err(e) => {
            warn!(
                "{}",
                fl!(
                    "shell_warn_command-not-run",
                    cmd = command,
                    error = format!("{:?}", e)
                )
            );
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
