---
id: lay-cr9t
status: closed
deps: []
links: []
created: 2026-03-15T18:21:29Z
type: feature
priority: 1
assignee: Bruce Mitchener
tags: [composition, incremental, invalidation]
---
# Incremental recomposition via InvalidationGraph

LiveStage supports scoped recomposition: when layer opinions change, only transitively affected prims are recomposed. InvalidationGraph is the single source of truth for dependency topology. Lazy propagation via InvalidationSet. Scoped compose with PopulationMask. Incremental edge updates via update_prim_edges.

## Acceptance Criteria

LiveStage matches full Stage::compose results after edits. Batch notifications work. No-op recompose returns empty. Arc dependencies propagate correctly through references and inherits.


## Notes

**2026-03-15T18:21:35Z**

Implemented in commits 58d43aa and 9c49c95. LiveStage with lazy propagation, DependencyMap removed in favor of direct InvalidationGraph ownership, comprehensive tests added.
