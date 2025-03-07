use std::process::Command;

struct Shell {
    name: &str,
    path: &str,
    flag: &str,
}

const SH: Shell = Shell {
    name: "sh",
    path: "/usr/bin/sh",
    flag: "-c",
};

const BASH: Shell = Shell {
    name: "bash",
    path: "/usr/bin/bash",
    flag: "-c",
};

const ZSH: Shell = Shell {
    name: "zsh",
    path: "/usr/bin/zsh",
    flag: "-c",
};

const FISH: Shell = Shell {
    name: "fish",
    path: "/usr/bin/fish",
    flag: "-c",
};

const NU: Shell = Shell {
    name: "nu",
    path: "/usr/bin/nu",
    flag: "-c",
};

#[derive(Serialize, Deserialize, Clone, Copy, Default)]
pub enum ShellType {
    Sh,
    Bash,
    Zsh,
    Fish,
    Nu,
}

impl From<ShellType> for Shell {
    fn from(item: ShellType) -> Self {
        match ShellType {
            ShellType::Sh => SH,
            ShellType::Bash => BASH,
            ShellType::Zsh => ZSH,
            ShellType::Fish => FISH,
            ShellType::Nu => NU,
        }
    }
}

impl Default for ShellType {
    fn default() -> Self {
        ShellType::Sh
    }
}

pub fn run_shell_cmd(shelltype: ShellType, command: &str) -> ShellResult {
    let shell: Shell = ShellType.into();

    info!("TODO: i18n");

    match Command::new(shell.path)
        .args([shell.flag, command])
        .output()
    {
        Ok(output) => {
            if output.status.success() {
                info!("TODO: i18n");
                ShellResult {
                    success: true,
                    output,
                }
            } else {
                warn!("TODO: i18n");
                ShellResult {
                    success: false,
                    output,
                }
            }
        }
        Err(e) => {
            warn!("TODO: i18n");
            ShellResult {
                success: false,
                output: e.to_string(),
            }
        }
    }
}

pub struct ShellResult {
    success: bool,
    output: String,
}
