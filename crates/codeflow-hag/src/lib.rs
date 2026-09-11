//! Human Abstraction Graph, deliberately separate from implementation UPSM truth.
use codeflow_core::{Confidence, EntityId, EvidenceId, FactClass};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
pub const HAG_SCHEMA_VERSION: u16 = 1;
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HagKind {
    Feature,
    Subsystem,
    Workflow,
    CrossCutting,
    SharedInfrastructure,
    Unknown,
}
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HagRelation {
    Contains,
    DependsOn,
    Uses,
    Implements,
    CrossCuts,
    Unknown,
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct HagEvidence {
    pub id: EvidenceId,
    pub fact_class: FactClass,
    pub provider: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HagNode {
    pub id: EntityId,
    pub kind: HagKind,
    pub label: String,
    pub parent: Option<EntityId>,
    pub implementations: BTreeSet<EntityId>,
    pub evidence: Vec<HagEvidence>,
    pub confidence: Confidence,
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct HagConstraint {
    pub node: EntityId,
    pub locked: bool,
    pub reason: String,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct HagGraph {
    pub schema_version: u16,
    pub nodes: BTreeMap<EntityId, HagNode>,
    pub relations: BTreeSet<(EntityId, EntityId, HagRelation)>,
    pub constraints: Vec<HagConstraint>,
}
impl HagGraph {
    pub fn new() -> Self {
        Self {
            schema_version: HAG_SCHEMA_VERSION,
            ..Self::default()
        }
    }
    pub fn add_node(
        &mut self,
        kind: HagKind,
        label: &str,
        confidence: Confidence,
        evidence: HagEvidence,
    ) -> EntityId {
        let id = EntityId::derive("codeflow.hag.node.v1", &[&format!("{kind:?}"), label]);
        self.nodes
            .entry(id.clone())
            .or_insert_with(|| HagNode {
                id: id.clone(),
                kind,
                label: label.into(),
                parent: None,
                implementations: BTreeSet::new(),
                evidence: vec![],
                confidence,
            })
            .evidence
            .push(evidence);
        id
    }
    pub fn link_implementation(&mut self, hag: &EntityId, implementation: EntityId) -> bool {
        self.nodes
            .get_mut(hag)
            .is_some_and(|node| node.implementations.insert(implementation))
    }
    pub fn add_relation(&mut self, from: EntityId, to: EntityId, relation: HagRelation) {
        self.relations.insert((from, to, relation));
    }
    pub fn set_parent(&mut self, child: &EntityId, parent: EntityId) -> bool {
        if child == &parent {
            return false;
        }
        let mut cursor = Some(parent.clone());
        while let Some(node) = cursor {
            if &node == child {
                return false;
            }
            cursor = self.nodes.get(&node).and_then(|value| value.parent.clone());
        }
        if let Some(node) = self.nodes.get_mut(child) {
            node.parent = Some(parent);
            true
        } else {
            false
        }
    }
    pub fn cross_cutting_nodes(&self) -> Vec<EntityId> {
        self.nodes
            .values()
            .filter(|node| {
                matches!(
                    node.kind,
                    HagKind::CrossCutting | HagKind::SharedInfrastructure
                )
            })
            .map(|node| node.id.clone())
            .collect()
    }
    pub fn lock(&mut self, node: EntityId, reason: &str) {
        self.constraints.push(HagConstraint {
            node,
            locked: true,
            reason: reason.into(),
        });
    }
    pub fn migrate(mut self) -> Result<Self, u16> {
        if self.schema_version > HAG_SCHEMA_VERSION {
            return Err(self.schema_version);
        }
        self.schema_version = HAG_SCHEMA_VERSION;
        Ok(self)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn ev() -> HagEvidence {
        HagEvidence {
            id: EvidenceId::derive("hag", &["e"]),
            fact_class: FactClass::Reconstructed,
            provider: "baseline".into(),
        }
    }
    #[test]
    fn many_to_many_membership_is_preserved() {
        let mut g = HagGraph::new();
        let feature = g.add_node(
            HagKind::Feature,
            "checkout",
            Confidence::new(1.0, 0.5, 0.0).unwrap(),
            ev(),
        );
        assert!(g.link_implementation(&feature, EntityId::derive("impl", &["a"])));
        assert!(g.link_implementation(&feature, EntityId::derive("impl", &["b"])));
        assert_eq!(g.nodes[&feature].implementations.len(), 2);
    }
    #[test]
    fn manual_constraint_is_explicit() {
        let mut g = HagGraph::new();
        let id = g.add_node(HagKind::Unknown, "unknown", Confidence::unknown(), ev());
        g.lock(id.clone(), "reviewed");
        assert!(g.constraints.iter().any(|c| c.node == id && c.locked));
    }
    #[test]
    fn hierarchy_rejects_self_parent_and_exposes_cross_cutting_nodes() {
        let mut g = HagGraph::new();
        let parent = g.add_node(HagKind::Subsystem, "billing", Confidence::unknown(), ev());
        let child = g.add_node(
            HagKind::CrossCutting,
            "logging",
            Confidence::unknown(),
            ev(),
        );
        assert!(g.set_parent(&child, parent.clone()));
        assert!(!g.set_parent(&parent.clone(), parent));
        assert_eq!(g.cross_cutting_nodes(), vec![child]);
    }
    #[test]
    fn hierarchy_rejects_parent_cycles_and_future_schema() {
        let mut g = HagGraph::new();
        let a = g.add_node(HagKind::Subsystem, "a", Confidence::unknown(), ev());
        let b = g.add_node(HagKind::Subsystem, "b", Confidence::unknown(), ev());
        assert!(g.set_parent(&b, a.clone()));
        assert!(!g.set_parent(&a, b));
        let mut future = HagGraph::new();
        future.schema_version = 9;
        assert!(matches!(future.migrate(), Err(9)));
    }
}
