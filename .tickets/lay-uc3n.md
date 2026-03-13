---
id: lay-uc3n
status: open
deps: []
links: []
created: 2026-03-13T18:04:59Z
type: epic
priority: 3
assignee: Bruce Mitchener
tags: [schemas, spec-alignment]
---
# Schemas

Implement the schema system per §13. Schemas define typed prim definitions (IsA) and API schemas (HasA) that provide fallback values, property definitions, and semantic classification. Includes core schema types (§13.4), ColorSpaceAPI (§14), and CollectionAPI (§15).

## Design

Schema definitions need a registry mapping schema type tokens to their property definitions and fallback values. IsA schemas define prim types with inheritance. HasA (API) schemas apply additional properties. Schema ordering rules (§13.3) govern how multiple schemas compose. This is downstream of both data model and composition work.

## Acceptance Criteria

Schema-defined fallback values are returned when no opinion is authored. IsA/HasA semantics correctly apply. Core schemas (Color, Collections) functional.

