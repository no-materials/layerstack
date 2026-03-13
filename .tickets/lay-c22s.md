---
id: lay-c22s
status: open
deps: []
links: []
created: 2026-03-13T18:04:39Z
type: epic
priority: 2
assignee: Bruce Mitchener
tags: [data-model, spec-alignment]
---
# Data Model: Complete Type System

Expand the Value enum and data model to cover all foundational types in §6 and document model fields in §7. Currently Value has Bool/Int(i64)/Float(f64)/String/Token/Opaque. Missing: full numeric types (half, uint, int64, uint64, uchar), dimensioned types (vec2/3/4, matrix2/3/4d, quaternions), semantic aliases (color, normal, point, vector, texCoord, frame), dictionary type with combining semantics (§6.6.2.1), specifier/typeName/variability fields, and the full set of core metadata fields.

## Design

The Value enum should expand to cover §6.2-6.5 types. Consider a separate DimensionedValue or use the Opaque variant with typed wrappers. Dictionary combining (§6.6.2.1) needs its own implementation for value resolution. Specifier (def/over/class) affects population (§12.2.1). Keep no_std compatible.

## Acceptance Criteria

All §6 scalar and dimensioned types representable. Dictionary combining produces correct results per §6.6.2.1. Specifier resolution per §12.2.1 works correctly.

