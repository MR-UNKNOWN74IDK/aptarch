### **AptArch**

A command-line tool that translates APT commands into pacman operations, letting Debian/Ubuntu users use familiar APT commands seamlessly on Arch Linux.

## Introduction

AptArch is a command-line tool written in Rust that mimics the command interface of the APT package manager, but actually converts those commands into the corresponding pacman operations. This lets users who are used to APT get up and running on Arch Linux quickly, without having to learn a new command syntax.

## Supported Commands

| APT Command    | Description               | Corresponding pacman command  |
| -------------- | -------------------------- | ------------------------------ |
| `list`         | List packages              | `pacman -Q`/`-Sl`/`-Qu`        |
| `search`       | Search for packages        | `pacman -Ss`                   |
| `show`         | Show package details       | `pacman -Si`                   |
| `install`      | Install a package          | `pacman -S`                    |
| `reinstall`    | Reinstall a package        | `pacman -S` (with a warning)   |
| `remove`       | Remove a package           | `pacman -R`                    |
| `purge`        | Remove completely          | `pacman -Rns`                  |
| `autoremove`   | Remove orphaned packages   | `pacman -Rns $(pacman -Qdtq)`  |
| `update`       | Update package sources     | `pacman -Sy`                   |
| `upgrade`      | Upgrade the system         | `pacman -Su`                   |
| `full-upgrade` | Full system upgrade        | `pacman -Syu`                  |
| `clean`        | Clean the cache            | `pacman -Scc`                  |
| `autoclean`    | Clean old cache            | `pacman -Sc`                   |
| `why`          | Dependency analysis        | `pactree -r`                   |
| `why-not`      | Install-reason analysis    | `pactree -r`                   |

## Supported Options

| Option                | Function                          | Corresponding pacman option |
| --------------------- | ---------------------------------- | ---------------------------- |
| `-y, --yes`           | Auto-confirm                       | `--noconfirm`                 |
| `-d, --download-only` | Download only                      | `-Sw`                          |
| `--no-upgrade`        | Don't upgrade installed packages   | `--needed`                     |
| `--reinstall`         | Reinstall                          | Warning shown                  |
| `--purge`             | Remove completely                  | `-Rns`                          |
| `--no-color`          | Disable colored output             | -                               |
| `--color`             | Enable colored output              | -                               |

## Installation

### Build from source

```bash
# Clone the repository
git clone https://github.com/ZXCLF/aptarch.git
cd aptarch

# Build
cargo build --release

# Install into the system path (use the name `aptarch` to avoid conflicts)
sudo cp target/release/aptarch /usr/local/bin/aptarch

# Optional: create a symlink to use the `apt` command (only if the system does not already use apt)
# sudo ln -s /usr/local/bin/aptarch /usr/local/bin/apt
```

### Install from an AUR repository

```bash
# Install from the AUR
# https://aur.archlinux.org/packages/aptarch-git
yay -Syu aptarch-git

# Install from a [self-hosted repository](https://github.com/taotieren/aur-repo)
sudo pacman -Syu aptarch

# Show help
aptarch --help
```

## Usage Examples

### Basic operations

```bash
# Update package sources
sudo aptarch update

# Upgrade the system
sudo aptarch upgrade

# Install a package
sudo aptarch install firefox

# Search for a package
aptarch search python

# Show package information
aptarch show firefox

# Remove a package
sudo aptarch remove firefox

# Purge (remove completely)
sudo aptarch purge firefox

# Automatically remove orphaned packages
sudo aptarch autoremove
```

### Advanced options

```bash
# Auto-confirm installation
sudo aptarch install -y vim

# Download only, don't install
sudo aptarch install -d docker

# Don't upgrade already-installed packages
sudo aptarch install --no-upgrade nginx

# View dependency relationships
aptarch why firefox

# Query why a package cannot be installed
aptarch why-not some-package
```

### Easter eggs

```bash
# Regular cow
aptarch moo

# Advanced cow
aptarch moo moo

# Ultimate cow
aptarch moo moo moo
```

## Known Limitations

Due to differences in design philosophy between pacman and APT, the following commands/features are not supported:

| Unsupported feature       | Explanation                                                            |
| --------------------------- | ------------------------------------------------------------------------ |
| `changelog`                 | pacman does not support viewing changelogs; use `asp log` instead        |
| `source`                    | To get source code, use the ABS or asp tools                              |
| `edit-sources`              | Please edit `/etc/pacman.d/mirrorlist` manually                            |
| `satisfy`                   | pacman does not support dependency string resolution                       |
| `history-*`                 | pacman has no history rollback mechanism; check `/var/log/pacman.log`      |
| `--fix-broken`              | pacman automatically rolls back failed transactions                        |
| `--no-install-recommends`   | pacman has no concept of recommended dependencies                           |
| `-t/--target-release`       | Arch's rolling release model has no concept of release versions              |
| `modernize-sources`         | Not applicable to Arch Linux                                               |

## License

This project is licensed under GPL v3.0.

## Acknowledgments

- Inspired by [aptman](https://github.com/Limou233/aptman)
