# Plugin Registry

This directory is the local foundation for phase 8 plugin support.

## Purpose

- define a stable home for detector and reporter manifests
- make third-party extension points concrete before runtime loading exists
- provide future dashboard and SDK consumers a shared registry shape

## Current contents

- `registry.json`: machine-readable list of known plugin manifests
- `detectors/`: detector manifest examples
- `reporters/`: reporter manifest examples
