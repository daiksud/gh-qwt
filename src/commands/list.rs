//! `gh qwt list` — list repositories and their worktrees.
//!
//! See `docs/development/specification/README.md` (`list`).

use anyhow::Result;

/// Arguments for `gh qwt list`.
#[derive(Debug, clap::Args)]
pub struct Args {
    /// Print absolute paths for worktrees.
    #[arg(short = 'p', long = "full-path")]
    pub full_path: bool,
}

/// Run the `list` command.
pub fn run(args: Args) -> Result<()> {
    let _ = args;
    // TODO(cmd-list): implement per specification.
    anyhow::bail!("`list` is not yet implemented")
}
