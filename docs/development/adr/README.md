# Architecture Decision Records

This directory records the significant architecture decisions for `gh-qwt` using lightweight
[ADRs](https://cognitect.com/blog/2011/11/15/documenting-architecture-decisions), in the style
popularized by Michael Nygard.

Each ADR is an immutable record of one decision: its **context**, the **decision** itself, and the
**consequences**. When a decision changes, we don't rewrite history — we add a new ADR and mark the
old one `Superseded`.

## How to add an ADR

1. Copy [`template/`](template/) to a new directory `NNNN-short-slug/` (next number, kebab-case).
2. Fill in `README.md`.
3. Add a row to the [index](#index) below.
4. If it replaces an earlier decision, set that ADR's status to `Superseded by ADR-NNNN`.

## Statuses

| Status | Meaning |
| --- | --- |
| `Proposed` | Under discussion; not yet in effect |
| `Accepted` | Decided and in effect |
| `Deprecated` | No longer recommended, but not replaced |
| `Superseded` | Replaced by a later ADR (linked) |

## Index

| # | Decision | Status |
| --- | --- | --- |
| [0001](0001-record-architecture-decisions/) | Record architecture decisions | Accepted |
| [0002](0002-distribute-as-gh-cli-extension/) | Distribute as a `gh` CLI extension | Accepted |
| [0003](0003-language-rust-precompiled-binary/) | Implement in Rust as a precompiled binary | Accepted |
| [0004](0004-bare-repo-plus-per-branch-worktree-layout/) | Bare repo + per-branch worktree layout | Accepted |
| [0005](0005-path-layout-without-host-segment/) | Path layout without a host segment | Accepted |
| [0006](0006-root-configuration-qwt-root/) | Root config: `QWT_ROOT` → `qwt.root` → `~/qwt` | Accepted |
| [0007](0007-command-set-v1/) | v1 command set | Accepted |
| [0008](0008-shell-integration-for-cd/) | Shell integration for `cd` | Accepted |
| [0009](0009-default-branch-detection-strategy/) | Default-branch detection strategy | Accepted |
| [0010](0010-release-and-distribution/) | Release & distribution | Accepted |
| [0011](0011-conventional-commits-and-release-notes/) | Conventional Commits & automated release notes | Accepted |
