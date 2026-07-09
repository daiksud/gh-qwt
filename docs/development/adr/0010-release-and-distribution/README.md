---
type: adr
title: "ADR-0010: Publish precompiled release binaries for gh extension install"
description: "Publish precompiled release binaries via GitHub Releases."
resource: gh-qwt
tags: [gh-qwt, adr, adr-0010]
timestamp: 2026-07-09
---

# ADR-0010: Publish precompiled release binaries for gh extension install

- **Status:** Accepted
- **Date:** 2026-07-09
- **Deciders:** gh-qwt maintainers
- **Related:** [ADR-0002](../0002-distribute-as-gh-cli-extension/), [ADR-0003](../0003-language-rust-precompiled-binary/)

## Context

GitHub CLI can install precompiled extensions when releases contain correctly named platform assets. gh-qwt is implemented as a Rust binary, so users should not need a Rust toolchain just to install it. Release automation should produce predictable artifacts for common platforms and let `gh` select the right binary.

The project also needs a straightforward local development path. Contributors should be able to build with Cargo and install the working tree as an extension without waiting for a release. Linux artifacts should be broadly usable, so static musl builds are preferred where practical.

## Decision

We will publish per-platform precompiled binaries on GitHub Releases named `gh-qwt-<os>-<arch>[.exe]`. A tag-triggered GitHub Actions matrix will build the assets, using musl for static Linux builds. Architecture names will map Rust targets to GitHub CLI asset conventions, including `x86_64` to `amd64` and `aarch64` to `arm64`. Users will install releases with `gh extension install daiksud/gh-qwt`. Local development will use `cargo build` followed by `gh extension install .`.

## Consequences

### Positive
- Users can install gh-qwt through the standard GitHub CLI extension workflow.
- Users do not need Rust, Cargo, or source builds for normal installation.
- Release assets follow the naming convention `gh` expects for platform selection.

### Negative
- Release automation must maintain a platform matrix and asset naming rules.
- Each supported platform increases CI time and release validation work.

### Neutral
- Source builds remain available for contributors and unsupported platforms.

## Alternatives considered

| Alternative | Why not chosen |
| --- | --- |
| Interpreted or Bash extension with no build | This would simplify releases but lose the single robust Rust binary chosen for the project. |
| Require users to `cargo install` | This would force users to install and maintain a Rust toolchain and would not match normal `gh extension install` expectations. |

## References

- GitHub CLI documentation for precompiled extensions
- Rust target naming conventions
