---
type: guide
title: "Contributing"
description: "Development environment, code style, testing, documentation conventions, and commit and pull request rules."
resource: gh-qwt
tags: [gh-qwt, development, contributing]
timestamp: 2026-07-09
---

# Contributing

Guidelines for contributing code, tests, and documentation to `gh-qwt`.

## Table of contents

- [Development environment](#development-environment)
- [Project structure](#project-structure)
- [Code style](#code-style)
- [Testing](#testing)
- [Documentation conventions](#documentation-conventions)
- [Commit and pull request conventions](#commit-and-pull-request-conventions)

## Development environment

Install the core tools:

- Rust via [`rustup`](https://rustup.rs/)
- GitHub CLI, `gh`
- `git`

Install the Rust components used by this project:

```bash
rustup component add rustfmt clippy
```

Verify the toolchain:

```console
$ cargo --version
$ cargo fmt --version
$ cargo clippy --version
$ gh --version
$ git --version
```

## Project structure

See the [architecture documentation](../architecture/) for the broader design.

Planned source layout:

```text
Cargo.toml
src/main.rs
src/config.rs
src/repo.rs
src/git.rs
src/gh.rs
src/commands/get.rs
src/commands/add.rs
src/commands/list.rs
src/commands/rm.rs
src/commands/root.rs
src/commands/path.rs
src/commands/prune.rs
```

| Module | Responsibility |
| --- | --- |
| `src/main.rs` | CLI entry point, argument parsing, command dispatch. |
| `src/config.rs` | Configuration defaults and qwt root resolution. |
| `src/repo.rs` | Repository naming, paths, and bare repository/worktree layout rules. |
| `src/git.rs` | Shelling out to `git` and translating git failures into actionable errors. |
| `src/gh.rs` | Shelling out to `gh` for GitHub-specific lookups when needed. |
| `src/commands/get.rs` | Implementing `gh qwt get` for clone/bootstrap flows. |
| `src/commands/add.rs` | Implementing `gh qwt add` for adding branch worktrees. |
| `src/commands/list.rs` | Implementing `gh qwt list` for showing managed worktrees. |
| `src/commands/rm.rs` | Implementing `gh qwt rm` for removing worktrees. |
| `src/commands/root.rs` | Implementing `gh qwt root` for printing the qwt root. |
| `src/commands/path.rs` | Implementing `gh qwt path` for resolving repository/worktree paths. |
| `src/commands/prune.rs` | Implementing `gh qwt prune` for cleanup of stale worktree metadata. |

## Code style

Format before opening a pull request:

```bash
cargo fmt
```

Run Clippy with warnings denied:

```bash
cargo clippy -- -D warnings
```

Keep changes focused and prefer clear, explicit error messages. `gh-qwt` shells out to `gh` and `git`, so command failures should tell users what failed and what they can try next.

## Testing

See the [testing documentation](../testing/) for the unit and offline integration test strategy.

At minimum, run:

```bash
cargo test
```

For a quick end-to-end sanity check, run the offline smoke test:

```bash
script/smoke-test.sh
```

For style and lint checks, also run:

```bash
cargo fmt --check
cargo clippy -- -D warnings
```

## Documentation conventions

- Every documentation page is a directory containing `README.md`, for example `docs/development/testing/README.md`.
- Page assets go in an `assets/` directory next to that page's `README.md`.
- Use GitHub Flavored Markdown (GFM): tables, task lists, alerts, and fenced code blocks with language tags.
- Keep terminology consistent: `gh qwt`, qwt root, worktree, and bare repository.
- Keep docs in sync with code and the project specification.
- New architectural decisions should go through an ADR in [`../adr/`](../adr/). Copy [`../adr/template/`](../adr/template/) when starting a new decision record.

> [!TIP]
> Prefer relative links so documentation works in the repository, on GitHub, and in local previews.

## Commit and pull request conventions

### Commit messages

Commit messages **MUST** follow [Conventional Commits 1.0.0](https://www.conventionalcommits.org/en/v1.0.0/):

```text
<type>[optional scope][!]: <description>

[optional body]

[optional footer(s)]
```

- Common `type`s: `feat`, `fix`, `docs`, `refactor`, `perf`, `test`, `build`, `ci`, `chore`.
- Use `feat` and `fix` **only** for changes to the product itself. For tooling, CI, or docs, use `build`, `ci`, `docs`, `chore`, and similar.
- Mark breaking changes with a `!` after the type/scope (for example `feat!:`) and/or a `BREAKING CHANGE:` footer.
- Write the description in the imperative mood, e.g. `feat: add get command`.

### Pull requests

- Keep pull requests small and focused.
- Label each pull request so [release notes](../building-and-releasing/#release-notes) categorize correctly:
  - `enhancement` for features (`feat`)
  - `bug` for fixes (`fix`)
  - `breaking-change` for breaking changes
- Reference the relevant specification or documentation section in the pull request body.
- Include tests for behavior changes.
- Update documentation in the same pull request when behavior, commands, or release steps change.
- Avoid unrelated cleanup in feature or fix pull requests.

Pull request checklist:

- [ ] The change is focused and easy to review.
- [ ] Commit messages follow [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/).
- [ ] The pull request is labeled by change type (`enhancement`, `bug`, or `breaking-change`) for release notes.
- [ ] `cargo fmt --check` passes, when Rust code exists.
- [ ] `cargo clippy -- -D warnings` passes, when Rust code exists.
- [ ] `cargo test` passes, when tests exist.
- [ ] Documentation and examples match the implementation.
