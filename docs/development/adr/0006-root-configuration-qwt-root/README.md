# ADR-0006: Resolve root from QWT_ROOT, qwt.root, then ~/qwt

- **Status:** Accepted
- **Date:** 2026-07-09
- **Deciders:** gh-qwt maintainers
- **Related:** [ADR-0005](../0005-path-layout-without-host-segment/), [ADR-0007](../0007-command-set-v1/)

## Context

gh-qwt needs a root directory under which it stores owners, repositories, bare databases, and branch worktrees. Users familiar with ghq expect a configurable root and often use environment variables or Git config for this kind of tool. However, gh-qwt’s path layout is intentionally not the same as ghq’s layout.

Reusing `GHQ_ROOT` or `ghq.root` would appear convenient, but it would mix incompatible directory structures. A user’s ghq tree may contain host segments and normal clones, while gh-qwt stores owner/repo directories containing a hidden bare database and branch worktrees. The tool needs a familiar configuration pattern without clobbering or confusing ghq-managed repositories.

## Decision

We will resolve the qwt root in this order: the `QWT_ROOT` environment variable, then `git config --get qwt.root`, then the default `~/qwt`. This configuration is independent from ghq’s `GHQ_ROOT` and `ghq.root` settings.

## Consequences

### Positive
- Users can override the root per process with `QWT_ROOT`.
- Persistent configuration is available through standard Git config.
- gh-qwt avoids mixing its worktree layout with existing ghq directories.

### Negative
- Users who already configured ghq must configure gh-qwt separately if they want a non-default root.
- Documentation must clearly explain the difference from ghq settings.

### Neutral
- The `root` command can report the resolved value without changing how it is configured.

## Alternatives considered

| Alternative | Why not chosen |
| --- | --- |
| Reuse `GHQ_ROOT` or `ghq.root` | ghq and gh-qwt use incompatible layouts; sharing a root would risk confusing tools and users. |

## References

- `git config` documentation
- [ADR-0005: Path layout without a host segment](../0005-path-layout-without-host-segment/)
