mod commands;
mod config;
mod error;
mod gh;
mod git;
mod repo;

use clap::{Parser, Subcommand};

/// ghq reimagined around git worktrees — a GitHub CLI extension.
///
/// `gh qwt` clones each repository once as a bare git database and gives every
/// branch its own worktree directory under `<qwt_root>/<owner>/<repo>/<branch>`.
#[derive(Debug, Parser)]
#[command(name = "qwt", bin_name = "gh qwt", version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Clone (bare) and create the default-branch worktree.
    Get(commands::get::Args),
    /// Create a worktree for a new or existing branch.
    Add(commands::add::Args),
    /// List repositories and their worktrees.
    List(commands::list::Args),
    /// Remove a worktree, or an entire repository.
    #[command(visible_alias = "rm")]
    Remove(commands::remove::Args),
    /// Print the resolved qwt root.
    Root(commands::root::Args),
    /// Print a worktree path (for `cd`).
    Path(commands::path::Args),
    /// Remove worktrees whose branch is gone from the remote.
    Prune(commands::prune::Args),
}

fn main() {
    // clap exits with code 2 on usage/parse errors, matching the specification.
    let cli = Cli::parse();

    let result = match cli.command {
        Command::Get(args) => commands::get::run(args),
        Command::Add(args) => commands::add::run(args),
        Command::List(args) => commands::list::run(args),
        Command::Remove(args) => commands::remove::run(args),
        Command::Root(args) => commands::root::run(args),
        Command::Path(args) => commands::path::run(args),
        Command::Prune(args) => commands::prune::run(args),
    };

    if let Err(err) = result {
        // Invalid usage maps to exit code 2; all other runtime errors to 1.
        let code = if err.downcast_ref::<error::UsageError>().is_some() {
            2
        } else {
            1
        };
        eprintln!("gh-qwt: {err:#}");
        std::process::exit(code);
    }
}
