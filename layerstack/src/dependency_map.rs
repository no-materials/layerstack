//! Dependency edges discovered during stage composition.
//!
//! When [`crate::stage::StageOptions::with_dependencies`] is enabled, the
//! composition algorithm records every arc and layer-opinion edge it discovers.
//! The resulting [`DependencyMap`] is queryable on the composed [`crate::stage::Stage`].

use alloc::vec::Vec;

use hashbrown::HashSet;

use crate::{doc::LayerId, path::PathId, prim_index::ArcKind};

/// A composition arc edge discovered during composition.
///
/// Direction: source was composed into target. "If source changes,
/// target may need recomposition."
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ArcDependency {
    /// The prim that authored the arc (or the arc target, for inherits).
    pub source: PathId,
    /// The composed prim that received opinions through this arc.
    pub target: PathId,
    /// Which arc type introduced this dependency.
    pub arc_kind: ArcKind,
    /// The layer providing the referenced opinions.
    pub layer: LayerId,
}

/// Layer-to-prim opinion dependency.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LayerDependency {
    /// The layer contributing opinions.
    pub layer: LayerId,
    /// The composed prim receiving opinions from this layer.
    pub prim: PathId,
}

/// Dependency edges discovered during stage composition.
///
/// Built by [`DependencyMapBuilder`] during composition when
/// [`crate::stage::StageOptions::with_dependencies`] is enabled.
#[derive(Clone, Debug, Default)]
pub struct DependencyMap {
    /// Arc-level dependencies (reference, inherit, payload, specialize edges).
    arcs: Vec<ArcDependency>,
    /// Layer-to-prim opinion dependencies.
    layer_opinions: Vec<LayerDependency>,
}

impl DependencyMap {
    /// Returns all arc dependencies.
    #[must_use]
    pub fn arcs(&self) -> &[ArcDependency] {
        &self.arcs
    }

    /// Returns all layer-opinion dependencies.
    #[must_use]
    pub fn layer_opinions(&self) -> &[LayerDependency] {
        &self.layer_opinions
    }

    /// Returns arc dependencies targeting the given prim.
    #[must_use]
    pub fn arcs_targeting(&self, prim: PathId) -> Vec<&ArcDependency> {
        self.arcs.iter().filter(|a| a.target == prim).collect()
    }

    /// Returns prims affected by opinions from the given layer.
    #[must_use]
    pub fn prims_affected_by_layer(&self, layer: LayerId) -> Vec<PathId> {
        let mut seen = HashSet::new();
        self.layer_opinions
            .iter()
            .filter(|d| d.layer == layer)
            .filter(|d| seen.insert(d.prim))
            .map(|d| d.prim)
            .collect()
    }

    /// Returns layers that contribute opinions to the given prim.
    #[must_use]
    pub fn layers_affecting_prim(&self, prim: PathId) -> Vec<LayerId> {
        let mut seen = HashSet::new();
        self.layer_opinions
            .iter()
            .filter(|d| d.prim == prim)
            .filter(|d| seen.insert(d.layer))
            .map(|d| d.layer)
            .collect()
    }

    /// Returns the total number of dependency edges.
    #[must_use]
    pub fn len(&self) -> usize {
        self.arcs.len() + self.layer_opinions.len()
    }

    /// Returns `true` if no dependency edges were recorded.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.arcs.is_empty() && self.layer_opinions.is_empty()
    }
}

/// Builder for [`DependencyMap`], used during composition.
pub(crate) struct DependencyMapBuilder {
    arc_set: HashSet<ArcDependency>,
    layer_set: HashSet<LayerDependency>,
}

impl DependencyMapBuilder {
    pub(crate) fn new() -> Self {
        Self {
            arc_set: HashSet::new(),
            layer_set: HashSet::new(),
        }
    }

    /// Records an arc dependency edge.
    pub(crate) fn add_arc(&mut self, dep: ArcDependency) {
        self.arc_set.insert(dep);
    }

    /// Records a layer-opinion dependency edge.
    pub(crate) fn add_layer_opinion(&mut self, dep: LayerDependency) {
        self.layer_set.insert(dep);
    }

    /// Consumes the builder and produces a [`DependencyMap`].
    pub(crate) fn finish(self) -> DependencyMap {
        DependencyMap {
            arcs: self.arc_set.into_iter().collect(),
            layer_opinions: self.layer_set.into_iter().collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_deduplicates_arcs() {
        let mut builder = DependencyMapBuilder::new();
        let dep = ArcDependency {
            source: PathId::from_raw(1),
            target: PathId::from_raw(2),
            arc_kind: ArcKind::References,
            layer: LayerId(10),
        };
        builder.add_arc(dep);
        builder.add_arc(dep);
        let map = builder.finish();
        assert_eq!(map.arcs().len(), 1);
    }

    #[test]
    fn builder_deduplicates_layer_opinions() {
        let mut builder = DependencyMapBuilder::new();
        let dep = LayerDependency {
            layer: LayerId(1),
            prim: PathId::from_raw(5),
        };
        builder.add_layer_opinion(dep);
        builder.add_layer_opinion(dep);
        let map = builder.finish();
        assert_eq!(map.layer_opinions().len(), 1);
    }

    #[test]
    fn query_arcs_targeting() {
        let mut builder = DependencyMapBuilder::new();
        let target = PathId::from_raw(2);
        builder.add_arc(ArcDependency {
            source: PathId::from_raw(1),
            target,
            arc_kind: ArcKind::Inherits,
            layer: LayerId(1),
        });
        builder.add_arc(ArcDependency {
            source: PathId::from_raw(3),
            target,
            arc_kind: ArcKind::References,
            layer: LayerId(2),
        });
        builder.add_arc(ArcDependency {
            source: PathId::from_raw(4),
            target: PathId::from_raw(99),
            arc_kind: ArcKind::Payloads,
            layer: LayerId(3),
        });
        let map = builder.finish();
        assert_eq!(map.arcs_targeting(target).len(), 2);
        assert_eq!(map.arcs_targeting(PathId::from_raw(99)).len(), 1);
        assert_eq!(map.arcs_targeting(PathId::from_raw(0)).len(), 0);
    }

    #[test]
    fn query_prims_affected_by_layer() {
        let mut builder = DependencyMapBuilder::new();
        let layer = LayerId(1);
        builder.add_layer_opinion(LayerDependency {
            layer,
            prim: PathId::from_raw(10),
        });
        builder.add_layer_opinion(LayerDependency {
            layer,
            prim: PathId::from_raw(20),
        });
        builder.add_layer_opinion(LayerDependency {
            layer: LayerId(2),
            prim: PathId::from_raw(30),
        });
        let map = builder.finish();
        let affected = map.prims_affected_by_layer(layer);
        assert_eq!(affected.len(), 2);
        assert!(affected.contains(&PathId::from_raw(10)));
        assert!(affected.contains(&PathId::from_raw(20)));
    }

    #[test]
    fn query_layers_affecting_prim() {
        let mut builder = DependencyMapBuilder::new();
        let prim = PathId::from_raw(5);
        builder.add_layer_opinion(LayerDependency {
            layer: LayerId(1),
            prim,
        });
        builder.add_layer_opinion(LayerDependency {
            layer: LayerId(2),
            prim,
        });
        builder.add_layer_opinion(LayerDependency {
            layer: LayerId(3),
            prim: PathId::from_raw(99),
        });
        let map = builder.finish();
        let layers = map.layers_affecting_prim(prim);
        assert_eq!(layers.len(), 2);
        assert!(layers.contains(&LayerId(1)));
        assert!(layers.contains(&LayerId(2)));
    }

    #[test]
    fn len_and_is_empty() {
        let map = DependencyMap::default();
        assert!(map.is_empty());
        assert_eq!(map.len(), 0);

        let mut builder = DependencyMapBuilder::new();
        builder.add_arc(ArcDependency {
            source: PathId::from_raw(1),
            target: PathId::from_raw(2),
            arc_kind: ArcKind::Inherits,
            layer: LayerId(1),
        });
        builder.add_layer_opinion(LayerDependency {
            layer: LayerId(1),
            prim: PathId::from_raw(3),
        });
        let map = builder.finish();
        assert!(!map.is_empty());
        assert_eq!(map.len(), 2);
    }
}
