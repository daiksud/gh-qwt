//! `gh qwt path` — print a worktree path (for `cd`).
//!
//! See `docs/development/specification/README.md` (`path`).

use anyhow::Result;

use crate::{config, repo};

/// Arguments for `gh qwt path`.
#[derive(Debug, clap::Args)]
pub struct Args {
    /// `owner/repo` or `owner/repo/branch`; omitted prints the qwt root.
    #[arg(value_name = "SPEC")]
    pub spec: Option<String>,
}

/// Run the `path` command.
pub fn run(args: Args) -> Result<()> {
    let root = config::resolve_root()?;

    let path = match args.spec {
        None => root,
        Some(spec) => {
            let segments: Vec<&str> = spec
                .split('/')
                .filter(|segment| !segment.is_empty())
                .collect();

            match segments.as_slice() {
                [owner, repo] => repo::repo_dir(&root, owner, repo),
                [owner, repo, branch @ ..] => {
                    repo::worktree_path(&root, owner, repo, &branch.join("/"))
                }
                _ => {
                    return Err(crate::error::usage(
                        "path argument must be owner/repo or owner/repo/branch",
                    ));
                }
            }
        }
    };

    println!("{}", path.display());
    Ok(())
}
