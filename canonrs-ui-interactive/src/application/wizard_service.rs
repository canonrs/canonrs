use uuid::Uuid;
use rs_canonrs::domain::{CanonDocument, CanonNode, CanonBlockType};
use rs_canonrs::domain::get_layout_def;

pub struct WizardConfig {
    pub layout: String,
    pub name: String,
}

impl Default for WizardConfig {
    fn default() -> Self {
        Self { layout: "dashboard".into(), name: "New Document".into() }
    }
}

pub fn generate_document(config: WizardConfig) -> CanonDocument {
    let layout_id = config.layout.to_lowercase();

    if let Some(def) = get_layout_def(&layout_id) {
        let nodes = def.regions.iter()
            .map(|r| CanonNode::new(CanonBlockType::Slot { name: r.id.to_string() }))
            .collect();
        return CanonDocument { id: Uuid::new_v4(), layout: layout_id, layout_version: def.version, version: 1, nodes };
    }

    CanonDocument::new(layout_id)
}


pub fn validate_document(doc: &rs_canonrs::domain::CanonDocument) -> Result<(), Vec<String>> {
    use rs_canonrs::domain::{CanonBlockType, CanonNode};
    use std::collections::{HashSet, HashMap};

    let Some(def) = get_layout_def(doc.layout.as_str()) else {
        return Err(vec![format!("Unknown layout: '{}'", doc.layout)]);
    };

    fn collect_slots(node: &CanonNode, acc: &mut Vec<String>) {
        if let CanonBlockType::Slot { name } = &node.block {
            acc.push(name.clone());
        }
        for child in &node.children { collect_slots(child, acc); }
    }

    let mut found = vec![];
    for n in &doc.nodes { collect_slots(n, &mut found); }

    let mut errors = vec![];

    let mut counts: HashMap<&String, usize> = HashMap::new();
    for r in &found { *counts.entry(r).or_insert(0) += 1; }
    for (region, count) in &counts {
        if *count > 1 { errors.push(format!("Duplicate region '{}'", region)); }
    }

    let doc_set: HashSet<String> = found.into_iter().collect();
    let def_set: HashSet<String> = def.regions.iter().map(|r| r.id.to_string()).collect();

    for r in def_set.difference(&doc_set) { errors.push(format!("Missing region '{}'", r)); }
    for r in doc_set.difference(&def_set) { errors.push(format!("Unknown region '{}'", r)); }

    // Checar layout_version
    if doc.layout_version < def.version {
        errors.push(format!("Layout version mismatch: doc=v{} def=v{} — migration needed", doc.layout_version, def.version));
    }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

/// Migration Engine — aplica migrações incrementais até atingir def.version
pub fn migrate_document(doc: CanonDocument) -> CanonDocument {
    let Some(def) = get_layout_def(doc.layout.as_str()) else { return doc; };
    if doc.layout_version >= def.version { return doc; }

    let mut current = doc;
    while current.layout_version < def.version {
        current = apply_migration(current, def);
    }
    current
}

fn apply_migration(
    mut doc: CanonDocument,
    def: &rs_canonrs::domain::LayoutDef,
) -> CanonDocument {
    use std::collections::HashSet;

    let next_version = doc.layout_version + 1;

    // Regiões presentes no doc
    let existing: HashSet<String> = doc.nodes.iter()
        .filter_map(|n| match &n.block {
            CanonBlockType::Slot { name } => Some(name.clone()),
            _ => None,
        })
        .collect();

    // Adicionar regiões faltantes (auto-heal)
    for region in def.regions {
        if !existing.contains(region.id) {
            leptos::logging::warn!("[Migration v{}] Adding missing region '{}'", next_version, region.id);
            doc.nodes.push(CanonNode::new(CanonBlockType::Slot { name: region.id.to_string() }));
        }
    }

    // Remover regiões extras
    let def_set: HashSet<&str> = def.regions.iter().map(|r| r.id).collect();
    doc.nodes.retain(|n| match &n.block {
        CanonBlockType::Slot { name } => def_set.contains(name.as_str()),
        _ => true,
    });

    doc.layout_version = next_version;
    doc
}

/// Structural Invariants — valida regras estruturais formais
/// 1. Slots apenas no root do documento
/// 2. Blocks apenas dentro de Slots
/// 3. Sem Slots aninhados dentro de Blocks
pub fn check_invariants(doc: &CanonDocument) -> Result<(), Vec<String>> {
    use rs_canonrs::domain::CanonBlockType;
    let mut errors = vec![];

    for node in &doc.nodes {
        match &node.block {
            // Root deve ser Slot
            CanonBlockType::Slot { name } => {
                // Filhos de Slot não podem ser Slots
                for child in &node.children {
                    if let CanonBlockType::Slot { name: inner } = &child.block {
                        errors.push(format!("Nested slot '{}' inside slot '{}'", inner, name));
                    }
                    // Recursivo: blocks dentro de blocks ok, slots dentro de blocks não
                    check_no_slots_in_blocks(child, &mut errors);
                }
            }
            // Root não pode ser Block
            other => {
                errors.push(format!("Root node must be Slot, found '{}'", other.to_id()));
            }
        }
    }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

fn check_no_slots_in_blocks(node: &rs_canonrs::domain::CanonNode, errors: &mut Vec<String>) {
    use rs_canonrs::domain::CanonBlockType;
    for child in &node.children {
        if let CanonBlockType::Slot { name } = &child.block {
            errors.push(format!("Slot '{}' found inside block — slots must be root only", name));
        } else {
            check_no_slots_in_blocks(child, errors);
        }
    }
}

/// Structural Fingerprint — hash determinístico do documento
/// Baseado em: layout_id + layout_version + regiões + árvore estrutural
pub fn structural_fingerprint(doc: &CanonDocument) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut h = DefaultHasher::new();
    doc.layout.hash(&mut h);
    doc.layout_version.hash(&mut h);

    fn hash_node(node: &rs_canonrs::domain::CanonNode, h: &mut DefaultHasher) {
        use rs_canonrs::domain::CanonBlockType;
        match &node.block {
            CanonBlockType::Slot { name } => { "slot".hash(h); name.hash(h); }
            other => { other.to_id().hash(h); }
        }
        node.id.hash(h);
        for child in &node.children { hash_node(child, h); }
    }

    for node in &doc.nodes { hash_node(node, &mut DefaultHasher::new()); }
    // hash da estrutura (sem IDs) para comparação semântica
    let mut structural = DefaultHasher::new();
    doc.layout.hash(&mut structural);
    doc.layout_version.hash(&mut structural);
    fn hash_structure(node: &rs_canonrs::domain::CanonNode, h: &mut DefaultHasher) {
        use rs_canonrs::domain::CanonBlockType;
        match &node.block {
            CanonBlockType::Slot { name } => { "slot".hash(h); name.hash(h); }
            other => { other.to_id().hash(h); }
        }
        for child in &node.children { hash_structure(child, h); }
    }
    for node in &doc.nodes { hash_structure(node, &mut structural); }

    structural.finish()
}
