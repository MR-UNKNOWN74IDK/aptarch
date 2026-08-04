use std::env;
use std::process::{exit, Command};

const RESET: &str = "\x1b[0m";
const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const CYAN: &str = "\x1b[36m";
const PURPLE: &str = "\x1b[35m";

fn is_root() -> bool {
    if let Ok(output) = Command::new("id").arg("-u").output() {
        if output.status.success() {
            return String::from_utf8_lossy(&output.stdout).trim() == "0";
        }
    }
    false
}

fn clr(use_color: bool, color: &str, text: &str) -> String {
    if use_color {
        format!("{}{}{}", color, text, RESET)
    } else {
        text.to_string()
    }
}

fn err(use_color: bool, msg: &str) -> String {
    clr(use_color, RED, &format!("E: {}", msg))
}

fn warn(use_color: bool, msg: &str) -> String {
    clr(use_color, YELLOW, &format!("W: {}", msg))
}

fn info(use_color: bool, msg: &str) -> String {
    clr(use_color, CYAN, msg)
}

fn print_usage(use_color: bool) {
    println!(r#"aptarch 1.0.0 (amd64)
    用法： apt [选项] 命令

    命令行软件包管理器 apt 提供软件包搜索，管理和信息查询等功能。
    它提供的功能与其他 APT 工具相同（像 apt-get 和 apt-cache），
    但是默认情况下被设置得更适合交互。

    常用命令：
    list - 根据名称列出软件包
    search - 搜索软件包描述
    show - 显示软件包细节
    install - 安装软件包
    reinstall - 重新安装软件包
    remove - 移除软件包
    purge - 彻底删除软件包及配置
    autoremove - 自动卸载所有不再使用的软件包
    update - 更新可用软件包列表
    upgrade - 通过 安装/升级 软件来更新系统
    full-upgrade - 通过 卸载/安装/升级 来更新系统
    edit-sources - 编辑软件源信息文件
    satisfy - 使系统满足依赖关系字符串
    why - 回溯依赖关系以解释软件包存在原因
    why-not - 回溯依赖关系以解释软件包不可安装原因
    history-list - 显示操作历史列表
    history-info - 显示特定事务的详细信息
    history-redo - 重新执行历史事务
    history-undo - 撤销历史事务
    history-rollback - 回滚到历史事务状态

    全局选项：
    -h, --help        显示简短的帮助信息。
    -v, --version     显示程序版本号。
    -o, --option      设置一个配置选项（忽略）。
    -c, --config-file 指定一个额外的配置文件（忽略）。
    --audit           显示审计信息（忽略）。
    --no-color        关闭彩色输出。
    --color           强制开启彩色输出。
    --cli-version     请求特定 CLI 版本（忽略）。

    参见 apt(8) 以获取更多关于可用命令的信息。
    本程序将 APT 命令转换为 pacman 操作。
    "#);
    println!(
        "{}",
        clr(use_color, YELLOW, "                                         本 APT 具有超级牛力。")
    );
}

fn moo() {
    println!("                  (__)");
    println!("                  (oo)");
    println!("            /------\\/");
    println!("           / |     ||");
    println!("          *  /\\---/\\");
    println!("             ~~   ~~");
    println!("...\"Have you mooed today?\"...");
}

fn moomoo() {
    println!("                     {}__{}", YELLOW, RESET);
    println!(
        "{}             -------{}~({}{}{})~",
        RED, YELLOW, CYAN, "..", YELLOW
    );
    println!(
        "{}               ,----{}\\{}({}{}{})",
        YELLOW, RED, YELLOW, PURPLE, "oo", YELLOW
    );
    println!("{}              /|____|,'", YELLOW);
    println!("{}               /\\   /\\", YELLOW);
    println!("{}wWwWwWwWwWwWwWwWwWwWwWwWwWwWw{}", GREEN, RESET);
    println!("...\"Have you mooed today?\"...");
}

fn moomoomoo() {
    println!("                       \\ /");
    println!("    mOOh  (__)        -(_)-");
    println!("       \\  ~Oo~___      / \\");
    println!("          (..)  |\\");
    println!("____________|_|_|____________");
    println!("...\"Have you mooed today?\"...");
}

/// Returns true if `pactree` is on PATH.
fn has_pactree() -> bool {
    Command::new("which")
        .arg("pactree")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Run a command via `Command` with individual `.arg()` calls (no shell).
/// Returns exit code or 1 on error.
fn run_cmd(cmd: &str, args: &[&str], use_color: bool) -> i32 {
    let status = Command::new(cmd)
        .args(args)
        .status()
        .unwrap_or_else(|e| {
            eprintln!("{}", err(use_color, &format!("执行命令失败: {}", e)));
            exit(1);
        });
    status.code().unwrap_or(1)
}

/// Run a pipeline: `first_cmd`'s stdout feeds into `second_cmd`'s args.
fn run_pipeline(
    first_cmd: &str,
    first_args: &[&str],
    second_cmd: &str,
    prefix_args: &[&str],
    use_color: bool,
) -> i32 {
    let out = Command::new(first_cmd)
        .args(first_args)
        .output()
        .unwrap_or_else(|e| {
            eprintln!("{}", err(use_color, &format!("执行命令失败: {}", e)));
            exit(1);
        });
    if !out.status.success() {
        return out.status.code().unwrap_or(1);
    }
    let pkgs = String::from_utf8_lossy(&out.stdout)
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
    let mut second_args: Vec<&str> = prefix_args.to_vec();
    for p in &pkgs {
        second_args.push(p.as_str());
    }
    run_cmd(second_cmd, &second_args, use_color)
}

/// Parse a subcommand's own `--flag` / `--value` arguments into a command and arg list.
/// Returns (program, vec_of_args).
fn build_pacman_command(
    flags: &str,
    rest: &[String],
    assume_yes: bool,
    download_only: bool,
    no_upgrade: bool,
    reinstall_flag: bool,
    purge_flag: bool,
    use_color: bool,
) -> (String, Vec<String>) {
    let mut program = "pacman".to_string();
    let mut args: Vec<String> = Vec::new();
    let mut cmd_flags = flags.to_string();

    // apply global flags
    if assume_yes {
        cmd_flags.push_str(" --noconfirm");
    }
    if no_upgrade {
        cmd_flags.push_str(" --needed");
    }
    if reinstall_flag {
        eprintln!(
            "{}",
            warn(use_color, "pacman 不支持直接重装，将尝试安装（可能不会重新下载）。")
        );
    }

    // install/reinstall subcommand handling of --purge global flag
    if purge_flag {
        eprintln!(
            "{}",
            warn(use_color, "`--purge` 不适用于 install 命令，已忽略。")
        );
    }

    let mut is_download_only = download_only;

    // parse subcommand-level flags
    let mut pkg_args: Vec<String> = Vec::new();
    for a in rest {
        match a.as_str() {
            "-y" | "--yes" | "--assume-yes" => {
                cmd_flags.push_str(" --noconfirm");
            }
            "-d" | "--download-only" => {
                is_download_only = true;
            }
            "--no-upgrade" => {
                cmd_flags.push_str(" --needed");
            }
            "--reinstall" => {}
            "--purge" => {}
            other => {
                pkg_args.push(other.to_string());
            }
        }
    }

    // rebuild program/flags based on download-only override
    if is_download_only {
        program = "pacman".to_string();
        cmd_flags = "-Sw".to_string();
        if assume_yes {
            cmd_flags.push_str(" --noconfirm");
        }
    }

    // split flags into args
    for f in cmd_flags.split_whitespace() {
        args.push(f.to_string());
    }
    args.extend(pkg_args);

    (program, args)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let argc = args.len();

    let mut use_color = true;
    let mut assume_yes = false;
    let mut download_only = false;
    let mut no_upgrade = false;
    let mut reinstall_flag = false;
    let mut purge_flag = false;

    let mut cmd_index = 1;
    while cmd_index < argc {
        let arg = &args[cmd_index];
        match arg.as_str() {
            "-h" | "--help" => {
                print_usage(use_color);
                exit(0);
            }
            "-v" | "--version" => {
                println!("aptarch 1.0.0 (amd64)");
                println!("基于 pacman 的 APT 界面");
                exit(0);
            }
            "-o" | "--option" => {
                if cmd_index + 1 < argc {
                    cmd_index += 2;
                } else {
                    eprintln!("{}", err(use_color, "选项 -o 需要一个参数"));
                    exit(1);
                }
                continue;
            }
            "-c" | "--config-file" => {
                if cmd_index + 1 < argc {
                    cmd_index += 2;
                } else {
                    eprintln!("{}", err(use_color, "选项 -c 需要一个参数"));
                    exit(1);
                }
                continue;
            }
            "--audit" | "--cli-version" => {
                cmd_index += 1;
                continue;
            }
            "--no-color" => {
                use_color = false;
                cmd_index += 1;
                continue;
            }
            "--color" => {
                use_color = true;
                cmd_index += 1;
                continue;
            }
            "-y" | "--yes" | "--assume-yes" => {
                assume_yes = true;
                cmd_index += 1;
                continue;
            }
            "-d" | "--download-only" => {
                download_only = true;
                cmd_index += 1;
                continue;
            }
            "--no-upgrade" => {
                no_upgrade = true;
                cmd_index += 1;
                continue;
            }
            "--reinstall" => {
                reinstall_flag = true;
                cmd_index += 1;
                continue;
            }
            "--purge" => {
                purge_flag = true;
                cmd_index += 1;
                continue;
            }
            other => {
                if other.starts_with('-') {
                    eprintln!("{}", err(use_color, &format!("未知选项 {}", other)));
                    exit(1);
                }
                break;
            }
        }
    }

    if cmd_index >= argc {
        if assume_yes || download_only || no_upgrade || reinstall_flag || purge_flag {
            eprintln!("{}", err(use_color, "选项必须与命令同时使用"));
        }
        print_usage(use_color);
        exit(1);
    }

    let cmd = args[cmd_index].clone();
    let rest_args = &args[cmd_index + 1..];

    if cmd == "moo" {
        if rest_args.len() >= 2 && rest_args[0] == "moo" && rest_args[1] == "moo" {
            moomoomoo();
        } else if rest_args.len() >= 1 && rest_args[0] == "moo" {
            moomoo();
        } else {
            moo();
        }
        exit(0);
    }

    let needs_root = matches!(
        cmd.as_str(),
        "install" | "reinstall" | "remove" | "purge" | "update"
            | "upgrade" | "full-upgrade" | "autoremove" | "clean" | "autoclean"
    );
    if needs_root && !is_root() {
        eprintln!("{}", err(use_color, "需要 root 权限才能执行此操作。"));
        exit(1);
    }

    match cmd.as_str() {
        "changelog" => {
            eprintln!("{}", err(use_color, "无效的操作 changelog"));
            println!("pacman 不支持查看变更日志。请从 AUR 安装 asp 并使用 `asp log` 命令。");
            exit(1);
        }
        "source" => {
            eprintln!("{}", err(use_color, "无效的操作 source"));
            println!("pacman 不支持获取源代码。请使用 ABS 或 asp 工具。");
            exit(1);
        }
        "edit-sources" => {
            eprintln!("{}", err(use_color, "无效的操作 edit-sources"));
            println!("pacman 不支持直接编辑软件源。请手动编辑 /etc/pacman.d/mirrorlist 文件。");
            exit(1);
        }
        "satisfy" => {
            eprintln!("{}", err(use_color, "pacman 不支持 satisfy 命令"));
            println!("无法自动满足依赖关系字符串。");
            exit(1);
        }
        "--fix-broken" => {
            eprintln!("{}", err(use_color, "无法识别的选项 --fix-broken"));
            println!("pacman 在事务失败时会自动回滚，无需单独修复。");
            exit(1);
        }
        "--no-install-recommends" => {
            eprintln!("{}", err(use_color, "无法识别的选项 --no-install-recommends"));
            println!("pacman 不支持推荐依赖机制，所有依赖均为强制安装。");
            exit(1);
        }
        "-t" | "--target-release" => {
            eprintln!("{}", err(use_color, &format!("无法识别的选项 {}", cmd)));
            println!("Arch Linux 采用滚动更新，不支持指定发行版本。");
            exit(1);
        }
        "-a" | "--architecture" => {
            eprintln!("{}", err(use_color, "无需指定架构"));
            println!("pacman 会自动处理多架构，无需手动干预。");
            exit(1);
        }
        "history-list" | "history-info" | "history-redo" | "history-undo" | "history-rollback" => {
            eprintln!("{}", err(use_color, "此系统不支持 APT 历史功能"));
            println!("pacman 没有内置历史回滚机制，但您可以查看 /var/log/pacman.log 获取操作记录。");
            exit(1);
        }
        "modernize-sources" => {
            eprintln!("{}", err(use_color, "modernize-sources 不适用于 Arch"));
            exit(1);
        }
        _ => {}
    }

    let status = match cmd.as_str() {
        "update" => run_cmd("pacman", &["-Sy"], use_color),
        "upgrade" => run_cmd("pacman", &["-Su"], use_color),
        "full-upgrade" => run_cmd("pacman", &["-Syu"], use_color),
        "clean" => run_cmd("pacman", &["-Scc"], use_color),
        "autoclean" => run_cmd("pacman", &["-Sc"], use_color),
        "autoremove" | "autopurge" => {
            eprintln!("{}", warn(use_color, "将会移除所有孤立的依赖包。"));
            if assume_yes {
                run_pipeline(
                    "pacman", &["-Qdtq"], "pacman", &["-Rns", "--noconfirm"],
                    use_color,
                )
            } else {
                run_pipeline("pacman", &["-Qdtq"], "pacman", &["-Rns"], use_color)
            }
        }
        "install" | "reinstall" => {
            let (prog, args_vec) = build_pacman_command(
                "-S", rest_args, assume_yes, download_only, no_upgrade,
                reinstall_flag, purge_flag, use_color,
            );
            let arg_refs: Vec<&str> = args_vec.iter().map(|s| s.as_str()).collect();
            run_cmd(&prog, &arg_refs, use_color)
        }
        "remove" => {
            let purge = purge_flag || rest_args.contains(&"--purge".to_string());
            let mut args_vec: Vec<String> = Vec::new();
            let flags = if purge { "-Rns" } else { "-R" };
            for f in flags.split_whitespace() {
                args_vec.push(f.to_string());
            }
            if assume_yes {
                args_vec.push("--noconfirm".to_string());
            }
            for a in rest_args {
                if a != "--purge" {
                    args_vec.push(a.clone());
                }
            }
            let arg_refs: Vec<&str> = args_vec.iter().map(|s| s.as_str()).collect();
            run_cmd("pacman", &arg_refs, use_color)
        }
        "purge" => {
            let mut args_vec: Vec<String> = vec!["-Rns".to_string()];
            if assume_yes {
                args_vec.push("--noconfirm".to_string());
            }
            args_vec.extend(rest_args.to_vec());
            let arg_refs: Vec<&str> = args_vec.iter().map(|s| s.as_str()).collect();
            run_cmd("pacman", &arg_refs, use_color)
        }
        "search" => {
            let mut args_vec: Vec<String> = vec!["-Ss".to_string()];
            args_vec.extend(rest_args.to_vec());
            let arg_refs: Vec<&str> = args_vec.iter().map(|s| s.as_str()).collect();
            run_cmd("pacman", &arg_refs, use_color)
        }
        "show" => {
            let mut args_vec: Vec<String> = vec!["-Si".to_string()];
            args_vec.extend(rest_args.to_vec());
            let arg_refs: Vec<&str> = args_vec.iter().map(|s| s.as_str()).collect();
            run_cmd("pacman", &arg_refs, use_color)
        }
        "download" => {
            let mut args_vec: Vec<String> = vec!["-Sw".to_string()];
            args_vec.extend(rest_args.to_vec());
            let arg_refs: Vec<&str> = args_vec.iter().map(|s| s.as_str()).collect();
            run_cmd("pacman", &arg_refs, use_color)
        }
        "list" => {
            let mut installed = false;
            let mut upgradeable = false;
            let mut list_pkgs: Vec<String> = Vec::new();
            for a in rest_args {
                match a.as_str() {
                    "--installed" => installed = true,
                    "--upgradeable" => upgradeable = true,
                    "--all-versions" => {}
                    other => {
                        if !other.starts_with('-') {
                            list_pkgs.push(other.to_string());
                        }
                    }
                }
            }
            let mut args_vec: Vec<String> = Vec::new();
            if installed {
                args_vec.push("-Q".to_string());
            } else if upgradeable {
                args_vec.push("-Qu".to_string());
            } else {
                args_vec.push("-Sl".to_string());
            }
            args_vec.extend(list_pkgs);
            let arg_refs: Vec<&str> = args_vec.iter().map(|s| s.as_str()).collect();
            run_cmd("pacman", &arg_refs, use_color)
        }
        "why" => {
            if rest_args.is_empty() {
                eprintln!("{}", err(use_color, "请指定一个软件包以查询依赖原因"));
                exit(1);
            }
            let pkg = &rest_args[0];
            if has_pactree() {
                run_cmd("pactree", &["-r", pkg], use_color)
            } else {
                eprintln!(
                    "{}",
                    warn(
                        use_color,
                        "未安装 pactree (pacman-contrib)，降级使用 pacman -Qi 显示依赖信息。"
                    )
                );
                run_cmd("pacman", &["-Qi", pkg], use_color)
            }
        }
        "why-not" => {
            if rest_args.is_empty() {
                eprintln!("{}", err(use_color, "请指定一个软件包以查询为何无法安装"));
                exit(1);
            }
            let pkg = &rest_args[0];
            if has_pactree() {
                println!(
                    "{}",
                    info(
                        use_color,
                        "注意：以下输出显示依赖关系，可能有助于分析为何该包未被安装。"
                    )
                );
                run_cmd("pactree", &["-r", pkg], use_color)
            } else {
                println!(
                    "{}",
                    info(use_color, "未安装 pactree，使用 pacman -Qi 显示信息。")
                );
                run_cmd("pacman", &["-Qi", pkg], use_color)
            }
        }
        _ => {
            eprintln!("{}", err(use_color, &format!("无效的操作 {}", cmd)));
            exit(1);
        }
    };

    exit(status);
}
