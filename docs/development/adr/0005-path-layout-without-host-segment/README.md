# ADR-0005: Use path layout without a host segment

- **Status:** Accepted
- **Date:** 2026-07-09
- **Deciders:** gh-qwt maintainers
- **Related:** [ADR-0002](../0002-distribute-as-gh-cli-extension/), [ADR-0004](../0004-bare-repo-plus-per-branch-worktree-layout/), [ADR-0006](../0006-root-configuration-qwt-root/)

## Context

ghq commonly stores repositories as `<root>/<host>/<owner>/<repo>`, which works well for a multi-host repository manager. gh-qwt has a narrower focus: it is a GitHub CLI extension, invoked through `gh`, and optimized for GitHub repositories. In the common case, users want short paths they can type and scan quickly.

Adding a host segment would make every path longer while providing little value for the primary GitHub.com workflow. At the same time, enterprise hosts and cross-host collisions are real possibilities. For v1, the project values a simple path shape and accepts that trade-off.

## Decision

We will lay out repositories as `<qwt_root>/<owner>/<repo>/<branch>` with no host segment. Full URLs will still be accepted by `get`, but the storage path will be based on owner and repository name rather than host.

## Consequences

### Positive
- Paths are shorter and easier to type than ghq-style host-qualified paths.
- The layout matches the GitHub-centric scope of the tool.
- Branch worktree paths remain predictable under each repository directory.

### Negative
- Repositories with the same owner and name on different hosts can collide.
- Enterprise-host workflows may need a future layout extension or migration path.

### Neutral
- This decision is independent of the root location, which is configured separately.

## Alternatives considered

| Alternative | Why not chosen |
| --- | --- |
| Include host as ghq does | Host-qualified paths are more general, but they lengthen every path for a v1 tool focused on GitHub and the common single-host case. |

## References

- ghq path layout conventions
- [ADR-0006: Root configuration](../0006-root-configuration-qwt-root/)
