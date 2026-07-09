//! `gh qwt prune` — remove an entire repository tree.
//!
//! See `docs/development/specification/README.md` (`prune`).

use crate::{config, repo};
use anyhow::{bail, Context, Result};
use std::io::{self, BufRead, Write};

/// Arguments for `gh qwt prune`.
#[derive(Debug, clap::Args)]
pub struct Args {
    /// Repository to remove: `owner/repo`.
    #[arg(value_name = "OWNER/REPO")]
    pub repo: String,

    /// Skip confirmation and remove the repository tree.
    #[arg(short = 'y', long = "force")]
    pub force: bool,
}

/// Run the `prune` command.
pub fn run(args: Args) -> Result<()> {
    let root = config::resolve_root()?;
    let spec = repo::RepoSpec::parse(&args.repo, "github.com").map_err(crate::error::usage_from)?;
    let repo_dir = repo::repo_dir(&root, &spec.owner, &spec.repo);

    if repo_dir == root || repo_dir.parent() == Some(root.as_path()) || !repo_dir.starts_with(&root)
    {
        bail!(
            "refusing to prune unsafe repository path: {}",
            repo_dir.display()
        );
    }

    if !repo::is_qwt_repo(&repo_dir) {
        bail!("not a qwt repository: {}", repo_dir.display());
    }

    if !args.force {
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
