---
id: lay-a4vo
status: open
deps: []
links: []
created: 2026-03-13T18:08:45Z
type: feature
priority: 3
assignee: Bruce Mitchener
tags: [composition, data-model, spec-alignment]
---
# Path expressions

Implement path expressions as a data type and in composition. Path expressions (§16.3.10.14, introduced in crate v0.10.0) allow pattern-based matching of paths, used in CollectionAPI and payload/reference expressions. ExpressionsInPayloads_root and ExpressionsInReferences_root conformance fixtures test this. Path expressions share representation with AssetPath in the binary format.

## Acceptance Criteria

Path expressions can be parsed and evaluated. ExpressionsInPayloads_root and ExpressionsInReferences_root fixtures pass.

