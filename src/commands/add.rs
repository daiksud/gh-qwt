//! `gh qwt add` — create a worktree for a new or existing branch.
//!
//! See `docs/development/specification/README.md` (`add`).

use crate::{config, git, repo};
use anyhow::{bail, Context, Result};

/// Arguments for `gh qwt add`.
#[derive(Debug, clap::Args)]
pub struct Args {
    /// Branch to create a worktree for (may contain `/`).
    #[arg(value_name = "BRANCH")]
    pub branch: String,

    /// Operate on this repository instead of discovering from the current directory.
    #[arg(long, value_name = "OWNER/REPO")]
    pub repo: Option<String>,

    /// Base ref for a new branch when no matching remote branch exists.
    #[arg(long, value_name = "REF")]
    pub from: Option<String>,
}

/// Run the `add` command.
pub fn run(args: Args) -> Result<()> {
    let Args { branch, repo, from } = args;

    let repo_dir = if let Some(repo_arg) = repo {
        let spec =
            repo::RepoSpec::parse(&repo_arg, "github.com").map_err(crate::error::usage_from)?;
        let root = config::resolve_root()?;
        let repo_dir = repo::repo_dir(&root, &spec.owner, &spec.repo);

        if !repo::is_qwt_repo(&repo_dir) {
            bail!("not a qwt repository: {}", repo_dir.display());
        }

        repo_dir
    } else {
        let cwd = std::env::current_dir().context("failed to determine current directory")?;
        repo::discover_repo_root(&cwd)?
    };

    let dest = repo::worktree_in(&repo_dir, &branch);
    if dest
        .try_exists()
        .with_context(|| format!("failed to inspect {}", dest.display()))?
    {
        bail!("worktree already exists: {}", dest.display());
    }

    repo::check_prefix_collision(&repo_dir, &branch)?;

    if git::local_branch_exists(&repo_dir, &branch)? {
        git::worktree_add_existing(&repo_dir, &branch, &dest)?;
    } else if git::remote_branch_exists(&repo_dir, &branch)? {
        git::worktree_add_tracking(&repo_dir, &branch, &dest)?;
    } else {
        let base = match from {
            Some(base) => base,
            None => git::default_branch_from_head(&repo_dir)?,
        };
        git::worktree_add_new(&repo_dir, &branch, &dest, &base)?;
    }

    println!("{}", dest.display());
    Ok(())
}
