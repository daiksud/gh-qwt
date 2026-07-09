//! `gh qwt path` — print a worktree path (for `cd`).
//!
//! See `docs/development/specification/README.md` (`path`).

use anyhow::Result;

/// Arguments for `gh qwt path`.
#[derive(Debug, clap::Args)]
pub struct Args {
    /// `owner/repo` or `owner/repo/branch`; omitted prints the qwt root.
    #[arg(value_name = "SPEC")]
    pub spec: Option<String>,
}

/// Run the `path` command.
pub fn run(args: Args) -> Result<()> {
    let _ = args;
    // TODO(cmd-path): implement per specification.
    anyhow::bail!("`path` is not yet implemented")
}
