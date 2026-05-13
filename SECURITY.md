# Security Policy

Sentinel Forge handles security-sensitive code, research artifacts, and eventually hostile smart contract inputs. Please report vulnerabilities privately and avoid publishing exploitable details before maintainers have assessed the issue.

## In scope

- Rust engine crates under `engines/`
- report generation and exported finding formats
- example fixtures and vulnerable-contract samples
- dashboard or landing page paths that process user-provided content
- CI and local tooling that execute or inspect untrusted inputs

## Preferred reporting channels

Use one of these private channels in order of preference:

1. GitHub Security Advisories for this repository, if enabled
2. A private maintainer contact published in the repository or organization profile
3. A minimal public issue requesting a secure contact channel without disclosing exploit details

## What to include

- affected component or file path
- impact summary
- reproduction steps
- proof of concept only when required to validate the issue
- assumptions, environment details, and any suspected root cause

## Severity classes

| Severity | Description |
| --- | --- |
| Critical | Remote code execution, sandbox escape, or analysis output that could directly enable exploitation at scale |
| High | Major integrity failure, severe false assurance, or bypass of a core security guarantee |
| Medium | Meaningful security weakness with limited blast radius or practical constraints |
| Low | Hardening issue, information leak, or niche misuse case with low direct impact |

## Disclosure process

1. Acknowledge receipt within 3 business days when possible.
2. Triage severity and affected scope.
3. Reproduce and prepare a fix or mitigation plan.
4. Coordinate disclosure timing with the reporter when the issue is real and material.
5. Publish a summary after remediation when public disclosure is appropriate.

## Expectations for researchers

- do not exfiltrate data
- do not access accounts or systems beyond what is necessary to demonstrate the issue
- do not publish working exploit details before coordinated disclosure
- do not open public issues containing live exploit paths

## Project-specific cautions

Sentinel Forge aims to analyze adversarial code. That means some future features will parse, execute, or simulate hostile artifacts. When reporting issues related to exploit replay, fuzzing, or symbolic execution, note whether the weakness could affect local developer machines, CI workers, or shared infrastructure.

For additional detail, see [docs/security/threat-model.md](docs/security/threat-model.md) and [docs/security/disclosure-policy.md](docs/security/disclosure-policy.md).
