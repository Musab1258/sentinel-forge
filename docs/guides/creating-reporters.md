# Creating Reporters

Reporter plugins turn structured findings into formats that humans or tools can consume.

## Reporter responsibilities

- read findings
- preserve severity, confidence, and evidence
- format output for a specific audience

## Reporter should not

- perform analysis
- reinterpret detector semantics
- invent missing evidence

## Current reporter surface

The analyzer currently renders one `AnalysisReport` into:

- text
- JSON
- SARIF
- HTML

The report and finding types live in:

- `engines/static-analyzer/src/reporting/mod.rs`
- `packages/shared/src/analysis.ts`

## Target outputs

- terminal-friendly CLI output
- JSON for automation
- SARIF for code scanning
- HTML or dashboard serialization

## Implementation approach

1. Start from the shared finding model instead of detector internals.
2. Preserve source locations, severity, confidence, evidence, and remediation.
3. Keep output deterministic so CI or UI consumers can trust diffs.
4. Add empty-state behavior and verify it intentionally.

## Design checklist

- does the format preserve source locations
- can a consumer distinguish severity from confidence
- is remediation guidance retained
- is the output stable enough for automation

## Reporter-specific review questions

- does this format change force a shared type change
- can existing consumers ignore the new data safely
- are large finding sets still readable or streamable
- does the output make unsupported coverage obvious enough

## Testing expectations

- snapshot or fixture-based output tests
- validation that required fields are not dropped
- clear behavior for empty finding sets
