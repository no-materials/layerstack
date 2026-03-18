---
id: lay-xrth
status: closed
deps: [lay-2b56]
links: []
created: 2026-03-13T18:06:20Z
type: feature
priority: 2
assignee: Bruce Mitchener
parent: lay-avtk
tags: [value-resolution, spec-alignment]
---
# Spline evaluation

Implement spline-based attribute value resolution (§12.3.3, §12.5). Splines provide smooth interpolation between knots using Bézier or Hermite curves. A spline is defined by knots with time/value pairs and tangent information. Splines sit between timeSamples and default in the resolution priority order. Spline data includes pre/post extrapolation modes (held, linear, sloped, loop repeat/reset/oscillate) and inner loop parameters.

## Design

The spline data model is defined in §16.3.10.33 with flag bytes encoding curve type, data type, extrapolation modes, and knot information. Evaluation requires: (1) finding the segment containing the query time, (2) evaluating the Bézier/Hermite curve for that segment, (3) handling extrapolation outside the knot range, (4) inner loop evaluation. Support float/double/half data types for knot values.

## Acceptance Criteria

Spline evaluation at knots returns exact values. Evaluation between knots returns correct curve values. Extrapolation modes (held, linear, sloped, loop) produce correct results. Hermite and Bézier curve types both work.

