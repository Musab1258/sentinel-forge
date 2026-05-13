# WASM Research Notes

Sentinel Forge needs a WASM-aware roadmap because Soroban contracts compile into WebAssembly modules.

## Research takeaways

- WASM provides a stable low-level surface for deeper inspection
- control-flow and host-call modeling become easier once modules are normalized
- malformed modules and hostile artifacts must be treated as dangerous inputs

## Analysis opportunities

- instruction-level control-flow reconstruction
- host interaction tracing
- execution-cost and hotspot analysis
- cross-checking source assumptions against compiled behavior

## Risks

- overreliance on source-only reasoning
- unsafe parsing of malformed modules
- complicated output that loses source-level developer context

## Design implication

WASM analysis should complement, not replace, the source and AST lanes. The system needs both if it wants broad coverage and usable explanations.
