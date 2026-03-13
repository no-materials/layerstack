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

