---
id: lay-nqrf
status: open
deps: []
links: []
created: 2026-03-13T18:05:40Z
type: feature
priority: 2
assignee: Bruce Mitchener
parent: lay-keas
tags: [composition, population, spec-alignment]
---
# Implement Instancing

Implement prim instancing per §11. Instancing allows prims marked as 'instanceable' to share composed scene graph subtrees, reducing memory and computation. Instanced prims with identical composition structure share a 'prototype' prim. Instancing interacts with composition arcs — the presence of arcs affects whether two prims can share a prototype.

## Design

Instancing requires: (1) instanceable metadata field on prim specs, (2) prototype identification — prims with identical arc structure share prototypes, (3) stage population must create prototype prims and wire instance prims to them, (4) value resolution through instances delegates to the prototype. This is largely a population/stage concern, not a composition arc itself.

## Acceptance Criteria

BasicInstancing_root, BasicInstancingAndNestedInstances_root, BasicInstancingAndVariants_root conformance fixtures pass.

