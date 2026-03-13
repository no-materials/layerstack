---
id: lay-baje
status: open
deps: []
links: []
created: 2026-03-13T18:07:21Z
type: feature
priority: 3
assignee: Bruce Mitchener
parent: lay-c22s
tags: [data-model, spec-alignment]
---
# Implement semantic type aliases

Add semantic type aliases from §6.5 to the data model. Semantic aliases assign roles to underlying types without changing their representation: color3f/color4f (float3/4 with color role), normal3f (float3 with normal role), point3f (float3 with position role), vector3f (float3 with direction+length role), texCoord2f/3f (float2/3 with texture coordinate role), frame4d (matrix4d with transform role). Aliases 'agree' with their underlying types but are not equivalent (§6.5.1). Higher-level constructs (transforms, rendering) should respect the semantic role.

## Design

Semantic aliases can be represented as a (underlying_type, role) pair or as distinct Value variants. The 'agreement' concept (§6.5.1) means alias and underlying type can be composed together but are tracked separately for schema validation. The group type (opaque with group role) is a proxy for multiple values.

## Acceptance Criteria

Semantic aliases can be stored and retrieved with their role. Agreement checks work per §6.5.1. Type names round-trip correctly through file formats.

