// TODO(integration): remove this crate-wide allow once every command wires the
// config/repo/git/gh modules together — it silences dead-code warnings that only
// exist while the modules are implemented incrementally in parallel.
#![allow(dead_code)]

mod commands;
mod config;
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
    /// Remove a worktree.
    Rm(commands::rm::Args),
    /// Print the resolved qwt root.
    Root(commands::root::Args),
    /// Print a worktree path (for `cd`).
    Path(commands::path::Args),
    /// Remove an entire repository tree.
    Prune(commands::prune::Args),
}

fn main() {
    // clap exits with code 2 on usage/parse errors, matching the specification.
    let cli = Cli::parse();

    let result = match cli.command {
        Command::Get(args) => commands::get::run(args),
        Command::Add(args) => commands::add::run(args),
        Command::List(args) => commands::list::run(args),
        Command::Rm(args) => commands::rm::run(args),
        Command::Root(args) => commands::root::run(args),
        Command::Path(args) => commands::path::run(args),
        Command::Prune(args) => commands::prune::run(args),
    };

    if let Err(err) = result {
        eprintln!("gh-qwt: {err:#}");
        std::process::exit(1);
    }
}
