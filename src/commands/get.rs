//! `gh qwt get` — clone (bare) and create the default-branch worktree.
//!
//! See `docs/development/specification/README.md` (`get`).

use anyhow::Result;

/// Arguments for `gh qwt get`.
#[derive(Debug, clap::Args)]
pub struct Args {
    /// Repository: `owner/repo`, an HTTPS URL, or an SSH URL.
    #[arg(value_name = "REPO")]
    pub spec: String,

    /// Create a worktree for this branch instead of the default branch.
    #[arg(short = 'b', long = "branch", value_name = "BRANCH")]
    pub branch: Option<String>,

    /// Host to use when the repo spec is `owner/repo`.
    #[arg(long, value_name = "HOST", default_value = "github.com")]
    pub host: String,
}

/// Run the `get` command.
pub fn run(args: Args) -> Result<()> {
    let _ = args;
    // TODO(cmd-get): implement per specification.
    anyhow::bail!("`get` is not yet implemented")
}
