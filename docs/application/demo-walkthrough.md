# Demo Walkthrough

This walkthrough turns the current repository into a repeatable local demo.

## 1. Analyzer output

Run the analyzer against the vulnerable fixtures:

```bash
cargo run -p static-analyzer --bin sentinel-forge -- scan examples/vulnerable-contracts --format html --output reports/demo-vulnerabilities.html
```

What to highlight:

- multiple detector classes fire from one shared finding model
- evidence, severity, and remediation are preserved across formats
- the examples tree is organized for vulnerability, secure, benchmark, and exploit-oriented work

## 2. CI posture

Run the secure fixture in CI mode:

```bash
cargo run -p static-analyzer --bin sentinel-forge -- ci examples/secure-contracts/guarded_admin.rs
```

What to highlight:

- the analyzer can operate as a build gate
- secure fixtures double as regression protection

## 3. Dashboard surface

Run:

```bash
corepack pnpm --filter @sentinel-forge/dashboard dev
```

What to highlight:

- severity distribution
- detector concentration
- findings table tied to evidence
- code-linked review surface

## 4. Exploit-lab surface

Run:

```bash
corepack pnpm --filter @sentinel-forge/exploit-lab dev
```

What to highlight:

- attack-path graph
- replay timeline
- state transition snapshots
- how findings connect to exploit reasoning

## 5. Landing page

Run:

```bash
corepack pnpm --filter @sentinel-forge/landing dev
```

What to highlight:

- positioning
- architecture narrative
- contributor and demo readiness
- roadmap through later engine phases
