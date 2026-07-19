---
type: index
title: "gh-qwt documentation"
description: "Documentation for using gh qwt, a GitHub CLI extension that clones each repository as a bare git database with one worktree directory per branch."
resource: gh-qwt
tags: [gh-qwt, documentation, index]
timestamp: 2026-07-15
template: splash
hero:
  title: "gh-qwt"
  tagline: "One clone. One worktree per branch. A calmer way to work with GitHub repositories."
  image:
    html: |
      <svg viewBox="0 0 420 320" role="img" aria-label="A repository branching into multiple worktrees" xmlns="http://www.w3.org/2000/svg">
        <defs><linearGradient id="hero-green" x1="70" y1="35" x2="350" y2="285" gradientUnits="userSpaceOnUse"><stop stop-color="#2da44e"/><stop offset="1" stop-color="#1a7f37"/></linearGradient></defs>
        <rect x="28" y="18" width="364" height="284" rx="42" fill="url(#hero-green)" opacity=".12"/>
        <rect x="55" y="44" width="136" height="58" rx="14" fill="#1f2328"/><path d="M76 66h23m-23 14h82" stroke="#fff" stroke-width="7" stroke-linecap="round" opacity=".9"/>
        <path d="M123 102v106c0 26 21 47 47 47h67M123 149h115c32 0 58 26 58 58v48" fill="none" stroke="#1a7f37" stroke-width="13" stroke-linecap="round" stroke-linejoin="round"/>
        <circle cx="123" cy="149" r="17" fill="#fff" stroke="#1a7f37" stroke-width="10"/><circle cx="237" cy="255" r="17" fill="#fff" stroke="#1a7f37" stroke-width="10"/><circle cx="296" cy="255" r="17" fill="#fff" stroke="#1a7f37" stroke-width="10"/>
      </svg>
  actions:
    - text: "Get started"
      link: "/gh-qwt/guides/getting-started/"
      icon: right-arrow
      variant: primary
    - text: "CLI reference"
      link: "/gh-qwt/references/cli/"
      icon: document
      variant: secondary
---

# gh-qwt documentation

Documentation for **using** `gh qwt` — a GitHub CLI extension that clones each repository once as a
**bare** git database and gives every branch its own **worktree** directory
(`<qwt_root>/<owner>/<repo>/<branch>`), so you can work on many branches side by side.

> [!NOTE]
> **GitHub only.** Unlike [ghq](https://github.com/x-motemen/ghq) (which supports many hosts and
> version-control systems), `gh-qwt` is built specifically for GitHub — it is a GitHub CLI (`gh`)
> extension and uses `gh` for authentication and GitHub API access.

> [!WARNING]
> **Pre-1.0 software.** `gh-qwt` is in the v0 series. Its CLI behavior, standard output, shell
> integration, and documentation may change incompatibly before 1.0.

## Install

```console
$ gh extension install daiksud/gh-qwt
```

## Explore the docs

<div class="docs-card-grid">
  <a class="docs-card" href="guides/getting-started/">
    <strong>Getting started</strong>
    <span>Install gh-qwt, clone your first repository, and enter a worktree.</span>
    <span class="docs-card-arrow">Start here →</span>
  </a>
  <a class="docs-card" href="guides/working-with-worktrees/">
    <strong>Working with worktrees</strong>
    <span>Create, inspect, and remove branch worktrees without disrupting your current work.</span>
    <span class="docs-card-arrow">Read the guide →</span>
  </a>
  <a class="docs-card" href="guides/configuration/">
    <strong>Configuration</strong>
    <span>Choose where repositories live with QWT_ROOT and qwt.root.</span>
    <span class="docs-card-arrow">Configure gh-qwt →</span>
  </a>
  <a class="docs-card" href="guides/shell-integration/">
    <strong>Shell integration</strong>
    <span>Jump directly into worktrees with qcd and your favorite fuzzy finder.</span>
    <span class="docs-card-arrow">Set up your shell →</span>
  </a>
  <a class="docs-card" href="references/cli/">
    <strong>CLI reference</strong>
    <span>Look up every command, argument, flag, output contract, and exit code.</span>
    <span class="docs-card-arrow">Browse the reference →</span>
  </a>
  <a class="docs-card" href="development/">
    <strong>Development</strong>
    <span>Explore the architecture, specification, release process, tests, and ADRs.</span>
    <span class="docs-card-arrow">Contribute →</span>
  </a>
</div>
