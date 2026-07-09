//! `gh qwt list` — list repositories and their worktrees.
//!
//! See `docs/development/specification/README.md` (`list`).

use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};

use crate::{config, git, repo};

/// Arguments for `gh qwt list`.
#[derive(Debug, clap::Args)]
pub struct Args {
    /// Print absolute paths for worktrees.
    #[arg(short = 'p', long = "full-path")]
    pub full_path: bool,
}

/// Run the `list` command.
pub fn run(args: Args) -> Result<()> {
    let root = config::resolve_root()?;
    if !root.exists() {
        return Ok(());
    }

    for (repo_name, repo_dir) in qwt_repositories(&root) {
        println!("{repo_name}");

        let mut worktrees = match git::worktree_list(&repo_dir) {
            Ok(worktrees) => worktrees,
            Err(err) => {
                eprintln!("warning: failed to inspect {}: {err:#}", repo_dir.display());
                continue;
            }
        };

        worktrees.sort_by_key(worktree_sort_key);
        for worktree in worktrees {
            if args.full_path {
                println!("  {}", worktree.path.display());
            } else {
                println!("  {}", compact_worktree_name(&worktree));
            }
        }
    }

    Ok(())
}

fn qwt_repositories(root: &Path) -> Vec<(String, PathBuf)> {
    let mut repositories = Vec::new();

    for owner_dir in sorted_child_dirs(root) {
        let owner = file_name_string(&owner_dir);

        for repo_dir in sorted_child_dirs(&owner_dir) {
            if repo::is_qwt_repo(&repo_dir) {
                let repo = file_name_string(&repo_dir);
                repositories.push((format!("{owner}/{repo}"), repo_dir));
            }
        }
    }

    repositories
}

fn sorted_child_dirs(dir: &Path) -> Vec<PathBuf> {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(err) => {
            eprintln!("warning: failed to read {}: {err}", dir.display());
            return Vec::new();
        }
    };

    let mut dirs = Vec::new();
    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(err) => {
                eprintln!("warning: failed to read entry in {}: {err}", dir.display());
                continue;
            }
        };

        match entry.file_type() {
            Ok(file_type) if file_type.is_dir() => dirs.push(entry.path()),
            Ok(_) => {}
            Err(err) => eprintln!(
                "warning: failed to inspect {}: {err}",
                entry.path().display()
            ),
        }
    }

    dirs.sort_by_key(|path| file_name_string(path));
    dirs
}

fn compact_worktree_name(worktree: &git::Worktree) -> String {
    if let Some(branch) = &worktree.branch {
        return branch.clone();
    }

    let name = file_name_string(&worktree.path);
    if worktree.detached {
        format!("{name} (detached)")
    } else {
        name
    }
}

fn worktree_sort_key(worktree: &git::Worktree) -> String {
    worktree
        .branch
        .clone()
        .unwrap_or_else(|| worktree.path.display().to_string())
}

fn file_name_string(path: &Path) -> String {
    path.file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.display().to_string())
}
