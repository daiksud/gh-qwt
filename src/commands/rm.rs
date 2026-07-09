//! `gh qwt rm` — remove a worktree.
//!
//! See `docs/development/specification/README.md` (`rm`).

use anyhow::Result;

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
    let _ = args;
    // TODO(cmd-rm): implement per specification.
    anyhow::bail!("`rm` is not yet implemented")
}
