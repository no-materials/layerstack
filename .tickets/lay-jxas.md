---
id: lay-jxas
status: open
deps: []
links: []
created: 2026-03-13T18:07:12Z
type: feature
priority: 3
assignee: Bruce Mitchener
parent: lay-c22s
tags: [data-model, spec-alignment]
---
# Implement dimensioned types (vectors, matrices, quaternions)

Add dimensioned types from §6.3 to the data model. These include: vectors (double2/3/4, float2/3/4, half2/3/4, int2/3/4), matrices (matrix2d, matrix3d, matrix4d — row-major, f64), and quaternions (quatd, quatf, quath — imaginary-first, real-last layout). Vectors are row vectors that pre-multiply matrices. Matrices store translations in the 4th row.

## Design

These are fixed-size numeric arrays. Consider inline storage (e.g., [f64; 4] for double4) vs. boxed storage for larger types (matrix4d = 128 bytes). Quaternion layout is (i,j,k,r) in storage but displayed as (r,i,j,k) per §16.3.10.22. Must support arrays of dimensioned types (e.g., point3f[]). Keep no_std compatible — avoid pulling in nalgebra or similar.

## Acceptance Criteria

All §6.3 dimensioned types representable. Correct memory layout for binary format compatibility. Array variants (e.g., float3[]) supported.

