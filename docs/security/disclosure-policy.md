# Disclosure Policy

This document expands the top-level security policy with a coordinated disclosure process tailored to Sentinel Forge.

## Goals

- protect users and contributors from avoidable exposure
- preserve trust in the project
- make it easy for researchers to report issues responsibly

## Reporting guidance

Please send:

- affected version or branch
- component details
- reproduction steps
- impact assessment
- any suggested fix or mitigation

Do not publish exploit details in a public issue before maintainers respond.

## Triage priorities

- sandbox escape or arbitrary code execution
- findings that provide dangerous false assurance
- parser or analyzer bypasses that meaningfully reduce coverage
- exposure of secrets, tokens, or sensitive CI material

## Coordination targets

- acknowledge the report within 3 business days when possible
- confirm triage direction within 7 business days when possible
- coordinate publication timing after a fix or mitigation exists

## Public disclosure

When a report is resolved, maintainers should publish a summary that includes:

- the affected area
- the risk level
- mitigation or fix guidance
- credit to the reporter when appropriate
