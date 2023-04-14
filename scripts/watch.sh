#!/usr/bin/env bash

set -e
set -o pipefail

# go to repo's root directory
cd "$(dirname $0)/.."

# requires cargo-watch be installed via:
#        cargo install cargo-watch
cargo watch --clear --exec run --why
