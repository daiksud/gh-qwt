//! `gh qwt rm` — remove a worktree.
//!
//! See `docs/development/specification/README.md` (`rm`).

use anyhow::{bail, Context, Result};

use crate::{git, repo};

/// Arguments for `gh qwt rm`.
#[derive(Debug, clap::Args)]
pub struct Args {
    /// Branch whose worktree should be removed (may contain `/`).
    #[arg(value_name = "BRANCH")]
    pub branch: String,

    /// Remove the worktree even when it has local changes.
    #[arg(long)]
    pub force: bool,

    /// Also delete the local branch with `git branch -D`.
    #[arg(long = "delete-branch")]
    pub delete_branch: bool,
}

/// Run the `rm` command.
pub fn run(args: Args) -> Result<()> {
    let current_dir = std::env::current_dir().context("failed to get current directory")?;
    let repo_dir = repo::discover_repo_root(&current_dir)?;
    let dest = repo::worktree_in(&repo_dir, &args.branch);

    if !dest.exists() {
        bail!(
            "no worktree for branch '{}' at {}",
            args.branch,
            dest.display()
        );
    }

    git::worktree_remove(&repo_dir, &dest, args.force)?;

    if args.delete_branch {
        git::branch_delete(&repo_dir, &args.branch)?;
    }

    println!("Removed worktree {}", dest.display());
    Ok(())
}
