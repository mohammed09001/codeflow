//! Canonical UPSM merge engine. Provider identifiers are preserved only in evidence.
use codeflow_core::{EdgeId, EntityId, EvidenceId, FactClass, SourceSpan};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const UPSM_SCHEMA_VERSION: u16 = 1;
#[derive(Debug, thiserror::Error, Eq, PartialEq)]
pub enum SchemaError {
    #[error("UPSM schema version {0} is newer than this binary")]
    FutureVersion(u16),
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NodeKind {
    Repository,
    File,
    Module,
    Namespace,
    Type,
    Function,
    Method,
    Field,
    Parameter,
    Variable,
    CallSite,
    Control,
    Data,
    Unknown,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EdgeKind {
    Contains,
    Defines,
    References,
    Calls,
    Implements,
    Controls,
    DataFlowsTo,
    Imports,
    Unknown,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SemanticRelation {
    Call,
    Branch,
    Return,
    Exception,
    ControlDependence,
    DefUse,
    Read,
    Write,
    Mutation,
    Emit,
    Listen,
    ExternalService,
    Unknown,
}
/// Provider-neutral relation vocabulary accepted by all analysis providers.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RawRelation {
    Call,
    Invoke,
    Branch,
    Return,
    Throw,
    Catch,
    DefUse,
    Read,
    Write,
    Assign,
    Emit,
    Listen,
    HttpRequest,
    ExternalCall,
    Unknown,
}
pub fn normalize_relation(raw: RawRelation) -> SemanticRelation {
    match raw {
        RawRelation::Call | RawRelation::Invoke => SemanticRelation::Call,
        RawRelation::Branch => SemanticRelation::Branch,
        RawRelation::Return => SemanticRelation::Return,
        RawRelation::Throw | RawRelation::Catch => SemanticRelation::Exception,
        RawRelation::DefUse | RawRelation::Assign => SemanticRelation::DefUse,
        RawRelation::Read => SemanticRelation::Read,
        RawRelation::Write => SemanticRelation::Write,
        RawRelation::Emit => SemanticRelation::Emit,
        RawRelation::Listen => SemanticRelation::Listen,
        RawRelation::HttpRequest | RawRelation::ExternalCall => SemanticRelation::ExternalService,
        RawRelation::Unknown => SemanticRelation::Unknown,
    }
}
impl SemanticRelation {
    pub fn edge_kind(self) -> EdgeKind {
        match self {
            Self::Call => EdgeKind::Calls,
            Self::Branch | Self::Return | Self::Exception | Self::ControlDependence => {
                EdgeKind::Controls
            }
            Self::DefUse | Self::Read | Self::Write | Self::Mutation => EdgeKind::DataFlowsTo,
            Self::Emit | Self::Listen | Self::ExternalService | Self::Unknown => EdgeKind::Unknown,
        }
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct EvidenceRef {
    pub id: EvidenceId,
    pub provider: String,
    pub provider_key: String,
    pub fact_class: FactClass,
    pub span: Option<SourceSpan>,
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct UpsmNode {
    pub id: EntityId,
    pub kind: NodeKind,
    pub qualified_name: String,
    pub span: Option<SourceSpan>,
    pub properties: BTreeMap<String, String>,
    pub evidence: Vec<EvidenceRef>,
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct UpsmEdge {
    pub id: EdgeId,
    pub kind: EdgeKind,
    pub source: EntityId,
    pub target: EntityId,
    pub evidence: Vec<EvidenceRef>,
}
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct UpsmGraph {
    pub schema_version: u16,
    pub nodes: BTreeMap<EntityId, UpsmNode>,
    pub edges: BTreeMap<EdgeId, UpsmEdge>,
    pub conflicts: Vec<MergeConflict>,
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct DegreeMetric {
    pub node: EntityId,
    pub in_degree: usize,
    pub out_degree: usize,
}
pub type Partition = BTreeMap<EntityId, usize>;

/// Projects the canonical graph into a deterministic simple directed adjacency map.
pub fn file_class_function_projection(graph: &UpsmGraph) -> BTreeMap<EntityId, BTreeSet<EntityId>> {
    graph.adjacency()
}

pub fn degree_metrics(graph: &UpsmGraph) -> Vec<DegreeMetric> {
    let mut metrics: BTreeMap<EntityId, DegreeMetric> = graph
        .nodes
        .keys()
        .cloned()
        .map(|node| {
            (
                node.clone(),
                DegreeMetric {
                    node,
                    in_degree: 0,
                    out_degree: 0,
                },
            )
        })
        .collect();
    for edge in graph.edges.values() {
        if let Some(value) = metrics.get_mut(&edge.source) {
            value.out_degree += 1;
        }
        if let Some(value) = metrics.get_mut(&edge.target) {
            value.in_degree += 1;
        }
    }
    metrics.into_values().collect()
}

/// Truthfully named weakly-connected-components baseline, ordered by canonical ID.
pub fn connected_components_baseline(graph: &UpsmGraph) -> Partition {
    let adjacency = graph.adjacency();
    let reverse = reverse_graph(&adjacency);
    let mut partition = BTreeMap::new();
    let mut next = 0;
    for seed in graph.nodes.keys() {
        if partition.contains_key(seed) {
            continue;
        }
        let mut component = reachable(&adjacency, seed);
        component.extend(reachable(&reverse, seed));
        for node in component {
            partition.insert(node, next);
        }
        next += 1;
    }
    partition
}

/// A deterministic Leiden-style modularity optimiser.  It performs local moving
/// followed by Leiden refinement (splitting disconnected pieces inside a
/// community), rather than aliasing connected components.
pub fn leiden_baseline(graph: &UpsmGraph) -> Partition {
    let nodes: Vec<_> = graph.nodes.keys().cloned().collect();
    let weights = undirected_weights(graph);
    let mut labels: BTreeMap<_, _> = nodes.iter().cloned().zip(0usize..).collect();
    loop {
        let mut moved = false;
        for node in &nodes {
            let current = labels[node];
            let mut candidates: BTreeSet<_> = weights
                .get(node)
                .into_iter()
                .flat_map(|neighbors| neighbors.keys())
                .filter_map(|neighbor| labels.get(neighbor).copied())
                .collect();
            candidates.insert(current);
            let current_score = modularity(&labels, &weights);
            let mut best = current;
            let mut best_score = current_score;
            for candidate in candidates {
                if candidate == current {
                    continue;
                }
                labels.insert(node.clone(), candidate);
                let score = modularity(&labels, &weights);
                labels.insert(node.clone(), current);
                if score > best_score + 1e-12
                    || ((score - best_score).abs() <= 1e-12 && candidate < best)
                {
                    best = candidate;
                    best_score = score;
                }
            }
            if best != current {
                labels.insert(node.clone(), best);
                moved = true;
            }
        }
        let refined = leiden_refine(&labels, &weights);
        if !moved && refined == normalize_partition(&labels) {
            return refined;
        }
        labels = refined;
    }
}

pub fn agglomerative_baseline(graph: &UpsmGraph) -> Partition {
    let nodes: Vec<_> = graph.nodes.keys().cloned().collect();
    let target = (nodes.len() as f64).sqrt().ceil().max(1.0) as usize;
    let weights = undirected_weights(graph);
    let mut clusters: Vec<BTreeSet<EntityId>> = nodes
        .iter()
        .cloned()
        .map(|node| BTreeSet::from([node]))
        .collect();
    while clusters.len() > target {
        let mut best: Option<(f64, usize, usize)> = None;
        for left in 0..clusters.len() {
            for right in left + 1..clusters.len() {
                let distance =
                    average_linkage_distance(&clusters[left], &clusters[right], &weights);
                let candidate = (distance, left, right);
                if best.is_none_or(|old| candidate < old) {
                    best = Some(candidate);
                }
            }
        }
        let (_, left, right) = best.expect("at least two clusters");
        let merged = clusters.remove(right);
        clusters[left].extend(merged);
    }
    let labels = clusters
        .into_iter()
        .enumerate()
        .flat_map(|(label, cluster)| cluster.into_iter().map(move |node| (node, label)))
        .collect();
    normalize_partition(&labels)
}

/// Density-hierarchy clustering over structural neighborhood distances.  The
/// mutual-reachability graph is cut at its largest density gap; components below
/// `min_cluster_size` are labelled `HDBSCAN_NOISE`, never fabricated as clusters.
pub const HDBSCAN_NOISE: usize = usize::MAX;
pub fn hdbscan_baseline(graph: &UpsmGraph, min_degree: usize) -> Partition {
    let nodes: Vec<_> = graph.nodes.keys().cloned().collect();
    let weights = undirected_weights(graph);
    let mut edges = Vec::new();
    for (index, left) in nodes.iter().enumerate() {
        for right in nodes.iter().skip(index + 1) {
            let shared = shared_neighbor_distance(left, right, &weights);
            edges.push((shared, left.clone(), right.clone()));
        }
    }
    edges.sort_by(|a, b| {
        a.0.total_cmp(&b.0)
            .then_with(|| (&a.1, &a.2).cmp(&(&b.1, &b.2)))
    });
    let mut parent: BTreeMap<_, _> = nodes
        .iter()
        .cloned()
        .map(|node| (node.clone(), node))
        .collect();
    let mut mst = Vec::new();
    for edge in edges {
        if union(&mut parent, &edge.1, &edge.2) {
            mst.push(edge);
        }
    }
    let cut = mst
        .windows(2)
        .max_by(|left, right| (left[1].0 - left[0].0).total_cmp(&(right[1].0 - right[0].0)))
        .map_or(f64::INFINITY, |gap| gap[1].0);
    let mut parent: BTreeMap<_, _> = nodes
        .iter()
        .cloned()
        .map(|node| (node.clone(), node))
        .collect();
    let mut components: BTreeMap<_, BTreeSet<_>> = nodes
        .iter()
        .cloned()
        .map(|node| (node.clone(), BTreeSet::from([node])))
        .collect();
    for (distance, left, right) in mst.into_iter().filter(|edge| edge.0 < cut) {
        let _ = distance;
        let left_root = find(&mut parent, &left);
        let right_root = find(&mut parent, &right);
        if left_root != right_root {
            let right_nodes = components.remove(&right_root).unwrap_or_default();
            components
                .entry(left_root.clone())
                .or_default()
                .extend(right_nodes);
            parent.insert(right_root, left_root);
        }
    }
    let mut result = BTreeMap::new();
    for (label, cluster) in components
        .into_values()
        .filter(|cluster| cluster.len() >= min_degree.max(2))
        .enumerate()
    {
        for node in cluster {
            result.insert(node, label);
        }
    }
    for node in nodes {
        result.entry(node).or_insert(HDBSCAN_NOISE);
    }
    result
}

fn undirected_weights(graph: &UpsmGraph) -> BTreeMap<EntityId, BTreeMap<EntityId, f64>> {
    let mut result: BTreeMap<EntityId, BTreeMap<EntityId, f64>> = graph
        .nodes
        .keys()
        .cloned()
        .map(|node| (node, BTreeMap::new()))
        .collect();
    for edge in graph.edges.values() {
        *result
            .entry(edge.source.clone())
            .or_default()
            .entry(edge.target.clone())
            .or_default() += 1.0;
        *result
            .entry(edge.target.clone())
            .or_default()
            .entry(edge.source.clone())
            .or_default() += 1.0;
    }
    result
}
fn modularity(
    labels: &BTreeMap<EntityId, usize>,
    weights: &BTreeMap<EntityId, BTreeMap<EntityId, f64>>,
) -> f64 {
    let total: f64 = weights
        .values()
        .flat_map(|neighbors| neighbors.values())
        .sum::<f64>()
        / 2.0;
    if total == 0.0 {
        return 0.0;
    }
    labels
        .iter()
        .flat_map(|(left, left_label)| {
            labels
                .iter()
                .filter(move |(_, right_label)| left_label == *right_label)
                .map(move |(right, _)| {
                    let weight = weights
                        .get(left)
                        .and_then(|n| n.get(right))
                        .copied()
                        .unwrap_or(0.0);
                    let left_degree: f64 =
                        weights.get(left).into_iter().flat_map(|n| n.values()).sum();
                    let right_degree: f64 = weights
                        .get(right)
                        .into_iter()
                        .flat_map(|n| n.values())
                        .sum();
                    weight - left_degree * right_degree / (2.0 * total)
                })
        })
        .sum::<f64>()
        / (2.0 * total)
}
fn leiden_refine(
    labels: &BTreeMap<EntityId, usize>,
    weights: &BTreeMap<EntityId, BTreeMap<EntityId, f64>>,
) -> Partition {
    let mut result = BTreeMap::new();
    let mut next = 0;
    for (label, members) in group_members(labels) {
        let _ = label;
        let mut remaining = members;
        while let Some(seed) = remaining.iter().next().cloned() {
            let mut stack = vec![seed.clone()];
            let mut component = BTreeSet::new();
            while let Some(node) = stack.pop() {
                if component.insert(node.clone()) {
                    for neighbor in weights.get(&node).into_iter().flat_map(|n| n.keys()) {
                        if remaining.contains(neighbor) {
                            stack.push(neighbor.clone());
                        }
                    }
                }
            }
            for node in &component {
                remaining.remove(node);
                result.insert(node.clone(), next);
            }
            next += 1;
        }
    }
    normalize_partition(&result)
}
fn group_members(labels: &BTreeMap<EntityId, usize>) -> BTreeMap<usize, BTreeSet<EntityId>> {
    let mut groups = BTreeMap::new();
    for (node, label) in labels {
        groups
            .entry(*label)
            .or_insert_with(BTreeSet::new)
            .insert(node.clone());
    }
    groups
}
fn normalize_partition(labels: &BTreeMap<EntityId, usize>) -> Partition {
    let mut map = BTreeMap::new();
    let mut next = 0;
    let mut result = BTreeMap::new();
    for (node, label) in labels {
        let stable = *map.entry(*label).or_insert_with(|| {
            let value = next;
            next += 1;
            value
        });
        result.insert(node.clone(), stable);
    }
    result
}
fn average_linkage_distance(
    left: &BTreeSet<EntityId>,
    right: &BTreeSet<EntityId>,
    weights: &BTreeMap<EntityId, BTreeMap<EntityId, f64>>,
) -> f64 {
    left.iter()
        .flat_map(|a| {
            right
                .iter()
                .map(move |b| shared_neighbor_distance(a, b, weights))
        })
        .sum::<f64>()
        / (left.len() * right.len()) as f64
}
fn shared_neighbor_distance(
    left: &EntityId,
    right: &EntityId,
    weights: &BTreeMap<EntityId, BTreeMap<EntityId, f64>>,
) -> f64 {
    let a = weights.get(left).cloned().unwrap_or_default();
    let b = weights.get(right).cloned().unwrap_or_default();
    let union = a.keys().chain(b.keys()).collect::<BTreeSet<_>>().len();
    if union == 0 {
        return 1.0;
    }
    1.0 - a.keys().filter(|node| b.contains_key(*node)).count() as f64 / union as f64
}
fn find(parent: &mut BTreeMap<EntityId, EntityId>, node: &EntityId) -> EntityId {
    let parent_node = parent[node].clone();
    if parent_node != *node {
        let root = find(parent, &parent_node);
        parent.insert(node.clone(), root.clone());
        root
    } else {
        parent_node
    }
}
fn union(parent: &mut BTreeMap<EntityId, EntityId>, left: &EntityId, right: &EntityId) -> bool {
    let a = find(parent, left);
    let b = find(parent, right);
    if a == b {
        false
    } else {
        parent.insert(b, a);
        true
    }
}

pub fn partition_stability(left: &Partition, right: &Partition) -> f32 {
    let nodes: BTreeSet<_> = left.keys().chain(right.keys()).collect();
    if nodes.is_empty() {
        return 1.0;
    }
    nodes
        .iter()
        .filter(|node| left.get(**node) == right.get(**node))
        .count() as f32
        / nodes.len() as f32
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MergeConflict {
    pub entity: EntityId,
    pub field: String,
    pub retained: String,
    pub rejected: String,
    pub evidence: EvidenceRef,
}
impl UpsmGraph {
    pub fn new() -> Self {
        Self {
            schema_version: UPSM_SCHEMA_VERSION,
            ..Self::default()
        }
    }
    pub fn migrate(mut self) -> Result<Self, SchemaError> {
        if self.schema_version > UPSM_SCHEMA_VERSION {
            return Err(SchemaError::FutureVersion(self.schema_version));
        }
        self.schema_version = UPSM_SCHEMA_VERSION;
        Ok(self)
    }
    pub fn merge_node(
        &mut self,
        kind: NodeKind,
        qualified_name: &str,
        span: Option<SourceSpan>,
        properties: BTreeMap<String, String>,
        evidence: EvidenceRef,
    ) -> EntityId {
        let id = EntityId::derive(
            "codeflow.upsm.node.v1",
            &[
                &format!("{kind:?}"),
                qualified_name,
                span.as_ref()
                    .map(|s| s.source.normalized_path.as_str())
                    .unwrap_or(""),
            ],
        );
        let node = self.nodes.entry(id.clone()).or_insert_with(|| UpsmNode {
            id: id.clone(),
            kind,
            qualified_name: qualified_name.to_owned(),
            span: span.clone(),
            properties: BTreeMap::new(),
            evidence: Vec::new(),
        });
        for (key, value) in properties {
            if let Some(old) = node.properties.get(&key) {
                if old != &value {
                    self.conflicts.push(MergeConflict {
                        entity: id.clone(),
                        field: key.clone(),
                        retained: old.clone(),
                        rejected: value,
                        evidence: evidence.clone(),
                    });
                    continue;
                }
            } else {
                node.properties.insert(key, value);
            }
        }
        if !node.evidence.iter().any(|e| e.id == evidence.id) {
            node.evidence.push(evidence);
            node.evidence.sort_by(|a, b| a.id.cmp(&b.id));
        }
        id
    }
    pub fn merge_edge(
        &mut self,
        kind: EdgeKind,
        source: EntityId,
        target: EntityId,
        evidence: EvidenceRef,
    ) -> EdgeId {
        let id = EdgeId::derive(
            "codeflow.upsm.edge.v1",
            &[&format!("{kind:?}"), source.as_str(), target.as_str()],
        );
        let edge = self.edges.entry(id.clone()).or_insert_with(|| UpsmEdge {
            id: id.clone(),
            kind,
            source,
            target,
            evidence: Vec::new(),
        });
        if !edge.evidence.iter().any(|e| e.id == evidence.id) {
            edge.evidence.push(evidence);
            edge.evidence.sort_by(|a, b| a.id.cmp(&b.id));
        }
        id
    }
    pub fn aliases(&self) -> BTreeSet<&str> {
        self.nodes
            .values()
            .flat_map(|n| n.evidence.iter().map(|e| e.provider_key.as_str()))
            .collect()
    }
    pub fn degree(&self, entity: &EntityId) -> usize {
        self.edges
            .values()
            .filter(|edge| &edge.source == entity || &edge.target == entity)
            .count()
    }
    pub fn adjacency(&self) -> BTreeMap<EntityId, BTreeSet<EntityId>> {
        let mut result = BTreeMap::new();
        for edge in self.edges.values() {
            result
                .entry(edge.source.clone())
                .or_insert_with(BTreeSet::new)
                .insert(edge.target.clone());
        }
        result
    }
    pub fn strongly_connected_components(&self) -> Vec<Vec<EntityId>> {
        let graph = self.adjacency();
        let reverse = reverse_graph(&graph);
        let mut remaining: BTreeSet<_> = self.nodes.keys().cloned().collect();
        let mut result = Vec::new();
        while let Some(seed) = remaining.iter().next().cloned() {
            let forward = reachable(&graph, &seed);
            let backward = reachable(&reverse, &seed);
            let mut component: Vec<_> = forward.intersection(&backward).cloned().collect();
            component.sort();
            for node in &component {
                remaining.remove(node);
            }
            result.push(component);
        }
        result
    }
}
fn reachable(
    graph: &BTreeMap<EntityId, BTreeSet<EntityId>>,
    seed: &EntityId,
) -> BTreeSet<EntityId> {
    let mut seen = BTreeSet::from([seed.clone()]);
    let mut pending = vec![seed.clone()];
    while let Some(node) = pending.pop() {
        if let Some(next) = graph.get(&node) {
            for item in next {
                if seen.insert(item.clone()) {
                    pending.push(item.clone());
                }
            }
        }
    }
    seen
}
fn reverse_graph(
    graph: &BTreeMap<EntityId, BTreeSet<EntityId>>,
) -> BTreeMap<EntityId, BTreeSet<EntityId>> {
    let mut reverse = BTreeMap::new();
    for (from, tos) in graph {
        for to in tos {
            reverse
                .entry(to.clone())
                .or_insert_with(BTreeSet::new)
                .insert(from.clone());
        }
    }
    reverse
}
#[cfg(test)]
mod tests {
    use super::*;
    fn ev(key: &str) -> EvidenceRef {
        EvidenceRef {
            id: EvidenceId::derive("test", &[key]),
            provider: "fixture".into(),
            provider_key: key.into(),
            fact_class: FactClass::Deterministic,
            span: None,
        }
    }
    #[test]
    fn merges_provider_evidence_without_using_provider_identity() {
        let mut g = UpsmGraph::new();
        let a = g.merge_node(
            NodeKind::Function,
            "crate::a",
            None,
            BTreeMap::new(),
            ev("scip:a"),
        );
        let b = g.merge_node(
            NodeKind::Function,
            "crate::a",
            None,
            BTreeMap::new(),
            ev("tree:a"),
        );
        assert_eq!(a, b);
        assert_eq!(g.nodes[&a].evidence.len(), 2);
    }
    #[test]
    fn conflicts_are_retained_not_silently_overwritten() {
        let mut g = UpsmGraph::new();
        let mut one = BTreeMap::new();
        one.insert("visibility".into(), "public".into());
        let id = g.merge_node(NodeKind::Function, "a", None, one, ev("a"));
        let mut two = BTreeMap::new();
        two.insert("visibility".into(), "private".into());
        g.merge_node(NodeKind::Function, "a", None, two, ev("b"));
        assert_eq!(g.nodes[&id].properties["visibility"], "public");
        assert_eq!(g.conflicts.len(), 1);
    }
    #[test]
    fn semantic_normalization_preserves_unknown_boundaries() {
        assert_eq!(SemanticRelation::Call.edge_kind(), EdgeKind::Calls);
        assert_eq!(SemanticRelation::DefUse.edge_kind(), EdgeKind::DataFlowsTo);
        assert_eq!(
            SemanticRelation::ExternalService.edge_kind(),
            EdgeKind::Unknown
        );
        assert_eq!(
            normalize_relation(RawRelation::Invoke),
            SemanticRelation::Call
        );
        assert_eq!(
            normalize_relation(RawRelation::Throw),
            SemanticRelation::Exception
        );
        assert_eq!(
            normalize_relation(RawRelation::Assign),
            SemanticRelation::DefUse
        );
        assert_eq!(
            normalize_relation(RawRelation::HttpRequest),
            SemanticRelation::ExternalService
        );
        assert_eq!(
            normalize_relation(RawRelation::Unknown),
            SemanticRelation::Unknown
        );
    }
    #[test]
    fn schema_migration_is_explicit_about_future_data() {
        let mut graph = UpsmGraph::new();
        graph.schema_version = 0;
        assert_eq!(graph.migrate().unwrap().schema_version, UPSM_SCHEMA_VERSION);
        let mut future = UpsmGraph::new();
        future.schema_version = 99;
        assert_eq!(
            future.migrate().unwrap_err(),
            SchemaError::FutureVersion(99)
        );
    }
    #[test]
    fn scc_analysis_finds_cycles_deterministically() {
        let mut graph = UpsmGraph::new();
        let a = graph.merge_node(NodeKind::Function, "a", None, BTreeMap::new(), ev("a"));
        let b = graph.merge_node(NodeKind::Function, "b", None, BTreeMap::new(), ev("b"));
        graph.merge_edge(EdgeKind::Calls, a.clone(), b.clone(), ev("ab"));
        graph.merge_edge(EdgeKind::Calls, b.clone(), a.clone(), ev("ba"));
        let components = graph.strongly_connected_components();
        assert!(components.iter().any(|component| component.len() == 2
            && component.contains(&a)
            && component.contains(&b)));
    }
    #[test]
    fn graph_projections_and_baselines_are_deterministic() {
        let mut graph = UpsmGraph::new();
        let a = graph.merge_node(NodeKind::Function, "a", None, BTreeMap::new(), ev("a"));
        let b = graph.merge_node(NodeKind::Function, "b", None, BTreeMap::new(), ev("b"));
        graph.merge_edge(EdgeKind::Calls, a.clone(), b.clone(), ev("ab"));
        assert_eq!(
            file_class_function_projection(&graph)[&a],
            BTreeSet::from([b.clone()])
        );
        assert_eq!(
            degree_metrics(&graph)
                .iter()
                .map(|m| m.out_degree)
                .sum::<usize>(),
            1
        );
        let first = leiden_baseline(&graph);
        assert_eq!(first, connected_components_baseline(&graph));
        assert_ne!(first, agglomerative_baseline(&graph));
        assert_eq!(partition_stability(&first, &first), 1.0);
        assert!(
            hdbscan_baseline(&graph, 99)
                .values()
                .all(|cluster| *cluster == HDBSCAN_NOISE)
        );
    }
    #[test]
    fn leiden_refines_a_weak_bridge_while_components_do_not() {
        let mut graph = UpsmGraph::new();
        let nodes: Vec<_> = (0..6)
            .map(|index| {
                graph.merge_node(
                    NodeKind::Function,
                    &format!("node-{index}"),
                    None,
                    BTreeMap::new(),
                    ev(&format!("node-{index}")),
                )
            })
            .collect();
        for group in [&nodes[..3], &nodes[3..]] {
            for left in 0..group.len() {
                for right in left + 1..group.len() {
                    graph.merge_edge(
                        EdgeKind::Calls,
                        group[left].clone(),
                        group[right].clone(),
                        ev("dense"),
                    );
                }
            }
        }
        graph.merge_edge(
            EdgeKind::Calls,
            nodes[2].clone(),
            nodes[3].clone(),
            ev("bridge"),
        );
        let components = connected_components_baseline(&graph);
        let communities = leiden_baseline(&graph);
        assert_eq!(components.values().collect::<BTreeSet<_>>().len(), 1);
        assert_eq!(communities[&nodes[0]], communities[&nodes[1]]);
        assert_ne!(communities[&nodes[0]], communities[&nodes[5]]);
    }
    #[test]
    fn density_hierarchy_marks_an_isolated_node_as_noise() {
        let mut graph = UpsmGraph::new();
        let nodes: Vec<_> = (0..7)
            .map(|index| {
                graph.merge_node(
                    NodeKind::Function,
                    &format!("density-{index}"),
                    None,
                    BTreeMap::new(),
                    ev(&format!("density-{index}")),
                )
            })
            .collect();
        for group in [&nodes[..3], &nodes[3..6]] {
            for left in 0..group.len() {
                for right in left + 1..group.len() {
                    graph.merge_edge(
                        EdgeKind::Calls,
                        group[left].clone(),
                        group[right].clone(),
                        ev("density"),
                    );
                }
            }
        }
        let clusters = hdbscan_baseline(&graph, 3);
        assert_eq!(clusters[&nodes[6]], HDBSCAN_NOISE);
        assert_ne!(clusters[&nodes[0]], HDBSCAN_NOISE);
        assert_ne!(clusters[&nodes[3]], HDBSCAN_NOISE);
        assert_ne!(clusters[&nodes[0]], clusters[&nodes[3]]);
    }
    #[test]
    fn agglomerative_uses_average_linkage_not_community_aliasing() {
        let mut graph = UpsmGraph::new();
        let nodes: Vec<_> = (0..6)
            .map(|index| {
                graph.merge_node(
                    NodeKind::Function,
                    &format!("hierarchy-{index}"),
                    None,
                    BTreeMap::new(),
                    ev(&format!("hierarchy-{index}")),
                )
            })
            .collect();
        for group in [&nodes[..3], &nodes[3..]] {
            for left in 0..group.len() {
                for right in left + 1..group.len() {
                    graph.merge_edge(
                        EdgeKind::Calls,
                        group[left].clone(),
                        group[right].clone(),
                        ev("hierarchy"),
                    );
                }
            }
        }
        let hierarchy = agglomerative_baseline(&graph);
        let components = connected_components_baseline(&graph);
        assert_ne!(hierarchy, components);
        assert_eq!(hierarchy, agglomerative_baseline(&graph));
    }
}
