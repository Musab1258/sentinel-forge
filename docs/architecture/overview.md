# Architecture Overview

Sentinel Forge is structured as a hybrid monorepo:

- `apps/landing` explains the product, research posture, and contributor path
- `engines/*` holds the Rust-based analysis systems
- `packages/*` provides shared TypeScript configuration and future reusable code
- `docs/*` keeps architecture, security, and research material close to the implementation

The first implemented engine is the static analyzer, because the research points to authorization, state mutation, arithmetic, and denial-of-service checks as the highest-leverage initial detector set.

