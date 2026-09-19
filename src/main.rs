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
    Usage: apt [options] command

    The command-line package manager apt provides package search,
    management, and information query functions. It offers the same
    functionality as other APT tools (like apt-get and apt-cache),
    but is set up by default to be more suitable for interactive use.

    Common commands:
    list - list packages by name
    search - search package descriptions
    show - show package details
    install - install packages
    reinstall - reinstall packages
    remove - remove packages
    purge - remove packages and their configuration completely
    autoremove - automatically remove all unused packages
    update - update the list of available packages
    upgrade - upgrade the system by installing/upgrading packages
    full-upgrade - upgrade the system by removing/installing/upgrading packages
    edit-sources - edit source information files
    satisfy - satisfy dependency strings
    why - trace back dependencies to explain why a package exists
    why-not - trace back dependencies to explain why a package cannot be installed
    history-list - show the list of operation history
    history-info - show details for a specific transaction
    history-redo - redo a historical transaction
    history-undo - undo a historical transaction
    history-rollback - roll back to a historical transaction state

    Global options:
    -h, --help        show a short help message.
    -v, --version     show the program version.
    -o, --option      set a configuration option (ignored).
    -c, --config-file specify an additional configuration file (ignored).
    --audit           show audit information (ignored).
    --no-color        turn off colored output.
    --color           force colored output on.
    --cli-version     request a specific CLI version (ignored).

    See apt(8) for more information about available commands.
    This program translates APT commands into pacman operations.
    "#);
    println!(
        "{}",
        clr(use_color, YELLOW, "                                         This APT has Super Cow Powers.")
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
            eprintln!("{}", err(use_color, &format!("Failed to execute command: {}", e)));
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
            eprintln!("{}", err(use_color, &format!("Failed to execute command: {}", e)));
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
            warn(use_color, "pacman does not support direct reinstallation; will attempt install instead (may not re-download).")
        );
    }

    // install/reinstall subcommand handling of --purge global flag
    if purge_flag {
        eprintln!(
            "{}",
            warn(use_color, "`--purge` does not apply to the install command; ignored.")
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
                println!("An APT interface based on pacman");
                exit(0);
            }
            "-o" | "--option" => {
                if cmd_index + 1 < argc {
                    cmd_index += 2;
                } else {
                    eprintln!("{}", err(use_color, "Option -o requires an argument"));
                    exit(1);
                }
                continue;
            }
            "-c" | "--config-file" => {
                if cmd_index + 1 < argc {
                    cmd_index += 2;
                } else {
                    eprintln!("{}", err(use_color, "Option -c requires an argument"));
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
                    eprintln!("{}", err(use_color, &format!("Unknown option {}", other)));
                    exit(1);
                }
                break;
            }
        }
    }

    if cmd_index >= argc {
        if assume_yes || download_only || no_upgrade || reinstall_flag || purge_flag {
            eprintln!("{}", err(use_color, "Options must be used together with a command"));
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
        eprintln!("{}", err(use_color, "Root privileges are required to perform this operation."));
        exit(1);
    }

    match cmd.as_str() {
        "changelog" => {
            eprintln!("{}", err(use_color, "Invalid operation: changelog"));
            println!("pacman does not support viewing changelogs. Install asp from the AUR and use the `asp log` command.");
            exit(1);
        }
        "source" => {
            eprintln!("{}", err(use_color, "Invalid operation: source"));
            println!("pacman does not support fetching source code. Please use the ABS or asp tools.");
            exit(1);
        }
        "edit-sources" => {
            eprintln!("{}", err(use_color, "Invalid operation: edit-sources"));
            println!("pacman does not support directly editing package sources. Please manually edit the /etc/pacman.d/mirrorlist file.");
            exit(1);
        }
        "satisfy" => {
            eprintln!("{}", err(use_color, "pacman does not support the satisfy command"));
            println!("Cannot automatically satisfy the dependency string.");
            exit(1);
        }
        "--fix-broken" => {
            eprintln!("{}", err(use_color, "Unrecognized option --fix-broken"));
            println!("pacman automatically rolls back on transaction failure, so no separate fix is needed.");
            exit(1);
        }
        "--no-install-recommends" => {
            eprintln!("{}", err(use_color, "Unrecognized option --no-install-recommends"));
            println!("pacman has no concept of recommended dependencies; all dependencies are mandatory.");
            exit(1);
        }
        "-t" | "--target-release" => {
            eprintln!("{}", err(use_color, &format!("Unrecognized option {}", cmd)));
            println!("Arch Linux uses a rolling release model and does not support specifying a release version.");
            exit(1);
        }
        "-a" | "--architecture" => {
            eprintln!("{}", err(use_color, "No need to specify an architecture"));
            println!("pacman automatically handles multiple architectures; no manual intervention is needed.");
            exit(1);
        }
        "history-list" | "history-info" | "history-redo" | "history-undo" | "history-rollback" => {
            eprintln!("{}", err(use_color, "This system does not support APT history features"));
            println!("pacman has no built-in history rollback mechanism, but you can check /var/log/pacman.log for a record of operations.");
            exit(1);
        }
        "modernize-sources" => {
            eprintln!("{}", err(use_color, "modernize-sources does not apply to Arch"));
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
            eprintln!("{}", warn(use_color, "This will remove all orphaned dependency packages."));
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
                eprintln!("{}", err(use_color, "Please specify a package to query the reason for its dependency"));
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
                        "pactree (pacman-contrib) is not installed; falling back to pacman -Qi to show dependency information."
                    )
                );
                run_cmd("pacman", &["-Qi", pkg], use_color)
            }
        }
        "why-not" => {
            if rest_args.is_empty() {
                eprintln!("{}", err(use_color, "Please specify a package to query why it cannot be installed"));
                exit(1);
            }
            let pkg = &rest_args[0];
            if has_pactree() {
                println!(
                    "{}",
                    info(
                        use_color,
                        "Note: the output below shows dependency relationships, which may help explain why this package was not installed."
                    )
                );
                run_cmd("pactree", &["-r", pkg], use_color)
            } else {
                println!(
                    "{}",
                    info(use_color, "pactree is not installed; using pacman -Qi to show information.")
                );
                run_cmd("pacman", &["-Qi", pkg], use_color)
            }
        }
        _ => {
            eprintln!("{}", err(use_color, &format!("Invalid operation: {}", cmd)));
            exit(1);
        }
    };

    exit(status);
}
