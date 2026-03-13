---
id: lay-avtk
status: open
deps: []
links: []
created: 2026-03-13T18:04:27Z
type: epic
priority: 1
assignee: Bruce Mitchener
tags: [value-resolution, spec-alignment]
---
# Value Resolution

Implement the full value resolution pipeline per §12. Currently the stage resolves scalar fields by returning the strongest opinion and composes list ops. Missing: time-varying resolution (timeSamples, splines, value clips), interpolation methods, value blocking, layer offset/scale, fallback values from schema definitions, and dictionary combining.

## Design

Value resolution sits on top of composition. The Stage API needs to accept a time parameter. Resolution order per §12.3: timeSamples > spline > default > clips, checked per-spec in strength order. Interpolation (§12.5) is a separable concern. Value clips (§12.3.4) are the most complex subsystem with manifest files, active/times metadata, and template expansion.

## Acceptance Criteria

Upstream value_resolution test fixtures pass. Time-varying queries return correct interpolated values. Value blocking correctly skips weaker opinions and falls back.

