use uuid::Uuid;
use std::collections::HashMap;

// ─── CanonBlockType ───────────────────────────────────────────────────────────
//
// Tipo formal de cada bloco — enum fechado e exaustivo.
// Separado de BlockDef (runtime) para ser estável e versionável.

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CanonBlockType {
    // Layout
    Layout,
    Section,
    Card,
    Dialog,
    Drawer,
    Popover,
    // Page
    Header,
    Footer,
    PageHeader,
    // Navigation
    Breadcrumb,
    Toolbar,
    ButtonGroup,
    // Form
    Form,
    Field,
    FormActions,
    // Content
    Alert,
    Callout,
    StatCard,
    EmptyState,
    DataTable,
    CodeBlock,
    List,
    Skeleton,
    Table,
    // Overlay
    CommandPanel,
    // Slot (região estrutural do layout)
    Slot { name: String },
}

impl CanonBlockType {
    pub fn from_id(id: &str) -> Option<Self> {
        match id {
            "card"          => Some(Self::Card),
            "section"       => Some(Self::Section),
            "dialog"        => Some(Self::Dialog),
            "drawer"        => Some(Self::Drawer),
            "popover"       => Some(Self::Popover),
            "header"        => Some(Self::Header),
            "footer"        => Some(Self::Footer),
            "page-header"   => Some(Self::PageHeader),
            "breadcrumb"    => Some(Self::Breadcrumb),
            "toolbar"       => Some(Self::Toolbar),
            "button-group"  => Some(Self::ButtonGroup),
            "form"          => Some(Self::Form),
            "field"         => Some(Self::Field),
            "form-actions"  => Some(Self::FormActions),
            "alert"         => Some(Self::Alert),
            "callout"       => Some(Self::Callout),
            "stat-card"     => Some(Self::StatCard),
            "empty-state"   => Some(Self::EmptyState),
            "data-table"    => Some(Self::DataTable),
            "code-block"    => Some(Self::CodeBlock),
            "list"          => Some(Self::List),
            "skeleton"      => Some(Self::Skeleton),
            "table"         => Some(Self::Table),
            "command-panel" => Some(Self::CommandPanel),
            _               => None,
        }
    }

    pub fn to_id(&self) -> &'static str {
        match self {
            Self::Card          => "card",
            Self::Section       => "section",
            Self::Dialog        => "dialog",
            Self::Drawer        => "drawer",
            Self::Popover       => "popover",
            Self::Header        => "header",
            Self::Footer        => "footer",
            Self::PageHeader    => "page-header",
            Self::Breadcrumb    => "breadcrumb",
            Self::Toolbar       => "toolbar",
            Self::ButtonGroup   => "button-group",
            Self::Form          => "form",
            Self::Field         => "field",
            Self::FormActions   => "form-actions",
            Self::Alert         => "alert",
            Self::Callout       => "callout",
            Self::StatCard      => "stat-card",
            Self::EmptyState    => "empty-state",
            Self::DataTable     => "data-table",
            Self::CodeBlock     => "code-block",
            Self::List          => "list",
            Self::Skeleton      => "skeleton",
            Self::Table         => "table",
            Self::CommandPanel  => "command-panel",
            Self::Layout        => "layout",
            Self::Slot { .. }   => "slot",
        }
    }
}

// ─── CanonProps ───────────────────────────────────────────────────────────────

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CanonProps(pub HashMap<String, serde_json::Value>);

impl CanonProps {
    pub fn empty() -> Self { Self(HashMap::new()) }
    pub fn get(&self, key: &str) -> Option<&serde_json::Value> { self.0.get(key) }
    pub fn set(&mut self, key: impl Into<String>, value: impl Into<serde_json::Value>) {
        self.0.insert(key.into(), value.into());
    }
}

// ─── CanonNode (Modelo Canônico Recursivo) ────────────────────────────────────
//
// Este é o modelo oficial de persistência e export.
// Nunca usado diretamente no runtime Leptos — apenas para:
//   - Serialização JSON
//   - Export para código Rust
//   - Validação estrutural
//   - Snapshot / diff / versionamento

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CanonNode {
    pub id: Uuid,
    pub block: CanonBlockType,
    pub props: CanonProps,
    pub children: Vec<CanonNode>,
}

impl CanonNode {
    pub fn new(block: CanonBlockType) -> Self {
        Self {
            id: Uuid::new_v4(),
            block,
            props: CanonProps::empty(),
            children: vec![],
        }
    }

    pub fn with_id(id: Uuid, block: CanonBlockType) -> Self {
        Self { id, block, props: CanonProps::empty(), children: vec![] }
    }

    pub fn add_child(&mut self, child: CanonNode) {
        self.children.push(child);
    }

    /// Visitor imutável — percorre toda a árvore
    pub fn visit<F: Fn(&CanonNode)>(&self, f: &F) {
        f(self);
        for child in &self.children {
            child.visit(f);
        }
    }

    /// Visitor mutável
    pub fn visit_mut<F: FnMut(&mut CanonNode)>(&mut self, f: &mut F) {
        f(self);
        for child in &mut self.children {
            child.visit_mut(f);
        }
    }

    /// Encontra nó por ID
    pub fn find(&self, id: Uuid) -> Option<&CanonNode> {
        if self.id == id { return Some(self); }
        self.children.iter().find_map(|c| c.find(id))
    }

    /// Conta total de nós
    pub fn count(&self) -> usize {
        1 + self.children.iter().map(|c| c.count()).sum::<usize>()
    }

    /// Profundidade máxima
    pub fn depth(&self) -> usize {
        if self.children.is_empty() { 1 }
        else { 1 + self.children.iter().map(|c| c.depth()).max().unwrap_or(0) }
    }
}

// ─── CanonDocument ────────────────────────────────────────────────────────────
//
// Container raiz de um layout completo.
// Isso é o que se serializa, versiona e persiste.

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CanonDocument {
    pub id: Uuid,
    pub layout: String,
    pub version: u32,
    pub nodes: Vec<CanonNode>, // raízes (slots do layout)
}

impl CanonDocument {
    pub fn new(layout: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            layout: layout.into(),
            version: 1,
            nodes: vec![],
        }
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    pub fn total_nodes(&self) -> usize {
        self.nodes.iter().map(|n| n.count()).sum()
    }
}

// ─── Conversão Flat ↔ Tree ────────────────────────────────────────────────────

use super::types::{Node, NodeKind};

/// Converte Vec<Node> flat (runtime) para Vec<CanonNode> recursivo (canônico)
pub fn build_tree(flat: &[Node]) -> Vec<CanonNode> {
    // Raízes são slots (parent_id = None)
    let roots: Vec<&Node> = flat.iter()
        .filter(|n| n.parent_id.is_none())
        .collect();

    let mut sorted_roots: Vec<&Node> = roots;
    sorted_roots.sort_by_key(|n| n.index);

    sorted_roots.iter().map(|root| build_canon_node(root, flat)).collect()
}

fn build_canon_node(node: &Node, flat: &[Node]) -> CanonNode {
    let block = match &node.kind {
        NodeKind::Slot { name } => CanonBlockType::Slot { name: name.to_string() },
        NodeKind::Block { def } => CanonBlockType::from_id(def.id)
            .unwrap_or(CanonBlockType::Section),
    };

    let mut canon = CanonNode::with_id(node.id, block);

    // Filhos ordenados por index
    let mut children: Vec<&Node> = flat.iter()
        .filter(|n| n.parent_id == Some(node.id))
        .collect();
    children.sort_by_key(|n| n.index);

    canon.children = children.iter().map(|c| build_canon_node(c, flat)).collect();
    canon
}

/// Converte Vec<CanonNode> recursivo para Vec<Node> flat (runtime)
pub fn flatten_tree(nodes: &[CanonNode]) -> Vec<Node> {
    let mut flat = vec![];
    for (i, node) in nodes.iter().enumerate() {
        flatten_node(node, None, i, &mut flat);
    }
    flat
}

fn flatten_node(canon: &CanonNode, parent_id: Option<Uuid>, index: usize, flat: &mut Vec<Node>) {
    let kind = match &canon.block {
        CanonBlockType::Slot { name } => {
            NodeKind::Slot { name: string_to_static(name) }
        },
        block_type => {
            use super::types::{BlockDef, NodeCategory};
            let id = block_type.to_id();
            NodeKind::Block {
                def: BlockDef {
                    id,
                    label: id,
                    icon: "▭",
                    category: NodeCategory::Content,
                    is_container: !canon.children.is_empty(),
                }
            }
        }
    };

    let node = Node { id: canon.id, kind, parent_id, index };
    flat.push(node);

    for (i, child) in canon.children.iter().enumerate() {
        flatten_node(child, Some(canon.id), i, flat);
    }
}

/// Helper — converte String para &'static str via leak
/// Usado apenas no flatten, que é operação ocasional
fn string_to_static(s: &str) -> &'static str {
    Box::leak(s.to_string().into_boxed_str())
}
