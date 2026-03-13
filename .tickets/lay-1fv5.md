---
id: lay-1fv5
status: closed
deps: []
links: []
created: 2026-03-13T18:05:21Z
type: feature
priority: 1
assignee: Bruce Mitchener
parent: lay-keas
tags: [composition, spec-alignment]
---
# Implement Specializes composition arc

Implement the Specializes composition arc (§10.3, §5.1.33). Specializes is similar to Inherits — it aggregates opinions from a base prim through all levels of referencing — but sits at the weakest position in LIVERPS (after Payloads). This means specialized opinions are the easiest to override. The ArcKind::Specializes enum variant already exists but is not wired into composition.

## Design

Follow the inherits implementation pattern closely. Key differences from inherits: (1) weaker in LIVERPS — specializes opinions lose to all other arc types, (2) like inherits, specializes propagates through references (implied specializes), (3) namespace mapping is the same as inherits. Add specializes field to PrimSpec, expand in population.rs, collect opinions in compose.rs.

## Acceptance Criteria

BasicSpecializes_root, BasicSpecializesAndInherits_root, BasicSpecializesAndReferences_root, BasicSpecializesAndVariants_root, SpecializesAndAncestralArcs_root (and all variants), TrickySpecializes* fixtures pass.

