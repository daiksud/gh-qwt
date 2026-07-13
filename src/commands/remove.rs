//! `gh qwt remove` (aliased `rm`) — remove a worktree or an entire repository.
//!
//! See `docs/development/specification/README.md` (`remove`).

use anyhow::{bail, Context, Result};
use std::io::{self, BufRead, Write};
use std::path::Path;

use crate::{config, git, repo};

/// Arguments for `gh qwt remove` / `gh qwt rm`.
#[derive(Debug, clap::Args)]
pub struct Args {
    /// A branch name, resolved by discovering the repository from the
    /// current directory, or an explicit `owner/repo` (whole repository) or
    /// `owner/repo/branch` (single worktree) spec, resolved directly when
    /// the current directory is not inside a qwt repository.
    #[arg(value_name = "SPEC")]
    pub spec: String,

    /// Remove a worktree even when it has local changes, or skip the
    /// confirmation prompt when removing an entire repository.
    #[arg(long)]
    pub force: bool,

    /// Also delete the local branch with `git branch -D`. Only applies when
    /// removing a single worktree.
    #[arg(long = "delete-branch")]
    pub delete_branch: bool,
}

/// Run the `remove` / `rm` command.
pub fn run(args: Args) -> Result<()> {
    let current_dir = std::env::current_dir().context("failed to get current directory")?;

    // Inside a qwt repository, the whole spec is a branch name for *that*
    // repository -- unchanged from the original `rm`, regardless of how many
    // `/` the branch name itself contains.
    if let Ok(repo_dir) = repo::discover_repo_root(&current_dir) {
        return remove_worktree(&repo_dir, &args.spec, args.force, args.delete_branch);
    }

    // Otherwise, the spec must be explicit: `owner/repo` or `owner/repo/branch`.
    let root = config::resolve_root()?;
    let segments: Vec<&str> = args
        .spec
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect();

    match segments.as_slice() {
        [owner, repo_name] => remove_repo(&root, owner, repo_name, args.force),
        [owner, repo_name, branch @ ..] => {
            let repo_dir = repo::repo_dir(&root, owner, repo_name);
            if !repo::is_qwt_repo(&repo_dir) {
                bail!("not a qwt repository: {}", repo_dir.display());
            }
            remove_worktree(&repo_dir, &branch.join("/"), args.force, args.delete_branch)
        }
        _ => Err(crate::error::usage(
            "not inside a qwt repository; SPEC must be a branch name (run inside a repository) \
             or owner/repo[/branch]",
        )),
    }
}

/// Remove a single worktree for `branch` under `repo_dir`.
fn remove_worktree(repo_dir: &Path, branch: &str, force: bool, delete_branch: bool) -> Result<()> {
    let dest = repo::worktree_in(repo_dir, branch);

    if !dest.exists() {
        bail!("no worktree for branch '{}' at {}", branch, dest.display());
    }

    git::worktree_remove(repo_dir, &dest, force)?;
    repo::remove_empty_worktree_ancestors(repo_dir, &dest)?;

    if delete_branch {
        git::branch_delete(repo_dir, branch)?;
    }

    println!("Removed worktree {}", dest.display());
    Ok(())
}

/// Remove an entire qwt-managed repository directory (`.bare` plus every
/// worktree), after confirmation unless `force`.
fn remove_repo(root: &Path, owner: &str, repo_name: &str, force: bool) -> Result<()> {
    let repo_dir = repo::repo_dir(root, owner, repo_name);

    if repo_dir == root || repo_dir.parent() == Some(root) || !repo_dir.starts_with(root) {
        bail!(
            "refusing to remove unsafe repository path: {}",
            repo_dir.display()
        );
    }

    if !repo::is_qwt_repo(&repo_dir) {
        bail!("not a qwt repository: {}", repo_dir.display());
    }

    if !force {
        eprint!("Remove {} and all worktrees? [y/N] ", repo_dir.display());
        io::stderr().flush().ok();

        let mut answer = String::new();
        io::stdin().lock().read_line(&mut answer)?;
        let yes = matches!(answer.trim().to_ascii_lowercase().as_str(), "y" | "yes");
        if !yes {
            eprintln!("Aborted.");
            return Ok(());
        }
    }

    std::fs::remove_dir_all(&repo_dir)
        .with_context(|| format!("failed to remove {}", repo_dir.display()))?;
    println!("Removed {}", repo_dir.display());
    Ok(())
}
