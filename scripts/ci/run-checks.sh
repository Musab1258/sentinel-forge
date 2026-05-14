#!/usr/bin/env bash

set -euo pipefail

corepack pnpm --filter @sentinel-forge/landing test
cargo test --workspace
cargo run -p static-analyzer --bin sentinel-forge -- ci examples/secure-contracts/guarded_admin.rs
cargo run -p static-analyzer --bin sentinel-forge -- scan examples/vulnerable-contracts --format json --output /tmp/sentinel-forge-vulnerabilities.json
