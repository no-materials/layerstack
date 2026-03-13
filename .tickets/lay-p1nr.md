---
id: lay-p1nr
status: open
deps: []
links: []
created: 2026-03-13T18:04:49Z
type: epic
priority: 2
assignee: Bruce Mitchener
tags: [file-formats, spec-alignment]
---
# File Formats

Implement production-quality readers (and eventually writers) for the three core file formats specified in §16: USDA (text, §16.2), USDC (binary/crate, §16.3), and USDZ (package, §16.4). Currently only a minimal USDA parser exists in the conformance crate for loading test fixtures. Full format support is required for compliance (§4.4.3).

## Design

USDA parser should be grammar-driven from the PEG in §16.2. USDC reader needs the bootstrap/sections/value-representation pipeline from §16.3. USDZ is a constrained ZIP (§16.4) wrapping USDA/USDC. Each format should live in its own crate or module. Reader-first; writers can follow. The USDA parser is highest priority as it unblocks all conformance fixtures.

## Acceptance Criteria

Can round-trip read all conformance fixture files. Passes §4.4.3 format compliance tests. USDZ reads 64-byte aligned uncompressed ZIPs per §16.4.

