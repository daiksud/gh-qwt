---
type: adr
title: "ADR-0002: Distribute as a gh CLI extension"
description: "Distribute gh-qwt as a GitHub CLI extension."
resource: gh-qwt
tags: [gh-qwt, adr, adr-0002]
timestamp: 2026-07-09
---

# ADR-0002: Distribute as a gh CLI extension

- **Status:** Accepted
- **Date:** 2026-07-09
- **Deciders:** gh-qwt maintainers
- **Related:** [ADR-0003](../0003-language-rust-precompiled-binary/), [ADR-0010](../0010-release-and-distribution/)

## Context

gh-qwt is inherently GitHub-centric: it resolves GitHub repositories, shells out to `gh` and `git`, and optimizes the workflow of people who already use the GitHub CLI. The target users are therefore likely to have `gh` installed, authenticated, and included in their daily terminal workflow.

GitHub CLI extensions provide a familiar discovery and installation path through `gh extension install`, reuse `gh` authentication, and give commands a natural namespace under `gh`. Shipping as an extension also avoids inventing a separate command-discovery story for a tool whose primary purpose is GitHub repository worktree management.

## Decision

We will distribute gh-qwt as a GitHub CLI extension named `gh-qwt`, invoked by users as `gh qwt`. The extension will rely on the `gh` extension model for installation, upgrade, command dispatch, and authentication context.

## Consequences

### Positive
- Users install and upgrade the tool with standard `gh extension` commands.
- The command name communicates that the tool is part of a GitHub CLI workflow.
- gh-qwt can reuse the user’s existing `gh` authentication and host configuration.

### Negative
- The project is constrained by GitHub CLI extension conventions, including repository naming and release asset expectations.
- Users who do not use `gh` must install it before using gh-qwt.

### Neutral
- gh-qwt remains a separate executable even though it is invoked through `gh qwt`.

## Alternatives considered

| Alternative | Why not chosen |
| --- | --- |
| Standalone binary with separate distribution | This would require a separate install and upgrade story and would not benefit from `gh` authentication or extension discovery. |
| Shell plugin | Shell-specific installation would fragment the experience and make cross-shell behavior harder to support. |
| Forking or patching ghq | ghq is a general repository organizer; gh-qwt needs GitHub-specific behavior and per-branch worktrees without changing ghq itself. |

## References

- GitHub CLI manual: `gh extension install`
- [ADR-0010: Release and distribution](../0010-release-and-distribution/)
