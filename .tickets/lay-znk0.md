---
id: lay-znk0
status: open
deps: [lay-y8q1]
links: []
created: 2026-03-13T18:08:39Z
type: feature
priority: 3
assignee: Bruce Mitchener
parent: lay-uc3n
tags: [schemas, spec-alignment]
---
# ColorSpaceAPI

Implement ColorSpaceAPI per §14. Provides color space metadata for scene-referred colors. Supports canonical color spaces (§14.1): ACEScg, ACES2065-1, Linear Rec.709, sRGB, P3-D65, Rec.2020, Adobe RGB, CIE XYZ-D65, and various encoded variants. Color space can be specified per-attribute or per-asset. Includes ColorSpaceDefinitionAPI (§14.3) for custom color space definitions and ColorSpaceAPI (§14.4) for color space assignment.

## Acceptance Criteria

Color space metadata can be authored and resolved on attributes and assets. Canonical color spaces from §14.1 are recognized. Custom color space definitions work via ColorSpaceDefinitionAPI.

