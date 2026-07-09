#!/usr/bin/env bash
set -euo pipefail

cargo build --release
cp target/release/gh-qwt ./gh-qwt
