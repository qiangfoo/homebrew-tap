use std::env;
use std::path::PathBuf;
use std::process::{self, Command};

use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Select};
use indicatif::ProgressBar;
use serde::Deserialize;

#[derive(Parser)]
#[command(name = "work", about = "Interactive git worktree manager")]
struct Cli {
    #[command(subcommand)]
    command: Option<SubCommand>,
}

#[derive(Parser)]
enum SubCommand {
    /// Add a new worktree
    #[command(alias = "create")]
    Add,
    /// Remove a worktree
    #[command(alias = "delete")]
    Remove,
    /// Print shell integration script
    Init,
}

#[derive(Deserialize, Default)]
struct Config {
    branch_prefix: Option<String>,
}

fn load_config() -> Config {
    let path = dirs::home_dir()
        .map(|h| h.join(".config/work.toml"))
        .unwrap_or_default();
    std::fs::read_to_string(&path)
        .ok()
        .and_then(|s| toml::from_str(&s).ok())
        .unwrap_or_default()
}

/// Returns the root of the main worktree (the one that owns .git as a directory).
fn main_worktree_path() -> Result<PathBuf, String> {
    let output = Command::new("git")
        .args(["worktree", "list", "--porcelain"])
        .output()
        .map_err(|e| format!("failed to run git: {e}"))?;
    if !output.status.success() {
        return Err("not in a git repository".into());
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    // The first entry is always the main worktree
    for line in stdout.lines() {
        if let Some(path) = line.strip_prefix("worktree ") {
            return Ok(PathBuf::from(path));
        }
    }
    Err("could not determine main worktree".into())
}

struct Worktree {
    path: PathBuf,
    branch: String,
    is_bare: bool,
}

fn list_worktrees() -> Result<Vec<Worktree>, String> {
    let output = Command::new("git")
        .args(["worktree", "list", "--porcelain"])
        .output()
        .map_err(|e| format!("failed to run git: {e}"))?;
    if !output.status.success() {
        return Err("not in a git repository".into());
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut worktrees = Vec::new();
    let mut current_path: Option<PathBuf> = None;
    let mut current_branch = String::new();
    let mut is_bare = false;

    for line in stdout.lines() {
        if let Some(path) = line.strip_prefix("worktree ") {
            if let Some(p) = current_path.take() {
                worktrees.push(Worktree {
                    path: p,
                    branch: std::mem::take(&mut current_branch),
                    is_bare,
                });
                is_bare = false;
            }
            current_path = Some(PathBuf::from(path));
        } else if let Some(branch_ref) = line.strip_prefix("branch ") {
            current_branch = branch_ref
                .strip_prefix("refs/heads/")
                .unwrap_or(branch_ref)
                .to_string();
        } else if line == "bare" {
            is_bare = true;
        }
    }
    if let Some(p) = current_path {
        worktrees.push(Worktree {
            path: p,
            branch: current_branch,
            is_bare,
        });
    }
    Ok(worktrees)
}

/// Print a cd command that the shell wrapper can eval.
fn emit_cd(path: &std::path::Path) {
    println!("cd:{}", path.display());
}

fn prompt_text(prompt: &str) -> String {
    dialoguer::Input::<String>::new()
        .with_prompt(prompt)
        .interact_text()
        .unwrap_or_else(|_| process::exit(1))
}

fn do_add(config: &Config) {
    let name = prompt_text("Worktree name");
    let name = name.trim().replace(' ', "-");
    if name.is_empty() {
        eprintln!("name cannot be empty");
        process::exit(1);
    }

    let branch = match &config.branch_prefix {
        Some(prefix) => format!("{prefix}-{name}"),
        None => name.clone(),
    };

    let main_path = main_worktree_path().unwrap_or_else(|e| {
        eprintln!("error: {e}");
        process::exit(1);
    });
    let worktree_path = main_path.parent().unwrap_or(&main_path).join(&name);

    let status = Command::new("git")
        .args([
            "worktree",
            "add",
            "-b",
            &branch,
            worktree_path.to_str().unwrap(),
        ])
        .status()
        .unwrap_or_else(|e| {
            eprintln!("failed to run git: {e}");
            process::exit(1);
        });
    if !status.success() {
        process::exit(1);
    }

    emit_cd(&worktree_path);
}

fn select_worktree(prompt: &str) -> Option<Worktree> {
    let worktrees = list_worktrees().unwrap_or_else(|e| {
        eprintln!("error: {e}");
        process::exit(1);
    });

    let worktrees: Vec<Worktree> = worktrees.into_iter().filter(|w| !w.is_bare).collect();

    if worktrees.is_empty() {
        eprintln!("no worktrees found");
        return None;
    }

    let display: Vec<String> = worktrees
        .iter()
        .map(|w| {
            let dir_name = w
                .path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            if w.branch.is_empty() {
                dir_name
            } else {
                format!("{dir_name} [{branch}]", branch = w.branch)
            }
        })
        .collect();

    let cwd = env::current_dir().ok();
    let default_idx = cwd
        .as_ref()
        .and_then(|cwd| worktrees.iter().position(|w| w.path == *cwd))
        .unwrap_or(0);

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .items(&display)
        .default(default_idx)
        .interact_opt()
        .unwrap_or_else(|_| process::exit(1));

    selection.map(|i| worktrees.into_iter().nth(i).unwrap())
}

fn do_remove() {
    let main_path = main_worktree_path().unwrap_or_else(|e| {
        eprintln!("error: {e}");
        process::exit(1);
    });

    let Some(worktree) = select_worktree("Select worktree to remove") else {
        return;
    };

    if worktree.path == main_path {
        eprintln!("cannot remove the main worktree");
        process::exit(1);
    }

    let spinner = ProgressBar::new_spinner();
    spinner.set_message("Removing worktree...");
    spinner.enable_steady_tick(std::time::Duration::from_millis(80));

    let status = Command::new("git")
        .args([
            "worktree",
            "remove",
            worktree.path.to_str().unwrap(),
            "--force",
        ])
        .output()
        .unwrap_or_else(|e| {
            spinner.finish_and_clear();
            eprintln!("failed to run git: {e}");
            process::exit(1);
        });

    if !status.status.success() {
        spinner.finish_and_clear();
        eprintln!("{}", String::from_utf8_lossy(&status.stderr));
        process::exit(1);
    }

    // Delete the branch too
    if !worktree.branch.is_empty() {
        let _ = Command::new("git")
            .args(["branch", "-D", &worktree.branch])
            .output();
    }

    spinner.finish_and_clear();

    emit_cd(&main_path);
}

fn do_init() {
    print!(
        r#"work() {{
    local output
    output="$(command work "$@")"
    local ret=$?

    if [[ $ret -ne 0 ]]; then
        return $ret
    fi

    while IFS= read -r line; do
        if [[ "$line" == cd:* ]]; then
            cd "${{line#cd:}}" || return 1
        else
            echo "$line"
        fi
    done <<< "$output"
}}"#
    );
}

fn do_goto() {
    let Some(worktree) = select_worktree("Select worktree") else {
        return;
    };
    emit_cd(&worktree.path);
}

fn main() {
    let cli = Cli::parse();
    let config = load_config();

    match cli.command {
        Some(SubCommand::Init) => do_init(),
        Some(SubCommand::Add) => do_add(&config),
        Some(SubCommand::Remove) => do_remove(),
        None => do_goto(),
    }
}
