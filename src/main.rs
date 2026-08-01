use core::{platform, log};
use std::process::exit;

mod errors;
mod core;

fn main() {
    if log::initialize_logger().is_err() {
        tracing::error!("Logger initialization failed!");
        exit(0);
    }

    tracing::info!("Logger initialized.");

    let pf = platform::Platform::new();

    let deps = pf.is_dependencies_installed();
    if !deps {
        tracing::error!("Not all dependencies are installed (zsh, git, curl)!");
        exit(0);
    }

    tracing::info!("All dependencies found.");
    tracing::info!("Installing OhMyZsh...");

    let ohmyzsh = pf.install_ohmyzsh();
    if ohmyzsh.is_err() {
        tracing::error!("Failed to install OhMyZsh!");
        exit(0);
    }

    tracing::info!("Installed OhMyZsh.");
    tracing::info!("Installing theme...");

    let theme = pf.install_theme();
    if theme.is_err() {
        tracing::error!("Failed to install powerlevel10k theme!");
        exit(0);
    }

    tracing::info!("Installed powerlevel10k theme.");
    tracing::info!("Installing zsh-syntax-highlighting plugin...");

    let syntax = pf.install_plugin("https://github.com/zsh-users/zsh-syntax-highlighting.git", "plugins/zsh-syntax-highlighting");
    if syntax.is_err() {
        tracing::error!("Failed to install zsh-syntax-highlighting plugin!");
        exit(0);
    }

    tracing::info!("Installed zsh-syntax-highlighting plugin.");
    tracing::info!("Installing zsh-autosuggestions plugin...");

    let autosuggestions = pf.install_plugin("https://github.com/zsh-users/zsh-autosuggestions", "plugins/zsh-autosuggestions");
    if autosuggestions.is_err() {
        tracing::error!("Failed to install zsh-autosuggestions plugin!");
        exit(0);
    }

    tracing::info!("Installed zsh-autosuggestions plugin.");
    tracing::info!("Configuring ~/.zshrc...");

    let zshrc = pf.configure_zshrc();
    if zshrc.is_err() {
        tracing::error!("Failed to change ~/.zshrc configuration!");
        exit(0);
    }

    tracing::info!("Updated configuration of ~/.zshrc");
    tracing::info!("Changing default user shell...");

    let chsh = pf.change_shell();
    if chsh.is_err() {
        tracing::error!("Failed to change user shell!");
        tracing::error!("Run `chsh` manually and change it!");
    }
}
