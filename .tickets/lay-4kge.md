---
id: lay-4kge
status: in_progress
deps: []
links: []
created: 2026-03-13T18:05:13Z
type: feature
priority: 1
assignee: Bruce Mitchener
parent: lay-keas
tags: [composition, spec-alignment]
---
# Implement Payloads composition arc

Implement the Payloads composition arc (§10.3). Payloads are structurally identical to references but support deferred loading — the consumer can choose whether to 'load' a payload or not. When loaded, they behave like references with the same namespace mapping. Their position in LIVERPS is between References and Specializes (weaker than references). The ArcKind::Payloads enum variant already exists but is not wired into compose.rs or population.rs.

## Design

Mirror the reference implementation path: add payload expansion in population.rs, opinion collection in compose.rs (add_payload_opinions analogous to add_reference_opinions), and respect the load/unload state via a population mask or stage option. The Reference struct can likely be reused since payloads have the same fields (layer, prim_path, asset) minus customData. Layer offsets apply to payloads per §12.3.2.1.

## Acceptance Criteria

BasicPayload_root, BasicNestedPayload_root, BasicPayloadDiamond_root, PayloadsAndAncestralArcs_root (and variants) conformance fixtures pass. Payloads correctly positioned in LIVERPS strength ordering.

