# ZSH Setup

![Rust Version](https://img.shields.io/badge/1.97-%23FFFFFF?logo=rust&logoColor=%23000000)
![Linux Support](https://img.shields.io/badge/Linux-%23FFFFFF?logo=linux)
![Termux Support](https://img.shields.io/badge/Termux-%23FFFFFF?logo=android)
![License](https://img.shields.io/github/license/lamefate22/zsh-setup?logo=license)

## Overview
A lightweight, Rust-powered CLI tool designed to automate and streamline your Zsh shell configuration:
  - Installs **Oh My Zsh**
  - Configures the **Powerlevel10k** theme
  - Installs essential plugins (`zsh-syntax-highlighting`, `zsh-autosuggestions`)
  - Automatically updates and applies changes to your `.zshrc`

---

## Installation

#### Quick Start (Recommended)

Download the latest pre-compiled binary directly from the [Releases](https://github.com/lamefate22/zsh-setup/releases/latest) page.

#### Build from Source

1. Clone the repository:
   ```bash
   git clone https://github.com/lamefate22/zsh-setup.git
   ```
2. Build the project:
   ```bash
   cd zsh-setup
   cargo build --release
   ```

## Usage

Run the executable to start the automated setup:

```bash
./zsh-setup
```
