---
id: lay-7kxf
status: open
deps: []
links: []
created: 2026-03-13T18:06:00Z
type: feature
priority: 1
assignee: Bruce Mitchener
parent: lay-avtk
tags: [value-resolution, spec-alignment]
---
# Value blocking

Implement value blocking semantics (§12.3). When an attribute spec's default value is set to the block sentinel (None in USDA, ValueBlock type in binary), weaker opinions must be skipped entirely and the fallback value returned instead. This is distinct from an unauthored field (which simply has no opinion). Blocking applies to both default values and individual time samples. The ValueBlock type is defined in §16.3.10.16.

## Design

Add a ValueBlock variant to the Value enum (or a blocked flag on FieldValue). During value resolution, when a blocked value is encountered in the opinion stack, stop iterating weaker opinions and return the fallback value. For time samples, individual time codes can be blocked independently.

## Acceptance Criteria

An attribute with 'None' value on the strongest layer resolves to fallback, not to weaker authored values. Blocked time samples at specific time codes resolve to fallback at those times.

