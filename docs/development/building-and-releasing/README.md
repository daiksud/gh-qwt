# Building and releasing

Developer notes for building `gh-qwt`, installing it locally as `gh qwt`, and publishing precompiled GitHub CLI extension releases.

## Table of contents

- [Prerequisites](#prerequisites)
- [Local build and install](#local-build-and-install)
- [Precompiled release model](#precompiled-release-model)
- [Cross-compilation notes](#cross-compilation-notes)
- [Example release workflow](#example-release-workflow)
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

`gh-qwt` is planned as a Rust precompiled binary extension for GitHub CLI. During local development, build the release binary, copy it to the repository root as `gh-qwt`, then install the extension from the local checkout.

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
| `x86_64-apple-darwin` | `darwin` | `amd64` | `gh-qwt-darwin-amd64` |
| `aarch64-apple-darwin` | `darwin` | `arm64` | `gh-qwt-darwin-arm64` |
| `x86_64-unknown-linux-musl` | `linux` | `amd64` | `gh-qwt-linux-amd64` |
| `aarch64-unknown-linux-musl` | `linux` | `arm64` | `gh-qwt-linux-arm64` |
| `x86_64-pc-windows-msvc` | `windows` | `amd64` | `gh-qwt-windows-amd64.exe` |

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
- Build both `x86_64-apple-darwin` and `aarch64-apple-darwin` for macOS users on Intel and Apple Silicon.
- Build `x86_64-pc-windows-msvc` for Windows users and keep the `.exe` extension in the release asset name.
- Linux ARM64 can be built with a suitable linker/toolchain or with [`cross`](https://github.com/cross-rs/cross).

Example target installation:

```bash
rustup target add x86_64-apple-darwin
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
            target: x86_64-apple-darwin
            asset_name: gh-qwt-darwin-amd64
            binary_name: gh-qwt
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
        uses: actions/checkout@v4

      - name: Install Rust toolchain
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ matrix.target }}

      - name: Install musl tools
        if: runner.os == 'Linux'
        run: sudo apt-get update && sudo apt-get install -y musl-tools

      - name: Build
        run: cargo build --release --target '${{ matrix.target }}'

      - name: Package asset
        shell: bash
        run: |
          mkdir -p dist
          cp 'target/${{ matrix.target }}/release/${{ matrix.binary_name }}' 'dist/${{ matrix.asset_name }}'

      - name: Upload artifact
        uses: actions/upload-artifact@v4
        with:
          name: ${{ matrix.asset_name }}
          path: dist/${{ matrix.asset_name }}
          if-no-files-found: error

  release:
    name: Create GitHub Release
    needs: build
    runs-on: ubuntu-latest
    steps:
      - name: Download assets
        uses: actions/download-artifact@v4
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

## Release checklist

- [ ] Update the crate version in `Cargo.toml`.
- [ ] Confirm user-facing docs match the released behavior.
- [ ] Create an annotated tag such as `v0.1.0`.
- [ ] Push the tag to trigger the release workflow.
- [ ] Verify release assets use the required `gh-qwt-<os>-<arch>[.exe]` names.
- [ ] Test published installation with `gh extension install daiksud/gh-qwt`.
- [ ] Test upgrade behavior with `gh extension upgrade qwt`.
