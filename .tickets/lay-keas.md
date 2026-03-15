---
id: lay-keas
status: open
deps: []
links: []
created: 2026-03-13T18:04:17Z
type: epic
priority: 1
assignee: Bruce Mitchener
tags: [composition, spec-alignment]
---
# Composition Arcs: Complete LIVERPS

Implement the remaining composition arc types to complete LIVERPS (§10.3). Currently Local, Inherits, VariantSets, and References are implemented. Missing: Relocates, Payloads, and Specializes. Each arc type has its own strength ordering position and namespace mapping semantics. ~120 upstream conformance fixtures are blocked on these.

## Design

Follow existing arc implementation patterns in compose.rs and population.rs. Each arc needs: population expansion, opinion collection, namespace remapping, and cycle detection. Payloads are closest to references (deferred loading variant). Specializes mirrors inherits but at weaker strength. Relocates is the most complex (path remapping across arcs).

## Acceptance Criteria

All upstream composition fixtures for payloads, specializes, and relocates pass. LIVERPS ordering fully enforced per §10.4.


## Notes

**2026-03-14T01:57:11Z**

Variant children filtering + variant opinion forwarding committed (81701d8). VariantSets arc now correctly: parses variant selections/ordering/branches, filters children to selected branches, forwards variant opinions through references. 4 new conformance tests passing: TrickyVariantAncestralSelection, BasicSpecializesAndVariants, TrickyVariantWeakerSelection3, TrickyVariantInPayload.

**2026-03-15T18:21:42Z**

Incremental recomposition (lay-cr9t) closed: LiveStage with InvalidationGraph, lazy propagation, scoped recompose via PopulationMask. Commits 58d43aa, 9c49c95, d41eba6.
