//! `gh qwt root` — print the resolved qwt root.
//!
//! See `docs/development/specification/README.md` (`root`).

use anyhow::Result;

/// Arguments for `gh qwt root`.
#[derive(Debug, clap::Args)]
pub struct Args {}

/// Run the `root` command.
pub fn run(args: Args) -> Result<()> {
    let _ = args;
    // TODO(cmd-path): implement per specification (root printing).
    anyhow::bail!("`root` is not yet implemented")
}
