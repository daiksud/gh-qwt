//! `gh qwt prune` — remove worktrees whose branch is gone from the remote.
//!
//! Modeled on real git's own "prune" vocabulary (`git worktree prune`,
//! `git fetch --prune`): a safe, automatic cleanup of *stale* things, not a
//! way to delete an entire repository. Use `remove`/`rm` for that.
//!
//! See `docs/development/specification/README.md` (`prune`).

use anyhow::{Context, Result};
use std::io::{self, BufRead, Write};
use std::path::PathBuf;

use crate::{git, repo};

/// Arguments for `gh qwt prune`.
#[derive(Debug, clap::Args)]
pub struct Args {
    /// Skip confirmation and remove the pruned worktrees and branches.
    #[arg(short = 'y', long = "force")]
    pub force: bool,
}

/// A worktree whose branch is gone from `origin` and is safe to prune.
struct Candidate {
    branch: String,
    path: PathBuf,
}

/// Run the `prune` command.
pub fn run(args: Args) -> Result<()> {
    let current_dir = std::env::current_dir().context("failed to get current directory")?;
    let repo_dir = repo::discover_repo_root(&current_dir)?;

    let default_branch = git::default_branch_from_head(&repo_dir)?;

    eprintln!("Fetching origin...");
    git::fetch_prune(&repo_dir)?;
    git::worktree_prune(&repo_dir)?;

    let worktrees = git::worktree_list(&repo_dir)?;

    let mut candidates = Vec::new();
    let mut skipped_dirty = Vec::new();

    for worktree in worktrees {
        // Detached HEAD: no branch to check against the remote, never a candidate.
        let Some(branch) = worktree.branch else {
            continue;
        };
        if branch == default_branch {
            continue; // never prune the default branch
        }
        if !git::branch_has_upstream(&repo_dir, &branch)? {
            continue; // never had a remote counterpart: local-only work, never touch
        }
        if git::remote_branch_exists(&repo_dir, &branch)? {
            continue; // still on the remote: keep
        }

        if git::worktree_is_dirty(&worktree.path)? {
            skipped_dirty.push(branch);
        } else {
            candidates.push(Candidate {
                branch,
                path: worktree.path,
            });
        }
    }

    for branch in &skipped_dirty {
        eprintln!("warning: skipping '{branch}': worktree has uncommitted changes");
    }

    if candidates.is_empty() {
        println!("Nothing to prune.");
        return Ok(());
    }

    if !args.force {
        eprintln!("The following worktrees are no longer on the remote and will be removed:");
        for candidate in &candidates {
            eprintln!("  {}", candidate.branch);
        }
        eprint!("Remove these worktrees and their local branches? [y/N] ");
        io::stderr().flush().ok();

        let mut answer = String::new();
        io::stdin().lock().read_line(&mut answer)?;
        let yes = matches!(answer.trim().to_ascii_lowercase().as_str(), "y" | "yes");
        if !yes {
            eprintln!("Aborted.");
            return Ok(());
        }
    }

    for candidate in &candidates {
        // Already verified clean above, so this should never hit the
        // dirty-worktree safety check that a plain `worktree remove` has.
        git::worktree_remove(&repo_dir, &candidate.path, false)?;
        git::branch_delete(&repo_dir, &candidate.branch)?;
        println!("Removed {}", candidate.branch);
    }

    Ok(())
}
