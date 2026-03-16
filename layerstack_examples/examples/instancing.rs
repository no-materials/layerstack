//! USD instancing: shared structure with local overrides stripped.
//!
//! Instancing (AOUSD Core §11) lets multiple prims reference the same
//! target while marking it `instanceable = true`. The composition engine
//! strips local descendant opinions from the instance, leaving only the
//! opinions introduced by the referenced asset. This enables renderers to
//! share geometry and shading data across instances.
//!
//! This example builds a small scene:
//!
//! ```text
//! asset.usd (LayerId 2)
//!   /Prop           ← defines "geom" child, field "color" = blue
//!     /geom         ← field "vertices" = 100
//!
//! root.usd (LayerId 1)
//!   /InstancedA     ← references /Prop, instanceable = true
//!     /geom         ← local override "vertices" = 999 (STRIPPED)
//!   /InstancedB     ← references /Prop, instanceable = true
//!   /PlainRef       ← references /Prop, instanceable = false
//!     /geom         ← local override "vertices" = 42 (KEPT)
//! ```
//!
//! After composition:
//! - `/InstancedA/geom` has vertices = 100 (local override stripped)
//! - `/InstancedB/geom` has vertices = 100 (no local override)
//! - `/PlainRef/geom`   has vertices = 42  (local override kept)

use layerstack::{
    FieldValue, HashMap, InMemoryStore, Layer, LayerId, Path, PathId, PrimSpec, Reference,
    Specifier, Stage, StageOptions, Value,
};

fn p(store: &mut InMemoryStore, s: &str) -> PathId {
    store
        .paths
        .intern(Path::parse_absolute(s, &mut store.tokens).expect("valid path"))
}

fn main() {
    let mut store = InMemoryStore::default();

    // Intern field names.
    let f_color = store.tokens.intern("color");
    let f_vertices = store.tokens.intern("vertices");

    // Intern paths.
    let prop = p(&mut store, "/Prop");
    let prop_geom = p(&mut store, "/Prop/geom");
    let inst_a = p(&mut store, "/InstancedA");
    let inst_a_geom = p(&mut store, "/InstancedA/geom");
    let inst_b = p(&mut store, "/InstancedB");
    let inst_b_geom = p(&mut store, "/InstancedB/geom");
    let plain = p(&mut store, "/PlainRef");
    let plain_geom = p(&mut store, "/PlainRef/geom");

    let geom_tok = store.tokens.intern("geom");

    // -----------------------------------------------------------------------
    // Asset layer: the shared prop definition.
    // -----------------------------------------------------------------------
    let mut asset = Layer {
        id: LayerId(2),
        sublayers: vec![],
        prims: HashMap::new(),
    };

    let mut prop_spec = PrimSpec {
        specifier: Some(Specifier::Def),
        authored_children: vec![geom_tok],
        ..PrimSpec::default()
    };
    prop_spec
        .fields
        .insert(f_color, FieldValue::Value(Value::String("blue".into())));
    asset.prims.insert(prop, prop_spec);

    let mut geom_spec = PrimSpec {
        specifier: Some(Specifier::Def),
        ..PrimSpec::default()
    };
    geom_spec
        .fields
        .insert(f_vertices, FieldValue::Value(Value::Int(100)));
    asset.prims.insert(prop_geom, geom_spec);

    store.insert_layer(asset);

    // -----------------------------------------------------------------------
    // Root layer: three prims referencing the shared prop.
    // -----------------------------------------------------------------------
    let mut root = Layer {
        id: LayerId(1),
        sublayers: vec![],
        prims: HashMap::new(),
    };

    // InstancedA: instanceable, with a local override on geom.
    let mut inst_a_spec = PrimSpec {
        specifier: Some(Specifier::Def),
        instanceable: Some(true),
        ..PrimSpec::default()
    };
    inst_a_spec.references.append.push(Reference {
        layer: LayerId(2),
        prim_path: prop,
        asset: Some("asset.usd".into()),
    });
    root.prims.insert(inst_a, inst_a_spec);

    // Local override on InstancedA/geom — will be stripped by instancing.
    let mut inst_a_geom_spec = PrimSpec {
        specifier: Some(Specifier::Over),
        ..PrimSpec::default()
    };
    inst_a_geom_spec
        .fields
        .insert(f_vertices, FieldValue::Value(Value::Int(999)));
    root.prims.insert(inst_a_geom, inst_a_geom_spec);

    // InstancedB: instanceable, no local overrides.
    let mut inst_b_spec = PrimSpec {
        specifier: Some(Specifier::Def),
        instanceable: Some(true),
        ..PrimSpec::default()
    };
    inst_b_spec.references.append.push(Reference {
        layer: LayerId(2),
        prim_path: prop,
        asset: Some("asset.usd".into()),
    });
    root.prims.insert(inst_b, inst_b_spec);

    // PlainRef: NOT instanceable, with a local override on geom.
    let mut plain_spec = PrimSpec {
        specifier: Some(Specifier::Def),
        instanceable: Some(false),
        ..PrimSpec::default()
    };
    plain_spec.references.append.push(Reference {
        layer: LayerId(2),
        prim_path: prop,
        asset: Some("asset.usd".into()),
    });
    root.prims.insert(plain, plain_spec);

    // Local override on PlainRef/geom — will be kept (not instanceable).
    let mut plain_geom_spec = PrimSpec {
        specifier: Some(Specifier::Over),
        ..PrimSpec::default()
    };
    plain_geom_spec
        .fields
        .insert(f_vertices, FieldValue::Value(Value::Int(42)));
    root.prims.insert(plain_geom, plain_geom_spec);

    store.insert_layer(root);

    // -----------------------------------------------------------------------
    // Compose and inspect.
    // -----------------------------------------------------------------------
    let stage = Stage::compose(
        &mut store,
        LayerId(1),
        StageOptions {
            with_provenance: true,
            ..StageOptions::default()
        },
    );

    println!("=== Instancing Example ===\n");

    // Show children of each reference.
    for (name, path) in [
        ("/InstancedA", inst_a),
        ("/InstancedB", inst_b),
        ("/PlainRef", plain),
    ] {
        let kids: Vec<&str> = stage
            .children_of(path)
            .unwrap_or(&[])
            .iter()
            .map(|c| {
                store
                    .paths
                    .resolve(*c)
                    .leaf()
                    .map(|t| store.tokens.resolve(t))
                    .unwrap_or("?")
            })
            .collect();
        println!("{name} children: {kids:?}");
    }

    println!();

    // Resolve "vertices" on each geom prim.
    for (name, geom_path) in [
        ("/InstancedA/geom", inst_a_geom),
        ("/InstancedB/geom", inst_b_geom),
        ("/PlainRef/geom", plain_geom),
    ] {
        let resolved = stage.resolve_field(geom_path, f_vertices);
        match resolved {
            Some(r) => {
                let layer = r.provenance.map(|p| p.layer.0).unwrap_or(0);
                println!("{name}: vertices = {:?} (from layer {layer})", r.value);
            }
            None => println!("{name}: vertices = <not authored>"),
        }
    }

    // Verify instancing stripped the local override.
    let inst_a_verts = stage.resolve_field(inst_a_geom, f_vertices);
    let plain_verts = stage.resolve_field(plain_geom, f_vertices);

    println!();
    assert_eq!(
        inst_a_verts.as_ref().map(|r| &r.value),
        Some(&Value::Int(100)),
        "InstancedA/geom should have vertices=100 (local override stripped)"
    );
    println!("✓ InstancedA/geom: local override stripped, vertices = 100 from asset");

    assert_eq!(
        plain_verts.as_ref().map(|r| &r.value),
        Some(&Value::Int(42)),
        "PlainRef/geom should have vertices=42 (local override kept)"
    );
    println!("✓ PlainRef/geom: local override kept, vertices = 42");
}
