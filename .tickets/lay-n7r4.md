---
id: lay-n7r4
status: closed
deps: [lay-2b56]
links: []
created: 2026-03-13T18:05:51Z
type: feature
priority: 1
assignee: Bruce Mitchener
parent: lay-avtk
tags: [value-resolution, spec-alignment]
---
# Time-varying attribute resolution (timeSamples)

Implement time-varying attribute value resolution via the timeSamples metadata field (§12.3.2.2). When an attribute value is queried at a specific time, the timeSamples field (a map of timeCode→value) is consulted. The correct value is found by locating the bracketing time samples and interpolating. TimeSamples take priority over spline and default values per §12.3. The data model needs a TimeSamples field variant in FieldValue, and Stage needs time-parameterized query APIs.

## Design

Add timeSamples support to FieldValue. Extend Stage::resolve_value to accept an optional time parameter. When time is specified, iterate specs in strength order checking for timeSamples first, then spline, then default, then clips (§12.3). Interpolation between samples uses the layer's interpolation type (held or linear, §12.5). TimeSamples are stored as sorted (timeCode, value) pairs.

## Acceptance Criteria

Querying an attribute at authored time codes returns exact values. Querying between time codes returns interpolated values (held or linear). TimeSamples on stronger layers override weaker layers entirely. Upstream value_resolution/tests/assets/timesamples fixtures pass.

