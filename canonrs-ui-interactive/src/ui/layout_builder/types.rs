use uuid::Uuid;

// ─── Layout ───────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ActiveLayout {
    Dashboard, Marketing, Fullscreen, SplitView, Wizard,
    Section, PageSingle, PageWithSidebar, PageWithAside, PageSidebarAndAside,
}

impl ActiveLayout {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Dashboard => "Dashboard", Self::Marketing => "Marketing",
            Self::Fullscreen => "Fullscreen", Self::SplitView => "Split View",
            Self::Wizard => "Wizard", Self::Section => "Section",
            Self::PageSingle => "Page — Single", Self::PageWithSidebar => "Page — Sidebar",
            Self::PageWithAside => "Page — Aside", Self::PageSidebarAndAside => "Page — Full",
        }
    }
    pub fn description(&self) -> &'static str {
        match self {
            Self::Dashboard => "App shell with content area",
            Self::Marketing => "Header + hero + main + footer",
            Self::Fullscreen => "Optional header + full canvas",
            Self::SplitView => "Left context + right action",
            Self::Wizard => "Header + stepper + content + footer",
            Self::Section => "Header + content + footer section",
            Self::PageSingle => "Content only",
            Self::PageWithSidebar => "Sidebar + content",
            Self::PageWithAside => "Content + aside",
            Self::PageSidebarAndAside => "Sidebar + content + aside",
        }
    }
    pub fn icon(&self) -> &'static str {
        match self {
            Self::Dashboard => "⬛", Self::Marketing => "🌐",
            Self::Fullscreen => "⬜", Self::SplitView => "◧",
            Self::Wizard => "📋", Self::Section => "▤",
            Self::PageSingle => "▭", Self::PageWithSidebar => "▐▭",
            Self::PageWithAside => "▭▌", Self::PageSidebarAndAside => "▐▭▌",
        }
    }
    pub fn all() -> Vec<ActiveLayout> {
        vec![
            Self::Dashboard, Self::Marketing, Self::Fullscreen,
            Self::SplitView, Self::Wizard, Self::Section,
            Self::PageSingle, Self::PageWithSidebar,
            Self::PageWithAside, Self::PageSidebarAndAside,
        ]
    }
    pub fn slots(&self) -> Vec<&'static str> {
        match self {
            Self::Dashboard => vec!["main"],
            Self::Marketing => vec!["hero", "main", "footer"],
            Self::Fullscreen => vec!["header", "main"],
            Self::SplitView => vec!["left", "right"],
            Self::Wizard => vec!["header", "stepper", "main", "footer"],
            Self::Section => vec!["header", "main", "footer"],
            Self::PageSingle => vec!["main"],
            Self::PageWithSidebar => vec!["sidebar", "main"],
            Self::PageWithAside => vec!["main", "aside"],
            Self::PageSidebarAndAside => vec!["sidebar", "main", "aside"],
        }
    }
}

// ─── Constraint Engine ────────────────────────────────────────────────────────

/// Categoria estrutural do bloco — define regras de hierarquia
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NodeCategory {
    /// Blocos estruturais que contêm outros blocos
    Layout,     // Section, Card, Dialog, Drawer
    /// Blocos de formulário
    Form,       // Form
    /// Elementos de formulário — só dentro de Form
    FormField,  // Field, FormActions
    /// Blocos de navegação
    Nav,        // Breadcrumb, Toolbar, ButtonGroup, PageHeader
    /// Blocos de conteúdo — folhas
    Content,    // Alert, Callout, StatCard, EmptyState, CodeBlock, List, Skeleton, Table, DataTable
    /// Blocos especiais de página
    Page,       // Header, Footer
    /// Overlay — só dentro de Layout
    Overlay,    // Popover, CommandPanel
}

/// Constraint: quais categorias um container aceita como filhos
pub fn allowed_children(category: NodeCategory) -> &'static [NodeCategory] {
    match category {
        NodeCategory::Layout => &[
            NodeCategory::Layout,
            NodeCategory::Form,
            NodeCategory::Nav,
            NodeCategory::Content,
            NodeCategory::Overlay,
        ],
        NodeCategory::Form => &[
            NodeCategory::FormField,
            NodeCategory::Nav,
            NodeCategory::Content,
        ],
        NodeCategory::FormField => &[],
        NodeCategory::Nav => &[],
        NodeCategory::Content => &[],
        NodeCategory::Page => &[
            NodeCategory::Nav,
            NodeCategory::Content,
        ],
        NodeCategory::Overlay => &[
            NodeCategory::Form,
            NodeCategory::Content,
            NodeCategory::Nav,
        ],
    }
}

/// Slot aceita qualquer categoria exceto FormField solto
pub fn slot_accepts(category: NodeCategory) -> bool {
    category != NodeCategory::FormField
}

// ─── BlockDef ─────────────────────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq)]
pub struct BlockDef {
    pub id: &'static str,
    pub label: &'static str,
    pub icon: &'static str,
    pub category: NodeCategory,
    /// Se true, renderiza DropZone interno
    pub is_container: bool,
}

impl BlockDef {
    pub fn can_accept(&self, child_category: NodeCategory) -> bool {
        if !self.is_container { return false; }
        allowed_children(self.category).contains(&child_category)
    }
}

// ─── Tree Node ────────────────────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq)]
pub enum NodeKind {
    Slot { name: &'static str },
    Block { def: BlockDef },
}

#[derive(Clone, Debug)]
pub struct Node {
    pub id: Uuid,
    pub kind: NodeKind,
    pub parent_id: Option<Uuid>,
    pub index: usize,
}

impl Node {
    pub fn slot(name: &'static str) -> Self {
        Self { id: Uuid::new_v4(), kind: NodeKind::Slot { name }, parent_id: None, index: 0 }
    }
    pub fn block(def: BlockDef, parent_id: Uuid, index: usize) -> Self {
        Self { id: Uuid::new_v4(), kind: NodeKind::Block { def }, parent_id: Some(parent_id), index }
    }
    pub fn is_container(&self) -> bool {
        match &self.kind {
            NodeKind::Slot { .. } => true,
            NodeKind::Block { def } => def.is_container,
        }
    }
    pub fn label(&self) -> &str {
        match &self.kind {
            NodeKind::Slot { name } => name,
            NodeKind::Block { def } => def.label,
        }
    }
    /// Verifica se este nó aceita um bloco como filho
    pub fn accepts(&self, child: &BlockDef) -> bool {
        match &self.kind {
            NodeKind::Slot { .. } => slot_accepts(child.category),
            NodeKind::Block { def } => def.can_accept(child.category),
        }
    }
}

// ─── Tree Operations ──────────────────────────────────────────────────────────

pub fn children_of(tree: &[Node], parent_id: Uuid) -> Vec<Node> {
    let mut nodes: Vec<Node> = tree.iter()
        .filter(|n| n.parent_id == Some(parent_id))
        .cloned()
        .collect();
    nodes.sort_by_key(|n| n.index);
    nodes
}

pub fn insert_node(tree: &mut Vec<Node>, node: Node) {
    let siblings: Vec<usize> = tree.iter().enumerate()
        .filter(|(_, n)| n.parent_id == node.parent_id && n.index >= node.index)
        .map(|(i, _)| i)
        .collect();
    for i in siblings { tree[i].index += 1; }
    tree.push(node);
}

pub fn remove_node(tree: &mut Vec<Node>, id: Uuid) {
    let mut to_remove = vec![id];
    loop {
        let before = to_remove.len();
        let children: Vec<Uuid> = tree.iter()
            .filter(|n| n.parent_id.map(|p| to_remove.contains(&p)).unwrap_or(false))
            .map(|n| n.id)
            .collect();
        to_remove.extend(children);
        if to_remove.len() == before { break; }
    }
    tree.retain(|n| !to_remove.contains(&n.id));
}

pub fn init_slots(layout: &ActiveLayout) -> Vec<Node> {
    layout.slots().into_iter().enumerate().map(|(i, name)| {
        Node { id: Uuid::new_v4(), kind: NodeKind::Slot { name }, parent_id: None, index: i }
    }).collect()
}

// ─── Canvas Mode ─────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum CanvasMode {
    Builder,
    Preview,
}

// ─── DragContext ───────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct DragContext {
    pub node_id: Option<Uuid>,
    pub block_def: Option<BlockDef>,
}

impl DragContext {
    pub fn empty() -> Self { Self { node_id: None, block_def: None } }
    pub fn is_dragging(&self) -> bool { self.block_def.is_some() }
}

// ─── Blocks ───────────────────────────────────────────────────────────────────

pub fn all_blocks() -> Vec<BlockDef> {
    use NodeCategory::*;
    vec![
        BlockDef { id: "header",        label: "Header",        icon: "▬",  category: Page,    is_container: false },
        BlockDef { id: "footer",        label: "Footer",        icon: "▬",  category: Page,    is_container: false },
        BlockDef { id: "card",          label: "Card",          icon: "▭",  category: Layout,  is_container: true  },
        BlockDef { id: "section",       label: "Section",       icon: "▤",  category: Layout,  is_container: true  },
        BlockDef { id: "dialog",        label: "Dialog",        icon: "◻",  category: Overlay, is_container: true  },
        BlockDef { id: "drawer",        label: "Drawer",        icon: "▷",  category: Overlay, is_container: true  },
        BlockDef { id: "form",          label: "Form",          icon: "📝", category: Form,    is_container: true  },
        BlockDef { id: "alert",         label: "Alert",         icon: "⚠",  category: Content, is_container: false },
        BlockDef { id: "callout",       label: "Callout",       icon: "💬", category: Content, is_container: false },
        BlockDef { id: "stat-card",     label: "Stat Card",     icon: "📊", category: Content, is_container: false },
        BlockDef { id: "empty-state",   label: "Empty State",   icon: "○",  category: Content, is_container: false },
        BlockDef { id: "data-table",    label: "Data Table",    icon: "⊞",  category: Content, is_container: false },
        BlockDef { id: "code-block",    label: "Code Block",    icon: "{}",  category: Content, is_container: false },
        BlockDef { id: "list",          label: "List",          icon: "≡",  category: Content, is_container: false },
        BlockDef { id: "skeleton",      label: "Skeleton",      icon: "░",  category: Content, is_container: false },
        BlockDef { id: "table",         label: "Table",         icon: "⊟",  category: Content, is_container: false },
        BlockDef { id: "toolbar",       label: "Toolbar",       icon: "⚙",  category: Nav,     is_container: false },
        BlockDef { id: "breadcrumb",    label: "Breadcrumb",    icon: "›",  category: Nav,     is_container: false },
        BlockDef { id: "button-group",  label: "Button Group",  icon: "⬚",  category: Nav,     is_container: false },
        BlockDef { id: "page-header",   label: "Page Header",   icon: "H",  category: Nav,     is_container: false },
        BlockDef { id: "field",         label: "Field",         icon: "▱",  category: FormField, is_container: false },
        BlockDef { id: "form-actions",  label: "Form Actions",  icon: "↵",  category: FormField, is_container: false },
        BlockDef { id: "popover",       label: "Popover",       icon: "◉",  category: Overlay, is_container: true  },
        BlockDef { id: "command-panel", label: "Command Panel", icon: "⌘",  category: Overlay, is_container: false },
    ]
}
