---
id: lay-fv0h
status: open
deps: [lay-n7r4, lay-ogax]
links: []
created: 2026-03-13T18:06:32Z
type: feature
priority: 2
assignee: Bruce Mitchener
parent: lay-avtk
tags: [value-resolution, spec-alignment]
---
# Value clips

Implement value clips for partitioning time samples across multiple layers (§12.3.4). Value clips allow heavy animated data (crowds, simulations) to be split into per-frame or per-range files. A clip set is defined by metadata on a prim (clipSets/clips dictionary) with either explicit metadata (assetPaths, active, times) or template metadata (templateAssetPath, templateStartTime/EndTime/Stride). Resolution requires: manifest lookup, active clip determination, stage-to-clip time mapping, and interpolation within clips.

## Design

Key subsystems: (1) clip set definition parsing from prim metadata, (2) template expansion to derive explicit metadata, (3) active clip determination at a given stage time, (4) stage→clip time mapping via the timing curve, (5) querying the active clip layer for attribute values, (6) missing value handling (default from manifest or interpolation from surrounding clips per §12.3.4.6-7), (7) jump discontinuities (§12.3.4.8). Clip strength is just weaker than Local in the anchoring layer stack.

## Acceptance Criteria

Upstream value_resolution/value_clips fixtures pass. Explicit and template clip metadata both work. Jump discontinuities and looping produce correct values. Missing clip values handled per interpolateMissingClipValues setting.

