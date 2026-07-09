---
type: adr
title: "ADR-0009: Detect the default branch with gh api then ls-remote"
description: "Detect the default branch with gh api, falling back to git ls-remote."
resource: gh-qwt
tags: [gh-qwt, adr, adr-0009]
timestamp: 2026-07-09
---

# ADR-0009: Detect the default branch with gh api then ls-remote

- **Status:** Accepted
- **Date:** 2026-07-09
- **Deciders:** gh-qwt maintainers
- **Related:** [ADR-0002](../0002-distribute-as-gh-cli-extension/), [ADR-0004](../0004-bare-repo-plus-per-branch-worktree-layout/), [ADR-0007](../0007-command-set-v1/)

## Context

The `get` command must create an initial worktree for the repository’s default branch. Assuming `master` or `main` is not correct: repositories can rename defaults, use project-specific branch names, or be private. Because gh-qwt is a GitHub CLI extension, it can ask GitHub for the repository’s `default_branch` through `gh`, benefiting from the user’s existing authentication and host configuration.

At the same time, tests and some URL-based flows should not depend exclusively on the GitHub API. Git itself exposes the remote HEAD symref through `git ls-remote --symref`, which is useful as a fallback and for local/offline-style test fixtures.

## Decision

We will detect the default branch by first running `gh api repos/{owner}/{repo} -q .default_branch`. If that is unavailable or unsuitable for the input, we will fall back to `git ls-remote --symref <url> HEAD` and parse the HEAD target. `get` will use the detected branch when creating the default-branch worktree.

## Consequences

### Positive
- Private repositories and renamed default branches work through authenticated `gh` API access.
- URL and test scenarios can still use Git’s remote HEAD information.
- The behavior avoids hardcoded branch-name assumptions.

### Negative
- Default-branch detection depends on external commands and network access in normal use.
- The fallback parser must handle Git output carefully.

### Neutral
- After clone, Git remote metadata may still be updated separately from default-branch detection.

## Alternatives considered

| Alternative | Why not chosen |
| --- | --- |
| Hardcode `main` or `master` | Many repositories use renamed or custom default branches, and private repositories need auth-aware lookup. |
| Infer only from `origin/HEAD` after clone | This delays detection until after clone setup and may be less reliable if remote HEAD metadata is missing or stale. |

## References

- GitHub REST API repository `default_branch` field
- `git ls-remote --symref` documentation
