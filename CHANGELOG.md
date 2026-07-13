# Changelog

All notable changes to this project are documented in this file.

The format loosely follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this
project adheres to [Semantic Versioning](https://semver.org/). This file is a lightweight,
human-readable summary; the canonical, detailed history for each release is the auto-generated
notes on [GitHub Releases](https://github.com/daiksud/gh-qwt/releases) (see
[Release notes](docs/development/building-and-releasing/#release-notes)).

## [Unreleased]

## [0.12.0] - 2026-07-13

### Fixed

- `gh qwt prune` and `gh qwt remove`/`rm` now remove empty parent directories left by
  slash-named worktrees without failing when repository and worktree paths use different symlinked,
  canonicalized, or Windows verbatim-path representations.

## [0.11.0] - 2026-07-10

### Changed

- **Breaking:** `gh qwt prune` no longer takes an `owner/repo` argument and no longer deletes an
  entire repository. It now discovers the repository from the current directory (like
  `git worktree prune`) and removes only worktrees whose branch has an upstream that is gone from
  `origin` — after refreshing remote-tracking refs with `git fetch origin --prune` and cleaning up
  stale worktree metadata with `git worktree prune`. A branch that was never pushed, has
  uncommitted changes, or is the repository's default branch is never touched.
- **Breaking:** `gh qwt rm` is now an alias for a new `gh qwt remove` command. Removing a single
  worktree from inside a repository is unchanged. Removing an entire repository (the old `prune
  <owner>/<repo>` behavior) is now `gh qwt remove <owner>/<repo>` / `gh qwt rm <owner>/<repo>`, run
  from outside any qwt repository. Use manual `rm -rf "$(gh qwt path owner/repo)"` if this
  particular relocation causes a problem for an existing script during migration.

### Added

- `gh qwt remove`/`rm` also accepts an explicit `owner/repo/branch` spec to remove a single
  worktree in a specific repository without first `cd`-ing into it.

## [0.10.0] - 2026-07-10

### Changed

- **Breaking:** `gh qwt list`'s default output is now a flat, sorted list of `owner/repo/branch`
  (one entry per line, no repository header lines or indentation), matching the shape `ghq list`
  uses. Previously, `list` grouped worktrees under an indented repository header.

### Added

- `gh qwt list` accepts an optional `<query>` argument for substring filtering (case-insensitive
  unless the query contains an uppercase letter — smartcase) and a `-e`/`--exact` flag for exact
  matching against `branch`, `repo/branch`, or `owner/repo/branch` — mirroring `ghq list`'s query
  semantics.

### Fixed

- The fuzzy worktree picker recipe in the shell integration guide (`gh qwt list -p | fzf`) now
  works correctly end to end. Previously, `list`'s grouped/indented output made selected lines
  unusable for `cd`: repository header lines had no path, and worktree lines had leading
  whitespace.

## [0.9.0] - 2026-07-09

Initial release.

### Added

- Core `gh qwt` command set (v1): `get`, `add`, `list`, `rm`, `root`, `path`, and `prune`.
- Bare-repository-plus-per-branch-worktree layout under `<qwt_root>/<owner>/<repo>/<branch>`,
  with no host segment in the path.
- `qwt.root` configuration resolution order: `QWT_ROOT`, `git config --get qwt.root`, then
  `~/qwt`.
- Default-branch detection via `gh api`, falling back to `git ls-remote --symref`.
- Shell integration guidance for `cd`-ing into worktrees.
- Precompiled release binaries for `darwin-arm64`, `linux-amd64`, `linux-arm64`, and
  `windows-amd64`, distributed via GitHub Releases for `gh extension install`.

[Unreleased]: https://github.com/daiksud/gh-qwt/compare/v0.12.0...HEAD
[0.12.0]: https://github.com/daiksud/gh-qwt/compare/v0.11.0...v0.12.0
[0.11.0]: https://github.com/daiksud/gh-qwt/compare/v0.10.0...v0.11.0
[0.10.0]: https://github.com/daiksud/gh-qwt/compare/v0.9.0...v0.10.0
[0.9.0]: https://github.com/daiksud/gh-qwt/releases/tag/v0.9.0
