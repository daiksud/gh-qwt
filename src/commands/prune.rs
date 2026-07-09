//! `gh qwt prune` — remove an entire repository tree.
//!
//! See `docs/development/specification/README.md` (`prune`).

use anyhow::Result;

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
    let _ = args;
    // TODO(cmd-prune): implement per specification.
    anyhow::bail!("`prune` is not yet implemented")
}
