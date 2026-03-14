---
id: lay-2b56
status: closed
deps: []
links: []
created: 2026-03-13T18:06:10Z
type: feature
priority: 1
assignee: Bruce Mitchener
parent: lay-avtk
tags: [value-resolution, spec-alignment]
---
# Interpolation methods (Held and Linear)

Implement the two basic interpolation methods for time-varying attributes (§12.5): Held interpolation (step function — value holds until the next time sample) and Linear interpolation (linearly interpolate between bracketing samples). The interpolation type is a stage-level or layer-level setting. Pre/post extrapolation behavior: before the first sample, return the first sample's value; after the last, return the last.

## Design

Interpolation is invoked during time-based value resolution when the query time falls between two authored time samples. For held: return the value at the earlier time sample. For linear: lerp between the two bracketing values. Linear interpolation requires numeric types; non-interpolable types (strings, tokens, bools) always use held. This should be a clean, separable module.

## Acceptance Criteria

Held interpolation returns previous sample value. Linear interpolation returns correct lerp result. Non-numeric types fall back to held. Edge cases (before first sample, after last sample, exactly at a sample) handled correctly.

