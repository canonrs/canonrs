use uuid::Uuid;
use rs_canonrs::domain::{CanonDocument, CanonNode, CanonBlockType};

pub struct WizardConfig {
    pub layout: String,
    pub name: String,
}

impl Default for WizardConfig {
    fn default() -> Self {
        Self { layout: "Dashboard".into(), name: "New Document".into() }
    }
}

pub fn generate_document(config: WizardConfig) -> CanonDocument {
    let mut header_slot = CanonNode::new(CanonBlockType::Slot { name: "header".into() });
    let mut content_slot = CanonNode::new(CanonBlockType::Slot { name: "content".into() });

    content_slot.children.push(CanonNode::new(CanonBlockType::Card));
    content_slot.children.push(CanonNode::new(CanonBlockType::StatCard));
    content_slot.children.push(CanonNode::new(CanonBlockType::Alert));

    CanonDocument {
        id: Uuid::new_v4(),
        layout: config.layout,
        version: 1,
        nodes: vec![header_slot, content_slot],
    }
}
