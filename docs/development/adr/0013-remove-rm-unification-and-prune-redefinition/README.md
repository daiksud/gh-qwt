---
type: adr
title: "ADR-0013: Unify `rm` into `remove`; redefine `prune` around real git's own \"prune\" semantics"
description: "Merge rm and prune's whole-repository removal into a new remove command (rm becomes its alias); redefine prune as a safe, automatic cleanup of worktrees whose branch is gone from the remote."
resource: gh-qwt
tags: [gh-qwt, adr, adr-0013]
timestamp: 2026-07-10
---

# ADR-0013: Unify `rm` into `remove`; redefine `prune` around real git's own "prune" semantics

- **Status:** Accepted
- **Date:** 2026-07-10
- **Deciders:** gh-qwt maintainers
- **Related:** [ADR-0004](../0004-bare-repo-plus-per-branch-worktree-layout/), [ADR-0007](../0007-command-set-v1/)

## Context

`gh qwt prune <owner>/<repo>` deleted an entire repository directory (the `.bare` database and
every worktree) after one confirmation. `gh qwt rm <branch>` removed a single worktree, discovered
from the current directory. Neither name matched what real git uses "prune" for: `git worktree
prune`, `git fetch --prune`, `git remote prune`, and `git gc --prune` are all *safe*, automatic
cleanups of stale or dangling things -- none of them ever deletes an entire live repository or an
intentionally created branch. Naming gh-qwt's most destructive operation "prune" was a mismatch
with that vocabulary from the start.

Separately, users following a typical GitHub squash-merge workflow (merge a PR, GitHub deletes the
source branch) wanted a way to clean up the now-stale local worktree automatically, in the spirit
of `git fetch --prune` -- keep a worktree if its branch is still on the remote, remove it if the
remote branch is gone. ADR-0007 anticipated exactly this kind of evolution ("command behavior can
evolve through later ADRs if v1 usage reveals missing primitives").

## Decision

**`prune` no longer takes an argument.** It discovers the repository from the current directory
(the same mechanism `add` already uses), matching real `git worktree prune`'s invocation (it always
operates on "here"). It:

1. Runs `git fetch origin --prune` to refresh remote-tracking refs and drop the ones the remote no
   longer has.
2. Runs `git worktree prune` to clean up administrative metadata for worktree directories that were
   removed outside of `gh qwt` (for example, by hand with `rm -rf`).
3. For every remaining worktree except the default branch and any detached `HEAD`, removes the
   worktree and its local branch **only if** `branch.<name>.remote` is configured (it *had* an
   upstream) **and** `origin/<branch>` no longer exists after the fetch, **and** the worktree has no
   uncommitted or untracked changes. It lists the candidates and asks for confirmation once, unless
   `-y`/`--force`.

Two safety rules were validated against real git behavior in a sandbox before being finalized:

- `branch.<name>.remote` is set by a tracking checkout and, unlike the remote-tracking ref itself,
  is **not** removed by `fetch --prune`. Checking it *before* checking whether `origin/<branch>`
  exists reliably distinguishes "this was tracking a remote branch that's now gone" from "this was
  never pushed at all" -- the latter must never be auto-removed.
- No ancestor/"is this actually merged" check is performed against the default branch. This
  project's own workflow (and GitHub's squash-merge generally) means a legitimately merged branch's
  commits are **not** ancestors of the squash commit on the target branch, so an ancestor check
  would incorrectly flag safely-cleaned-up branches as unsafe. "Gone from origin after a fresh
  fetch" is used directly as the merge signal.

**`rm` and `prune`'s old whole-repository-removal job are unified into a new `remove` command,**
with `rm` becoming a `visible_alias` for it (one implementation, two invocable names):

- `remove <branch>` -- discovered from the current directory, removes only that worktree. Identical
  to the original `rm`.
- `remove <owner>/<repo>` -- resolved explicitly (not discovered), removes the entire repository.
  This is the original `prune`'s job, relocated here.
- `remove <owner>/<repo>/<branch>` -- resolved explicitly, removes only that worktree. New: this
  did not exist before (the original `rm` only worked from inside the repository).

Because a branch name can itself contain `/`, a bare two-segment string like `fix/parser` is
inherently ambiguous between "a branch name" and "`owner=fix, repo=parser`." This is resolved by
context, not by counting segments: if the current directory is inside a qwt-managed repository, the
whole argument is always a branch name in *that* repository (matching the original `rm` exactly, no
matter how many `/` the branch has); otherwise it is parsed as an explicit `owner/repo[/branch]`
spec, the same way `path` already parses its argument.

**Known limitation, accepted:** there is no way to target a *different* repository's worktree while
standing inside another repository's worktree -- you would `cd` out first (for example, to the qwt
root). A `--repo <owner>/<repo>` escape hatch (mirroring `add --repo`) would resolve this but was
not requested and is left for a future ADR if needed.

## Consequences

### Positive
- `prune`'s name now means what it means everywhere else in git: a safe, automatic cleanup, never a
  destructive wholesale delete. Users familiar with `git fetch --prune`/`git worktree prune` get the
  behavior they expect.
- The common `rm <branch>` workflow is completely unchanged (same discovery, same flags).
- `remove <owner>/<repo>/<branch>` is a genuinely new capability: removing a single worktree in a
  specific repository without first `cd`-ing into it.
- The remote-gone signal (not an ancestor check) works correctly under this project's own
  squash-merge-and-delete-branch workflow, where an ancestor check would not.

### Negative
- Breaking change: `prune <owner>/<repo>` (delete a whole repository by name, from anywhere) no
  longer exists under that name; it is now `remove <owner>/<repo>` / `rm <owner>/<repo>`, and only
  works when *not* standing inside another qwt repository.
- `prune` now depends on the remote being reachable (`git fetch`); it is no longer a purely local,
  offline-safe operation the way the old whole-repo delete was (fetching from a `file://` remote in
  tests is still fully offline, so this does not affect this project's own test suite).
- The cwd-vs-explicit disambiguation rule for `remove`/`rm` must be learned; it is not discoverable
  from the argument shape alone.

### Neutral
- `get`, `add`, `list`, `root`, and `path` are unchanged. This ADR does not supersede ADR-0007's
  overall v1 command set, only evolves two commands within it, as ADR-0007 itself anticipated.

## Alternatives considered

| Alternative | Why not chosen |
| --- | --- |
| Keep `prune <owner>/<repo>` deleting the whole repository, add the remote-aware cleanup as a new command (for example `gc`) | Leaves gh-qwt's most destructive operation permanently misnamed relative to every real git use of "prune," and adds a third destructive/cleanup verb instead of consolidating to two clear ones. |
| Require an ancestor/"is merged" check before `prune` removes a candidate | Breaks under squash-merge (and rebase-merge): the branch's commits are not ancestors of the squash commit even though the branch was legitimately merged and deleted upstream, which is exactly this project's own workflow. |
| `remove`/`rm` always requires an explicit `owner/repo[/branch]` spec (no cwd discovery) | Regresses the common case: today's `rm <branch>` from inside a worktree is the most frequent invocation and needs no repository name. |
| Add a `--repo` flag to `remove`/`rm` now, to resolve the cross-repo-while-inside-another-repo gap | Not requested; adds surface area beyond what was asked. Left as a documented, explicit limitation for a future ADR if it turns out to matter in practice. |

## References

- `git-worktree(1)`: `git worktree prune`
- `git-fetch(1)`: `--prune`
- [ADR-0007: v1 command set](../0007-command-set-v1/)
- [ADR-0012: Flat, query-filterable `list` output](../0012-flat-queryable-list-output/) (the prior
  ADR in this same "command behavior evolves" spirit)
