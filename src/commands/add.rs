//! `gh qwt add` — create a worktree for a new or existing branch.
//!
//! See `docs/development/specification/README.md` (`add`).

use anyhow::Result;

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
    let _ = args;
    // TODO(cmd-add): implement per specification.
    anyhow::bail!("`add` is not yet implemented")
}
