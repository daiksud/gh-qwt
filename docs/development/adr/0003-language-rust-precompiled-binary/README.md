---
type: adr
title: "ADR-0003: Implement in Rust as a precompiled binary"
description: "Implement gh-qwt in Rust as a precompiled binary."
resource: gh-qwt
tags: [gh-qwt, adr, adr-0003]
timestamp: 2026-07-09
---

# ADR-0003: Implement in Rust as a precompiled binary

- **Status:** Accepted
- **Date:** 2026-07-09
- **Deciders:** gh-qwt maintainers
- **Related:** [ADR-0002](../0002-distribute-as-gh-cli-extension/), [ADR-0010](../0010-release-and-distribution/)

## Context

gh-qwt should feel fast, install cleanly, and avoid requiring a runtime language environment. Its core operations are filesystem manipulation, process execution, parsing command output, and cross-platform path handling. The project also needs predictable release artifacts for GitHub CLI’s precompiled extension flow.

The maintainers prefer Rust, and Rust is a good fit for a small command-line tool that should ship as a single binary. The main trade-off is that GitHub CLI’s Go ecosystem has `go-gh`, which provides conveniences for extension authors. gh-qwt can mitigate that loss by shelling out to `gh` for GitHub API operations and to `git` for repository operations.

## Decision

We will implement gh-qwt in Rust and ship precompiled binaries per supported platform as a `--precompiled=other`-style GitHub CLI extension. The binary will shell out to `gh` and `git` rather than embedding GitHub or Git client libraries such as `go-gh`.

## Consequences

### Positive
- Users receive a single fast executable with no interpreter runtime dependency.
- Rust gives strong typing and robust error handling for path-heavy command behavior.
- Precompiled artifacts support predictable GitHub CLI extension installation.

### Negative
- The project loses the direct conveniences of `go-gh` available to Go extensions.
- Release automation must build and publish binaries for each supported platform.

### Neutral
- `gh` and `git` remain runtime dependencies because gh-qwt delegates GitHub and Git operations to them.

## Alternatives considered

| Alternative | Why not chosen |
| --- | --- |
| Go with `go-gh` | `go-gh` is convenient, but the maintainers prefer Rust and want Rust’s ergonomics for this tool. |
| Bash script extension | Bash would be easy to start, but robust parsing, errors, tests, and cross-platform behavior would be harder; it would still require `git` and `gh` at runtime. |

## References

- GitHub CLI extension documentation for precompiled extensions
- [ADR-0010: Release and distribution](../0010-release-and-distribution/)
