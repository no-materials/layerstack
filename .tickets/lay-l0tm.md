---
id: lay-l0tm
status: open
deps: []
links: []
created: 2026-03-13T18:05:33Z
type: feature
priority: 2
assignee: Bruce Mitchener
parent: lay-keas
tags: [composition, spec-alignment]
---
# Implement Relocates composition arc

Implement the Relocates composition arc (§10.3, §5.1.29). Relocates maps a prim path from one location to another within the local namespace, effectively moving prims introduced by other arcs. Its position in LIVERPS is between VariantSets and References. This is the most complex composition arc because it requires path remapping across all other arcs and affects how other arcs' opinions are addressed.

## Design

Relocates requires: (1) a source→target path mapping on each prim spec, (2) during composition, remap paths from source to target for all opinions contributed by weaker arcs, (3) handle cascading relocates and validation of conflicting relocates (§ErrorInvalidConflictingRelocates), (4) opinions at relocation sources may produce errors (§ErrorOpinionAtRelocationSource). The Relocates type was added to the crate binary format in §16.3.10.15. Consider implementing incrementally: basic single-level relocates first, then cascading/nested cases.

## Acceptance Criteria

BasicRelocateToAnimInterface_root, ElidedAncestralRelocates_root, RelocatePrimsWithSameName_root, RelocateToNone_root, all TrickyInheritsAndRelocates*, TrickyMultipleRelocations*, TrickyLocalClassHierarchyWithRelocates_root, SubrootReferenceAndRelocates_root, and error fixtures pass.

