---
id: lay-eknp
status: closed
deps: [lay-v1mn, lay-ogax]
links: []
created: 2026-03-13T18:08:10Z
type: feature
priority: 3
assignee: Bruce Mitchener
parent: lay-p1nr
tags: [file-formats, spec-alignment]
---
# USDZ package reader

Implement a USDZ package reader per §16.4. USDZ is a constrained ZIP file containing USD layers and associated media (textures, audio). Constraints: uncompressed, unencrypted, 32-bit ZIP, 64-byte aligned file headers, first file is the root layer, no comment in End of Central Directory record. The reader must extract the root layer and resolve internal asset references to other files within the package.

## Design

USDZ is a standard ZIP with restrictions (§16.4.1). Can use a minimal ZIP reader (or existing crate with feature gates) that validates the USDZ constraints. The key integration point is with the resource interface (§9) — asset paths within a USDZ resolve to entries in the same package. The first file in both the local file headers and central directory must be the root layer. Supported file types: .usd/.usda/.usdc, .png/.jpg/.jpeg/.exr/.avif, .m4a/.mp3/.wav.

## Acceptance Criteria

Can open USDZ files and extract the root layer. Internal asset references (e.g., textures referenced by layers) resolve correctly within the package. Validates USDZ constraints (uncompressed, 64-byte alignment, no comment).

