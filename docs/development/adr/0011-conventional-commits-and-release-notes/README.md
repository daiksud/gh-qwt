---
type: adr
title: "ADR-0011: Adopt Conventional Commits and GitHub automated release notes"
description: "Adopt Conventional Commits and GitHub automated release notes."
resource: gh-qwt
tags: [gh-qwt, adr, adr-0011]
timestamp: 2026-07-09
---

# ADR-0011: Adopt Conventional Commits and GitHub automated release notes

- **Status:** Accepted
- **Date:** 2026-07-09
- **Deciders:** gh-qwt maintainers
- **Related:** [ADR-0002](../0002-distribute-as-gh-cli-extension/), [ADR-0010](../0010-release-and-distribution/)

## Context

Releases are published on GitHub Releases (see [ADR-0010](../0010-release-and-distribution/)). We
want a **consistent commit history** and **low-effort, categorized release notes** without adopting
a heavyweight changelog toolchain.

GitHub can generate release notes automatically when creating a release, and the grouping of those
notes can be configured with a `.github/release.yml` file. GitHub groups entries by **pull request
label**, not by commit message content.

## Decision

We will adopt the following conventions:

1. **Commit messages MUST follow [Conventional Commits 1.0.0](https://www.conventionalcommits.org/en/v1.0.0/)** — `<type>[scope][!]: <description>`, with `feat`/`fix` reserved for product changes and a `!`/`BREAKING CHANGE:` footer for breaking changes.
2. **Releases use GitHub [automatically generated release notes](https://docs.github.com/en/repositories/releasing-projects-on-github/automatically-generated-release-notes)** — the release workflow passes `--generate-notes` to `gh release create`.
3. **Release notes use exactly four categories**, configured in `.github/release.yml` and driven by pull request labels:

   | Category | Pull request label |
   | --- | --- |
   | BREAKING CHANGE | `breaking-change` |
   | New Features | `enhancement` |
   | Bug Fixes | `bug` |
   | Others | everything else (`*`) |

   Contributors label pull requests to match the Conventional Commit type (`feat` → `enhancement`, `fix` → `bug`, breaking → `breaking-change`).

## Consequences

### Positive
- Consistent, machine-readable history and predictable, categorized release notes.
- Native to GitHub — no extra changelog generator or parser to maintain.

### Negative
- Categorization depends on pull requests being **labeled** correctly (GitHub groups by label, not commit type), so labels and commit types must be kept in sync.
- Requires a non-default `breaking-change` label in the repository.

### Neutral
- Squash-merge titles should also follow Conventional Commits so the default entry text stays clean.

## Alternatives considered

| Alternative | Why not chosen |
| --- | --- |
| Free-form commits + hand-written changelog | More effort per release and inconsistent history. |
| Commit-parsing tools (release-please, git-cliff, semantic-release) | Extra tooling and configuration; GitHub's built-in notes cover the need. |
| Categorize notes by commit type directly | GitHub's generated notes group by PR label, not commit message type. |

## References

- [Conventional Commits 1.0.0](https://www.conventionalcommits.org/en/v1.0.0/)
- [Automatically generated release notes](https://docs.github.com/en/repositories/releasing-projects-on-github/automatically-generated-release-notes)
- [ADR-0010: Release & distribution](../0010-release-and-distribution/)
