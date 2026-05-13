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

## Target outputs

- terminal-friendly CLI output
- JSON for automation
- SARIF for code scanning
- HTML or dashboard serialization

## Design checklist

- does the format preserve source locations
- can a consumer distinguish severity from confidence
- is remediation guidance retained
- is the output stable enough for automation

## Testing expectations

- snapshot or fixture-based output tests
- validation that required fields are not dropped
- clear behavior for empty finding sets
