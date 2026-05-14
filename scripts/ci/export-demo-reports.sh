#!/usr/bin/env bash

set -euo pipefail

mkdir -p reports

cargo run -p static-analyzer --bin sentinel-forge -- scan examples/vulnerable-contracts --format json --output reports/demo-vulnerabilities.json
cargo run -p static-analyzer --bin sentinel-forge -- scan examples/vulnerable-contracts --format html --output reports/demo-vulnerabilities.html
