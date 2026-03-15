//! Dependency edges discovered during stage composition.
//!
//! When [`crate::stage::StageOptions::with_dependencies`] is enabled, the
//! composition algorithm records every arc and layer-opinion edge it discovers.
//! The resulting [`DependencyMap`] is queryable on the composed [`crate::stage::Stage`].
//!
//! Internally, arc dependencies are stored in an [`InvalidationGraph`] which
//! serves as the single source of truth for the dependency topology. Arc
//! metadata ([`ArcKind`], [`LayerId`]) is stored separately for diagnostic
//! queries.

use alloc::vec::Vec;

use hashbrown::{HashMap, HashSet};
use invalidation::{CycleHandling, InvalidationGraph};

use crate::{doc::LayerId, path::PathId, prim_index::ArcKind};

use crate::live_stage::OPINION_EDIT;

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

/// Dependency edges discovered during stage composition.
///
/// The arc dependency topology is stored in an [`InvalidationGraph`], which
/// is the authoritative source for "which prims depend on which other prims."
/// Arc metadata and layer-opinion edges are stored alongside for diagnostic
/// and notification queries.
#[derive(Clone, Debug, Default)]
pub struct DependencyMap {
    /// The invalidation graph: `target` depends on `source` in `OPINION_EDIT`.
    graph: InvalidationGraph<PathId>,
    /// Arc metadata for diagnostic queries (keyed by `(source, target)`).
    arc_metadata: HashSet<ArcDependency>,
    /// Layer → prims that receive opinions from that layer.
    layer_to_prims: HashMap<LayerId, HashSet<PathId>>,
    /// Prim → layers that contribute opinions to it.
    prim_to_layers: HashMap<PathId, HashSet<LayerId>>,
}

impl DependencyMap {
    /// Returns a reference to the underlying invalidation graph.
    #[must_use]
    pub fn graph(&self) -> &InvalidationGraph<PathId> {
        &self.graph
    }

    /// Consumes `self` and returns the underlying invalidation graph.
    #[must_use]
    pub fn into_graph(self) -> InvalidationGraph<PathId> {
        self.graph
    }

    /// Returns all arc dependencies (diagnostic/inspection API).
    #[must_use]
    pub fn arcs(&self) -> Vec<ArcDependency> {
        self.arc_metadata.iter().copied().collect()
    }

    /// Returns arc dependencies targeting the given prim.
    #[must_use]
    pub fn arcs_targeting(&self, prim: PathId) -> Vec<ArcDependency> {
        self.arc_metadata
            .iter()
            .filter(|a| a.target == prim)
            .copied()
            .collect()
    }

    /// Returns prims affected by opinions from the given layer.
    #[must_use]
    pub fn prims_affected_by_layer(&self, layer: LayerId) -> Vec<PathId> {
        self.layer_to_prims
            .get(&layer)
            .map(|set| set.iter().copied().collect())
            .unwrap_or_default()
    }

    /// Returns layers that contribute opinions to the given prim.
    #[must_use]
    pub fn layers_affecting_prim(&self, prim: PathId) -> Vec<LayerId> {
        self.prim_to_layers
            .get(&prim)
            .map(|set| set.iter().copied().collect())
            .unwrap_or_default()
    }

    /// Returns prim paths that depend on `source` (i.e. prims that need
    /// recomposition when `source` changes).
    pub fn dependents_of(&self, source: PathId) -> impl Iterator<Item = PathId> + '_ {
        self.graph.dependents(source, OPINION_EDIT)
    }

    /// Returns prim paths that `target` depends on (i.e. arc sources that
    /// feed opinions into `target`).
    pub fn dependencies_of(&self, target: PathId) -> impl Iterator<Item = PathId> + '_ {
        self.graph.dependencies(target, OPINION_EDIT)
    }

    /// Returns `true` if no dependency edges were recorded.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.arc_metadata.is_empty() && self.layer_to_prims.is_empty()
    }

    /// Removes all edges involving the given prim and re-adds them from the
    /// provided arc and layer-opinion lists.
    ///
    /// This is used during scoped recomposition to update edges for
    /// recomposed prims without rebuilding the entire map.
    pub(crate) fn update_prim_edges(
        &mut self,
        prim: PathId,
        new_arcs: &[ArcDependency],
        new_layers: &[LayerId],
    ) {
        // Remove old arc metadata and graph edges for this prim (as target).
        self.arc_metadata.retain(|a| a.target != prim);
        // Remove old graph edges where prim is the dependent.
        let old_deps: Vec<PathId> = self.graph.dependencies(prim, OPINION_EDIT).collect();
        for dep in old_deps {
            self.graph.remove_dependency(prim, dep, OPINION_EDIT);
        }

        // Remove old layer-opinion edges for this prim.
        if let Some(old_layers) = self.prim_to_layers.remove(&prim) {
            for layer in &old_layers {
                if let Some(prim_set) = self.layer_to_prims.get_mut(layer) {
                    prim_set.remove(&prim);
                }
            }
        }

        // Add new arcs.
        for arc in new_arcs {
            self.arc_metadata.insert(*arc);
            let _ = self.graph.add_dependency(
                arc.target,
                arc.source,
                OPINION_EDIT,
                CycleHandling::Ignore,
            );
        }

        // Add new layer-opinion edges.
        let mut layers_for_prim = HashSet::new();
        for &layer in new_layers {
            self.layer_to_prims.entry(layer).or_default().insert(prim);
            layers_for_prim.insert(layer);
        }
        if !layers_for_prim.is_empty() {
            self.prim_to_layers.insert(prim, layers_for_prim);
        }
    }
}

/// Builder for [`DependencyMap`], used during composition.
///
/// Writes arc dependencies directly into an [`InvalidationGraph`] and
/// records metadata and layer-opinion edges for queries.
pub(crate) struct DependencyMapBuilder {
    graph: InvalidationGraph<PathId>,
    arc_set: HashSet<ArcDependency>,
    layer_to_prims: HashMap<LayerId, HashSet<PathId>>,
    prim_to_layers: HashMap<PathId, HashSet<LayerId>>,
}

impl DependencyMapBuilder {
    pub(crate) fn new() -> Self {
        Self {
            graph: InvalidationGraph::new(),
            arc_set: HashSet::new(),
            layer_to_prims: HashMap::new(),
            prim_to_layers: HashMap::new(),
        }
    }

    /// Records an arc dependency edge.
    ///
    /// Writes both the graph edge (target depends on source) and the
    /// diagnostic metadata.
    pub(crate) fn add_arc(&mut self, dep: ArcDependency) {
        if self.arc_set.insert(dep) {
            // target depends on source: if source changes, target needs recomposition.
            let _ = self.graph.add_dependency(
                dep.target,
                dep.source,
                OPINION_EDIT,
                CycleHandling::Ignore,
            );
        }
    }

    /// Records a layer-opinion dependency edge.
    pub(crate) fn add_layer_opinion(&mut self, layer: LayerId, prim: PathId) {
        self.layer_to_prims.entry(layer).or_default().insert(prim);
        self.prim_to_layers.entry(prim).or_default().insert(layer);
    }

    /// Consumes the builder and produces a [`DependencyMap`].
    pub(crate) fn finish(self) -> DependencyMap {
        DependencyMap {
            graph: self.graph,
            arc_metadata: self.arc_set,
            layer_to_prims: self.layer_to_prims,
            prim_to_layers: self.prim_to_layers,
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
        assert_eq!(map.arcs().len(), 1, "duplicate arcs should be deduplicated");
    }

    #[test]
    fn builder_deduplicates_layer_opinions() {
        let mut builder = DependencyMapBuilder::new();
        builder.add_layer_opinion(LayerId(1), PathId::from_raw(5));
        builder.add_layer_opinion(LayerId(1), PathId::from_raw(5));
        let map = builder.finish();
        assert_eq!(
            map.prims_affected_by_layer(LayerId(1)).len(),
            1,
            "duplicate layer opinions should be deduplicated"
        );
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
        builder.add_layer_opinion(layer, PathId::from_raw(10));
        builder.add_layer_opinion(layer, PathId::from_raw(20));
        builder.add_layer_opinion(LayerId(2), PathId::from_raw(30));
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
        builder.add_layer_opinion(LayerId(1), prim);
        builder.add_layer_opinion(LayerId(2), prim);
        builder.add_layer_opinion(LayerId(3), PathId::from_raw(99));
        let map = builder.finish();
        let layers = map.layers_affecting_prim(prim);
        assert_eq!(layers.len(), 2);
        assert!(layers.contains(&LayerId(1)));
        assert!(layers.contains(&LayerId(2)));
    }

    #[test]
    fn graph_has_correct_topology() {
        let mut builder = DependencyMapBuilder::new();
        builder.add_arc(ArcDependency {
            source: PathId::from_raw(1),
            target: PathId::from_raw(2),
            arc_kind: ArcKind::Inherits,
            layer: LayerId(1),
        });
        let map = builder.finish();

        // target (2) depends on source (1).
        let deps: Vec<_> = map
            .graph()
            .dependencies(PathId::from_raw(2), OPINION_EDIT)
            .collect();
        assert!(
            deps.contains(&PathId::from_raw(1)),
            "target should depend on source in the graph"
        );

        let dependents: Vec<_> = map
            .graph()
            .dependents(PathId::from_raw(1), OPINION_EDIT)
            .collect();
        assert!(
            dependents.contains(&PathId::from_raw(2)),
            "source should have target as a dependent"
        );
    }

    #[test]
    fn is_empty() {
        let map = DependencyMap::default();
        assert!(map.is_empty());

        let mut builder = DependencyMapBuilder::new();
        builder.add_arc(ArcDependency {
            source: PathId::from_raw(1),
            target: PathId::from_raw(2),
            arc_kind: ArcKind::Inherits,
            layer: LayerId(1),
        });
        let map = builder.finish();
        assert!(!map.is_empty());
    }

    #[test]
    fn update_prim_edges_replaces_old() {
        let mut builder = DependencyMapBuilder::new();
        builder.add_arc(ArcDependency {
            source: PathId::from_raw(1),
            target: PathId::from_raw(2),
            arc_kind: ArcKind::Inherits,
            layer: LayerId(1),
        });
        builder.add_layer_opinion(LayerId(1), PathId::from_raw(2));
        let mut map = builder.finish();

        // Update prim 2: new arc from source 3, new layer 2.
        let new_arc = ArcDependency {
            source: PathId::from_raw(3),
            target: PathId::from_raw(2),
            arc_kind: ArcKind::References,
            layer: LayerId(2),
        };
        map.update_prim_edges(PathId::from_raw(2), &[new_arc], &[LayerId(2)]);

        // Old arc from source 1 should be gone.
        let arcs = map.arcs_targeting(PathId::from_raw(2));
        assert_eq!(arcs.len(), 1);
        assert_eq!(arcs[0].source, PathId::from_raw(3));

        // Old layer opinion should be gone, new one present.
        assert!(map.prims_affected_by_layer(LayerId(1)).is_empty());
        assert!(
            map.prims_affected_by_layer(LayerId(2))
                .contains(&PathId::from_raw(2))
        );
    }
}
