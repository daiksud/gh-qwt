# ADR-0004: Use a bare repo plus per-branch worktree layout

- **Status:** Accepted
- **Date:** 2026-07-09
- **Deciders:** gh-qwt maintainers
- **Related:** [ADR-0005](../0005-path-layout-without-host-segment/), [ADR-0007](../0007-command-set-v1/), [ADR-0009](../0009-default-branch-detection-strategy/)

## Context

gh-qwt exists to make it easy to have many branches of the same GitHub repository checked out at the same time. Git already has the right primitive for this: one object database with multiple worktrees. The layout should keep branch directories easy to see and enter, while keeping the shared repository database out of the way.

A bare repository avoids giving one branch special status as the “main” checkout. Storing that bare database under a hidden directory keeps the repository root focused on branch worktree directories. For relocatability, the root-level `.git` should be a file with a relative `gitdir`, not an absolute path.

## Decision

We will clone each repository as a bare Git database at `<repo>/.bare`, add a `.git` file containing `gitdir: ./.bare`, and create one Git worktree per branch as sibling directories under the repository directory. During setup, we will ensure the bare repository has the fetch refspec `+refs/heads/*:refs/remotes/origin/*` so remote branch refs are populated correctly for worktree creation.

## Consequences

### Positive
- Multiple branches can be checked out concurrently without duplicate object databases.
- Branch directories are visible at the top level and easy to navigate.
- Relative `gitdir` metadata makes the layout more relocatable.

### Negative
- The layout is more specialized than a normal clone and may surprise users inspecting internals.
- Tooling that assumes a conventional working tree at the repository root may need adjustment.

### Neutral
- Git remains the source of truth for worktree state; gh-qwt manages the layout around it.

## Alternatives considered

| Alternative | Why not chosen |
| --- | --- |
| Normal clone plus manual `git worktree` directories under a subdirectory | One checkout would be privileged, and branch directories would be less cleanly presented. |
| Store the database as `.git` directory | A real `.git` directory at the repo root would clash conceptually with the branch worktree layout and be less clearly hidden from branch directories. |

## References

- `git worktree` documentation
- `git clone --bare` documentation
