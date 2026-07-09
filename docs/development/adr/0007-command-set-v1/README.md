---
type: adr
title: "ADR-0007: Ship the v1 command set"
description: "Ship the v1 command set: get, add, list, rm, root, path, prune."
resource: gh-qwt
tags: [gh-qwt, adr, adr-0007]
timestamp: 2026-07-09
---

# ADR-0007: Ship the v1 command set

- **Status:** Accepted
- **Date:** 2026-07-09
- **Deciders:** gh-qwt maintainers
- **Related:** [ADR-0004](../0004-bare-repo-plus-per-branch-worktree-layout/), [ADR-0006](../0006-root-configuration-qwt-root/), [ADR-0008](../0008-shell-integration-for-cd/), [ADR-0009](../0009-default-branch-detection-strategy/)

## Context

gh-qwt needs a small command surface that proves the core workflow without becoming a general repository manager. The essential workflow is cloning a GitHub repository once, creating a default-branch worktree, and adding more branch worktrees over time. Users also need ways to inspect paths, list managed worktrees, remove them, and clean up stale Git worktree metadata.

The command set should feel familiar to ghq users where that familiarity helps, but gh-qwt’s distinguishing feature is branch-oriented worktree management. Some useful ideas, such as multiple roots, broader non-GitHub host support, and a built-in fuzzy picker, can be deferred until the core model is stable.

## Decision

We will ship v1 with the commands `get`, `add`, `list`, `rm`, `root`, `path`, and `prune`. `get` will create or prepare the repository layout and default-branch worktree. `add` will create additional branch worktrees. The remaining commands provide ghq-like discovery, path reporting, lifecycle management, and Git worktree pruning.

## Consequences

### Positive
- The v1 surface covers the complete branch worktree lifecycle.
- Users get enough convenience commands to script and navigate the layout.
- Deferred features do not block the core workflow.

### Negative
- Users wanting multiple roots, richer host support, or an integrated picker must compose gh-qwt with other tools or wait for later versions.
- Each command adds documentation and test surface.

### Neutral
- Command behavior can evolve through later ADRs if v1 usage reveals missing primitives.

## Alternatives considered

| Alternative | Why not chosen |
| --- | --- |
| Minimal `get` and `add` only | This would prove the core but leave users without basic listing, path lookup, removal, and cleanup commands. |
| Larger command surface | More commands would slow v1 and risk committing to workflows before the core layout is validated. |

## References

- ghq command concepts
- `git worktree prune` documentation
