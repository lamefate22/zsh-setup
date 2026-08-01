use crate::errors::{AppError, FileSystem, System};
use std::process::{Command, Stdio};
use which::which;
use std::env;
use std::fs;
use std::path::PathBuf;

fn resolve_zsh_custom() -> PathBuf {
    let home = env::var("HOME").unwrap_or_else(|_| String::from("~"));
    let custom = env::var("ZSH_CUSTOM").unwrap_or_else(|_| format!("{}/.oh-my-zsh/custom", home));
    PathBuf::from(custom)
}

pub struct Platform {
    os: String
}

impl Platform {
    pub fn new() -> Self {
        Self {
            os: env::consts::OS.to_string()
        }
    }

    pub fn is_dependencies_installed(&self) -> bool {
        let curl = self.is_command_available("curl");
        let zsh = self.is_command_available("zsh");
        let git = self.is_command_available("git");

        curl && zsh && git
    }

    pub fn install_ohmyzsh(&self) -> Result<(), AppError> {
        self.run_command("sh",
            &[
                "-c",
                "curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh | sh -s -- --unattended",
            ]
        )?;
        Ok(())
    }

    pub fn install_theme(&self) -> Result<(), AppError> {
        let custom = resolve_zsh_custom();
        let theme_path = custom.join("themes/powerlevel10k");
        self.run_command("git",
            &["clone", "--depth=1", "https://github.com/romkatv/powerlevel10k.git", theme_path.to_str().unwrap()]
        )?;

        if self.os == "android" {
            tracing::info!("Fonts will be installed automatically.");
        } else {
            tracing::info!("You must install fonts from the repository before using a new theme!");
            tracing::info!("Instructions - https://github.com/romkatv/powerlevel10k#meslo-nerd-font-patched-for-powerlevel10k");
        }

        Ok(())
    }

    pub fn install_plugin(&self, repo: &'static str, relative_path: &'static str) -> Result<(), AppError> {
        let custom = resolve_zsh_custom();
        let plugin_path = custom.join(relative_path);
        self.run_command("git",
            &["clone", repo, plugin_path.to_str().unwrap()]
        )?;

        Ok(())
    }

    pub fn configure_zshrc(&self) -> Result<(), AppError> {
        let home = env::var("HOME").map_err(System::EnvVarError)?;
        let zshrc_path = format!("{}/.zshrc", home);

        let content = fs::read_to_string(&zshrc_path)
            .map_err(FileSystem::FileNotFound)?;

        let mut new_lines = Vec::new();

        for line in content.lines() {
            if line.starts_with("ZSH_THEME=") {
                new_lines.push("ZSH_THEME=powerlevel10k/powerlevel10k".to_string());
            } else if line.starts_with("plugins=") {
                new_lines.push("plugins=(git zsh-autosuggestions zsh-syntax-highlighting)".to_string());
            } else {
                new_lines.push(line.to_string());
            }
        }

        fs::write(&zshrc_path, new_lines.join("\n")).map_err(FileSystem::FileWriteError)?;
        Ok(())
    }

    pub fn change_shell(&self) -> Result<(), AppError> {
        let zsh = which("zsh");

        match zsh {
            Ok(path) => {
                self.run_command("chsh", &["-s", path.to_str().unwrap()])?;
                Ok(())
            }
            Err(_) => Err(System::WhichFailed.into())
        }
    }

    fn run_command(&self, command: &str, args:&[&str]) -> Result<(), AppError> {
        let status = Command::new(command)
            .args(args)
            .stdout(Stdio::inherit())
            .status()
            .map_err(System::CommandFailed)?;

        if status.success() {
            Ok(())
        } else {
            Err(System::CommandExitError(command.to_string()).into())
        }
    }

    fn is_command_available(&self, command: &'static str) -> bool {
        let result = which(command);
        match result {
            Ok(_) => true,
            Err(_) => false
        }
    }
}
