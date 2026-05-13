# Soroban Security Model Notes

These notes capture the phase 0 research themes that most directly affect Sentinel Forge design.

## Key properties

- deterministic execution
- explicit authorization flows
- persistent contract state
- WebAssembly-based runtime
- resource metering

## Why they matter

### Deterministic execution

Helps reproducibility and makes security findings easier to confirm.

### Authorization flows

Likely the highest-value early detector category because mistakes here can lead directly to privileged state changes.

### Persistent state

State transition reasoning is central. A contract can be logically wrong even when arithmetic and parsing look safe.

### WASM runtime

Source analysis alone will not be enough forever. Low-level modeling and cross-checking are likely necessary for stronger guarantees.

### Resource metering

Denial-of-service and execution-cost concerns should be part of the security model, not treated as separate performance work.
