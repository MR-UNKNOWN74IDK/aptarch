### **AptArch**

一个将 APT 命令转换为 pacman 操作的命令行工具，让 Debian/Ubuntu 用户能够在 Arch Linux 系统上无缝使用熟悉的 APT 命令。

## 简介

APTMan 是一个用 Rust 编写的命令行工具，它模拟了 APT 包管理器的命令接口，但实际上将这些命令转换为对应的 pacman 操作。这使得习惯使用 APT 的用户能够快速上手 Arch Linux，而无需学习新的命令语法。

## 支持的命令

| APT 命令       | 功能描述       | 对应的 pacman 命令            |
| -------------- | -------------- | ----------------------------- |
| `list`         | 列出软件包     | `pacman -Q`/`-Sl`/`-Qu`       |
| `search`       | 搜索软件包     | `pacman -Ss`                  |
| `show`         | 显示软件包详情 | `pacman -Si`                  |
| `install`      | 安装软件包     | `pacman -S`                   |
| `reinstall`    | 重新安装       | `pacman -S`（警告）           |
| `remove`       | 移除软件包     | `pacman -R`                   |
| `purge`        | 彻底删除       | `pacman -Rns`                 |
| `autoremove`   | 移除孤立包     | `pacman -Rns $(pacman -Qdtq)` |
| `update`       | 更新软件源     | `pacman -Sy`                  |
| `upgrade`      | 升级系统       | `pacman -Su`                  |
| `full-upgrade` | 完全升级       | `pacman -Syu`                 |
| `clean`        | 清理缓存       | `pacman -Scc`                 |
| `autoclean`    | 清理旧缓存     | `pacman -Sc`                  |
| `why`          | 依赖关系分析   | `pactree -r`                  |
| `why-not`      | 安装原因分析   | `pactree -r`                  |

## 支持的选项

| 选项                  | 功能           | 对应 pacman 选项 |
| --------------------- | -------------- | ---------------- |
| `-y, --yes`           | 自动确认       | `--noconfirm`    |
| `-d, --download-only` | 仅下载         | `-Sw`            |
| `--no-upgrade`        | 不升级已安装包 | `--needed`       |
| `--reinstall`         | 重新安装       | 警告提示         |
| `--purge`             | 彻底删除       | `-Rns`           |
| `--no-color`          | 关闭彩色输出   | -                |
| `--color`             | 开启彩色输出   | -                |

## 安装

### 从源码编译

```bash
# 克隆仓库
git clone https://github.com/ZXCLF/aptarch.git
cd aptarch

# 编译
cargo build --release

# 安装到系统路径
sudo cp target/release/aptarch /usr/local/bin/apt
```

## 使用示例

### 基本操作

```bash
# 更新软件源
sudo apt update

# 升级系统
sudo apt upgrade

# 安装软件包
sudo apt install firefox

# 搜索软件包
apt search python

# 显示软件包信息
apt show firefox

# 移除软件包
sudo apt remove firefox

# 彻底删除
sudo apt purge firefox

# 自动移除孤立包
sudo apt autoremove
```

### 高级选项

```bash
# 自动确认安装
sudo apt install -y vim

# 仅下载不安装
sudo apt install -d docker

# 不升级已安装的包
sudo apt install --no-upgrade nginx

# 查看依赖关系
apt why firefox

# 查询无法安装的原因
apt why-not some-package
```

### 彩蛋

```bash
# 普通奶牛
apt moo

# 进阶奶牛
apt moo moo

# 终极奶牛
apt moo moo moo
```

## 已知限制

由于 pacman 和 APT 在设计理念上的差异，以下命令/功能不被支持：

| 不支持的功能              | 说明                                              |
| ------------------------- | ------------------------------------------------- |
| `changelog`               | pacman 不支持查看变更日志，可使用 `asp log`       |
| `source`                  | 获取源代码请使用 ABS 或 asp 工具                  |
| `edit-sources`            | 请手动编辑 `/etc/pacman.d/mirrorlist`             |
| `satisfy`                 | pacman 不支持依赖字符串解析                       |
| `history-*`               | pacman 无历史回滚机制，查看 `/var/log/pacman.log` |
| `--fix-broken`            | pacman 事务失败自动回滚                           |
| `--no-install-recommends` | pacman 无推荐依赖概念                             |
| `-t/--target-release`     | Arch 滚动更新无版本概念                           |
| `modernize-sources`       | 不适用于 Arch Linux                               |

## 许可证

本项目采用 GPL v3.0 许可证。

## 致谢

- 灵感来源于 [aptman](https://github.com/Limou233/aptman)