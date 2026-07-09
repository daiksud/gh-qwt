---
type: guide
title: "Building and releasing"
description: "Build gh-qwt locally, install it as a gh extension, cross-compile, and publish GitHub Releases."
resource: gh-qwt
tags: [gh-qwt, development, building-and-releasing]
timestamp: 2026-07-09
---

# Building and releasing

Developer notes for building `gh-qwt`, installing it locally as `gh qwt`, and publishing precompiled GitHub CLI extension releases.

## Table of contents

- [Prerequisites](#prerequisites)
- [Local build and install](#local-build-and-install)
- [Precompiled release model](#precompiled-release-model)
- [Cross-compilation notes](#cross-compilation-notes)
- [Example release workflow](#example-release-workflow)
- [GitHub Actions version pinning policy](#github-actions-version-pinning-policy)
- [Release notes](#release-notes)
- [Release checklist](#release-checklist)

## Prerequisites

Install these tools before working on releases:

- Rust toolchain via [`rustup`](https://rustup.rs/)
- GitHub CLI, `gh`
- `git`

```console
$ rustup --version
$ cargo --version
$ gh --version
$ git --version
```

## Local build and install

`gh-qwt` is a Rust precompiled binary extension for GitHub CLI. During local development, build the release binary, copy it to the repository root as `gh-qwt`, then install the extension from the local checkout.

```bash
cargo build --release
cp target/release/gh-qwt ./gh-qwt
gh extension install .
```

After installation, the command is available as:

```console
$ gh qwt --help
```

Iterate by rebuilding and replacing the root executable:

```bash
cargo build --release
cp target/release/gh-qwt ./gh-qwt
gh qwt --help
```

A local helper script can keep this repeatable:

```bash
#!/usr/bin/env bash
set -euo pipefail

cargo build --release
cp target/release/gh-qwt ./gh-qwt
```

Save that as `script/build.sh` and make it executable:

```bash
chmod +x script/build.sh
```

The generated local executable and Rust build output must not be committed:

```gitignore
/gh-qwt
/target
```

## Precompiled release model

`gh-qwt` is distributed exclusively through [**GitHub Releases**](https://docs.github.com/en/repositories/releasing-projects-on-github/about-releases): each pushed `v*` tag produces a GitHub Release with the precompiled per-platform binaries attached as assets, which GitHub CLI downloads at install time.

A GitHub CLI extension repository must be named `gh-<name>` and provide either:

1. an executable named the same as the repository at the repository root, or
2. precompiled binaries attached to GitHub Releases.

For this project, the repository is `gh-qwt`, and the executable is `gh-qwt`. Published users install it with:

```console
$ gh extension install daiksud/gh-qwt
```

Users upgrade it with:

```console
$ gh extension upgrade qwt
```

Release assets must end with a `-<OS>-<ARCH>` suffix. Windows assets also need `.exe`. GitHub CLI selects the matching asset for the user's operating system and architecture at install time.

| Rust target | OS | Arch | Asset name |
| --- | --- | --- | --- |
| `aarch64-apple-darwin` | `darwin` | `arm64` | `gh-qwt-darwin-arm64` |
| `x86_64-unknown-linux-musl` | `linux` | `amd64` | `gh-qwt-linux-amd64` |
| `aarch64-unknown-linux-musl` | `linux` | `arm64` | `gh-qwt-linux-arm64` |
| `x86_64-pc-windows-msvc` | `windows` | `amd64` | `gh-qwt-windows-amd64.exe` |

> [!NOTE]
> **Intel macOS (`darwin-amd64`) is not supported.** GitHub Actions is
> [retiring the `macos-13` (Intel) runner image](https://github.blog/changelog/2025-09-19-github-actions-macos-13-runner-image-is-closing-down/),
> so there is no hosted runner to build or smoke-test `x86_64-apple-darwin`. macOS releases target
> Apple Silicon (`aarch64-apple-darwin`) only.

Architecture mapping:

| Rust architecture | GitHub CLI asset architecture |
| --- | --- |
| `x86_64` | `amd64` |
| `aarch64` | `arm64` |
| `i686` | `386` |

OS mapping:

| Rust OS target fragment | GitHub CLI asset OS |
| --- | --- |
| `apple-darwin` | `darwin` |
| `unknown-linux-gnu` | `linux` |
| `unknown-linux-musl` | `linux` |
| `pc-windows-msvc` | `windows` |

## Cross-compilation notes

- Prefer `x86_64-unknown-linux-musl` and `aarch64-unknown-linux-musl` for Linux release binaries. `musl` produces static binaries and avoids glibc version compatibility issues.
- Build `aarch64-apple-darwin` for macOS users on Apple Silicon. Intel macOS (`x86_64-apple-darwin`) is not supported (see the note above).
- Build `x86_64-pc-windows-msvc` for Windows users and keep the `.exe` extension in the release asset name.
- Linux ARM64 can be built with a suitable linker/toolchain or with [`cross`](https://github.com/cross-rs/cross).

Example target installation:

```bash
rustup target add aarch64-apple-darwin
rustup target add x86_64-unknown-linux-musl
rustup target add aarch64-unknown-linux-musl
rustup target add x86_64-pc-windows-msvc
```

> [!NOTE]
> Cross-compilation support depends on the runner OS and installed linkers. If direct `cargo build --target ...` is not enough for a target, add the required system packages or use a purpose-built cross-compilation action/tool.

## Example release workflow

The planned workflow lives at `.github/workflows/release.yml` and creates release assets when a `v*` tag is pushed.

```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

permissions:
  contents: write

jobs:
  build:
    name: Build ${{ matrix.asset_name }}
    runs-on: ${{ matrix.os }}
    strategy:
      fail-fast: false
      matrix:
        include:
          - os: macos-latest
            target: aarch64-apple-darwin
            asset_name: gh-qwt-darwin-arm64
            binary_name: gh-qwt
          - os: ubuntu-latest
            target: x86_64-unknown-linux-musl
            asset_name: gh-qwt-linux-amd64
            binary_name: gh-qwt
          - os: ubuntu-latest
            target: aarch64-unknown-linux-musl
            asset_name: gh-qwt-linux-arm64
            binary_name: gh-qwt
          - os: windows-latest
            target: x86_64-pc-windows-msvc
            asset_name: gh-qwt-windows-amd64.exe
            binary_name: gh-qwt.exe

    steps:
      - name: Check out repository
        uses: actions/checkout@v7

      - name: Install Rust toolchain
        uses: dtolnay/rust-toolchain@4be7066ada62dd38de10e7b70166bc74ed198c30 # stable
        with:
          targets: ${{ matrix.target }}

      - name: Install musl tools
        if: runner.os == 'Linux'
        run: sudo apt-get update && sudo apt-get install -y musl-tools

      - name: Install aarch64 cross linker
        # ubuntu-latest runners are x86_64, so cross-compiling to
        # aarch64-unknown-linux-musl needs an aarch64-capable linker; the
        # host's default `cc` cannot link aarch64 objects (it fails with
        # "Relocations in generic ELF (EM: 183)").
        if: matrix.target == 'aarch64-unknown-linux-musl'
        run: sudo apt-get update && sudo apt-get install -y gcc-aarch64-linux-gnu

      - name: Build
        env:
          CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_LINKER: aarch64-linux-gnu-gcc
        run: cargo build --release --target '${{ matrix.target }}'

      - name: Package asset
        shell: bash
        run: |
          mkdir -p dist
          cp 'target/${{ matrix.target }}/release/${{ matrix.binary_name }}' 'dist/${{ matrix.asset_name }}'

      - name: Upload artifact
        uses: actions/upload-artifact@v7
        with:
          name: ${{ matrix.asset_name }}
          path: dist/${{ matrix.asset_name }}
          if-no-files-found: error

  release:
    name: Create GitHub Release
    needs: build
    runs-on: ubuntu-latest
    steps:
      - name: Check out repository
        uses: actions/checkout@v7

      - name: Download assets
        uses: actions/download-artifact@v8
        with:
          path: dist
          merge-multiple: true

      - name: Create release
        env:
          GH_TOKEN: ${{ github.token }}
        run: gh release create "$GITHUB_REF_NAME" ./dist/* --title "$GITHUB_REF_NAME" --generate-notes
```

> [!NOTE]
> The [`cli/gh-extension-precompile`](https://github.com/cli/gh-extension-precompile) action is another option for packaging precompiled GitHub CLI extension assets.

## GitHub Actions version pinning policy

Both `.github/workflows/ci.yml` and `.github/workflows/release.yml` pin third-party actions by
their full-length commit SHA, following GitHub's [secure use guidance for third-party
actions](https://docs.github.com/en/actions/reference/security/secure-use#using-third-party-actions):

- **GitHub official or Marketplace-Verified actions** (owner shows a "Verified creator" badge,
  such as everything under the [`actions`](https://github.com/actions) organization) are pinned by
  their **major version tag**, for example `actions/checkout@v7`. GitHub controls these tags
  directly, so they are trusted to move within a major version.
- **All other actions** are pinned to a **full-length commit SHA**, with the human-readable version
  as a trailing comment, for example `Swatinem/rust-cache@c19371144df3bb44fab255c43d04cbc2ab54d1c4 # v2.9.1`.
  A SHA cannot be silently repointed to different code the way a tag or branch can.
- `dtolnay/rust-toolchain` has no `vX.Y.Z` release tags; the ref itself selects the Rust toolchain
  (`@stable`, `@nightly`, `@1.89.0`, ...). It is pinned to the commit SHA at the tip of the
  `stable` branch, commented as `# stable`, which keeps installing the latest stable Rust while
  fixing the action code that runs.

When updating a pinned action, resolve the new tag to its commit SHA (for example with
`gh api repos/<owner>/<repo>/commits/<tag> --jq .sha`) and update both the workflow YAML and this
document together.

## Release notes

Releases use GitHub's [automatically generated release notes](https://docs.github.com/en/repositories/releasing-projects-on-github/automatically-generated-release-notes). The release workflow passes `--generate-notes` to `gh release create`, and the grouping is configured in `.github/release.yml`.

Notes are grouped into exactly four categories, driven by pull request labels:

| Category | Pull request label |
| --- | --- |
| BREAKING CHANGE | `breaking-change` |
| New Features | `enhancement` |
| Bug Fixes | `bug` |
| Others | everything else |

`.github/release.yml`:

```yaml
changelog:
  categories:
    - title: 💥 BREAKING CHANGE
      labels:
        - breaking-change
    - title: ✨ New Features
      labels:
        - enhancement
    - title: 🐛 Bug Fixes
      labels:
        - bug
    - title: 🧰 Others
      labels:
        - "*"
```

> [!IMPORTANT]
> GitHub groups each pull request by its **labels**, not by commit message type. Keep labels aligned
> with the [Conventional Commits](../contributing/#commit-messages) type: `feat` → `enhancement`,
> `fix` → `bug`, breaking changes → `breaking-change`. A pull request is placed in the first matching
> category, so `breaking-change` is listed first. The non-default `breaking-change` label must be
> created in the repository.

## Release checklist

- [ ] Update the crate version in `Cargo.toml`.
- [ ] Confirm user-facing docs match the released behavior.
- [ ] Create an annotated tag such as `v0.9.0`.
- [ ] Push the tag to trigger the release workflow.
- [ ] Verify release assets use the required `gh-qwt-<os>-<arch>[.exe]` names.
- [ ] Test published installation with `gh extension install daiksud/gh-qwt`.
- [ ] Test upgrade behavior with `gh extension upgrade qwt`.
- [ ] Confirm merged pull requests are labeled so release notes categorize correctly.
