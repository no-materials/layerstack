---
id: lay-c2o2
status: open
deps: [lay-y8q1, lay-a4vo]
links: []
created: 2026-03-13T18:08:32Z
type: feature
priority: 3
assignee: Bruce Mitchener
parent: lay-uc3n
tags: [schemas, spec-alignment]
---
# CollectionAPI

Implement CollectionAPI per §15. Collections allow grouping prims and properties via include/exclude path expressions. A collection is defined by includes and excludes membership expressions, an expansion rule (expandPrims or explicitOnly), and membership queries. Collections are authored as API schema instances on prims and can reference other collections.

## Acceptance Criteria

Collections can be authored with include/exclude expressions. Membership queries return correct results. expandPrims mode includes descendant prims. Collections can reference other collections.

