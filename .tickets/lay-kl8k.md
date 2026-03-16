---
id: lay-kl8k
status: open
deps: []
links: [lay-y8q1]
created: 2026-03-16T12:12:34Z
type: feature
priority: 3
assignee: Bruce Mitchener
parent: lay-uc3n
tags: [schemas, ergonomics, rust-interop]
---
# Typed views: Rust struct ergonomics over composed prims

Explore derive-macro or trait-based typed views over composed prims, providing Rust-native struct ergonomics for reading and writing schema-typed data while preserving per-field composition semantics.

## Design

The core tension: USD composition is inherently dynamic and field-granular — a stronger layer can override one property without touching others. Rust structs are static and monolithic. A typed view layer must bridge this gap without compromising either side.

Possible approaches:

1. **Trait-based typed views (read side)**: A TypedView<T> that projects a composed prim into a Rust struct, resolving fields on access. Something like stage.view::<Mesh>(path)? returning a wrapper whose accessors call resolve_field with the right tokens and value conversions. Pro: zero-cost at compose time, ergonomic reads. Con: lazy resolution, no caching without care.

2. **Derive macro on user structs**: #[derive(PrimSchema)] that generates token interning, field-name-to-token mappings, typed accessors, and optionally schema registration with fallback values. Pro: single source of truth, compile-time validation. Con: macro complexity, unclear how to handle USD-specific concepts (timeSamples, listOps) in struct fields.

3. **Code generation from schema definitions**: External schema files (or Rust DSL) generate both the typed Rust structs and the schema registry entries. Pro: matches USD's schema-definition-driven workflow. Con: build complexity, code gen maintenance.

4. **Hybrid**: Traits for the read/write view layer, derive macros for the mapping boilerplate, registry as an optional backing store for fallback values. The view layer and the registry are independent — views work with or without registered schemas.

Key open questions:

- Should typed views be zero-copy references into the stage, or should they extract/clone values into owned structs?
- How do timeSamples and listOps map to Rust types? A field could be Value::Double at default time but FieldValue::TimeSamples with animation — the Rust type needs to handle both.
- How does the write side work? mesh.set_vertices(data) needs to know which layer to write to and produce the right PrimSpec mutations.
- Should schema fallback values live in the registry (lay-y8q1) or be derived from struct field defaults (#[schema(default = 0.0)])?
- How does this interact with no_std? Derive macros are fine, but proc-macro crates need std. The generated code can be no_std.
- What about nested schemas and relationships (e.g., a Material referencing Shaders)? These cross prim boundaries.
- How do typed views interact with instancing? Instance-root properties are kept, descendant opinions are stripped — the view needs to reflect this.
- Can the same approach work for both built-in USD schemas (Mesh, Xform) and user-defined application schemas?

## Acceptance Criteria

Design decision documented. Spike implementation of typed view for one schema type (e.g., Mesh or Xform) demonstrating read-side ergonomics. Evaluate trade-offs against registry-first approach (lay-y8q1) and recommend sequencing.

