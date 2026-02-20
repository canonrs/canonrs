#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ActiveLayout {
    Dashboard, Marketing, Fullscreen, SplitView, Wizard,
    Section, PageSingle, PageWithSidebar, PageWithAside, PageSidebarAndAside,
}

impl ActiveLayout {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Dashboard => "Dashboard",
            Self::Marketing => "Marketing",
            Self::Fullscreen => "Fullscreen",
            Self::SplitView => "Split View",
            Self::Wizard => "Wizard",
            Self::Section => "Section",
            Self::PageSingle => "Page — Single",
            Self::PageWithSidebar => "Page — Sidebar",
            Self::PageWithAside => "Page — Aside",
            Self::PageSidebarAndAside => "Page — Full",
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
            Self::Dashboard => "⬛",
            Self::Marketing => "🌐",
            Self::Fullscreen => "⬜",
            Self::SplitView => "◧",
            Self::Wizard => "📋",
            Self::Section => "▤",
            Self::PageSingle => "▭",
            Self::PageWithSidebar => "▐▭",
            Self::PageWithAside => "▭▌",
            Self::PageSidebarAndAside => "▐▭▌",
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
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum LayoutRegion {
    Main, Sidebar, Aside, Header, Footer, Hero, Left, Right, Stepper,
}

impl LayoutRegion {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Main => "main", Self::Sidebar => "sidebar", Self::Aside => "aside",
            Self::Header => "header", Self::Footer => "footer", Self::Hero => "hero",
            Self::Left => "left", Self::Right => "right", Self::Stepper => "stepper",
        }
    }
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "main" => Some(Self::Main), "sidebar" => Some(Self::Sidebar),
            "aside" => Some(Self::Aside), "header" => Some(Self::Header),
            "footer" => Some(Self::Footer), "hero" => Some(Self::Hero),
            "left" => Some(Self::Left), "right" => Some(Self::Right),
            "stepper" => Some(Self::Stepper), _ => None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct BlockDef {
    pub id: &'static str,
    pub label: &'static str,
    pub icon: &'static str,
}

#[derive(Clone, Debug)]
pub struct DroppedBlock {
    pub instance_id: uuid::Uuid,
    pub block: BlockDef,
    pub region: LayoutRegion,
    pub order: usize,
}

/// Estado global do drag — compartilhado via provide_context
#[derive(Clone, Debug)]
pub struct DragContext {
    /// None = nada sendo arrastado
    pub instance_id: Option<uuid::Uuid>,
    /// BlockDef sendo arrastado (novo do aside = sem instance_id)
    pub block_def: Option<BlockDef>,
    /// Região de origem (None se vier do aside)
    pub source_region: Option<LayoutRegion>,
}

impl DragContext {
    pub fn empty() -> Self {
        Self { instance_id: None, block_def: None, source_region: None }
    }
    pub fn is_dragging(&self) -> bool {
        self.block_def.is_some()
    }
}

pub fn all_blocks() -> Vec<BlockDef> {
    vec![
        BlockDef { id: "header",        label: "Header",        icon: "▬" },
        BlockDef { id: "footer",        label: "Footer",        icon: "▬" },
        BlockDef { id: "card",          label: "Card",          icon: "▭" },
        BlockDef { id: "alert",         label: "Alert",         icon: "⚠" },
        BlockDef { id: "callout",       label: "Callout",       icon: "💬" },
        BlockDef { id: "toolbar",       label: "Toolbar",       icon: "⚙" },
        BlockDef { id: "dialog",        label: "Dialog",        icon: "◻" },
        BlockDef { id: "drawer",        label: "Drawer",        icon: "▷" },
        BlockDef { id: "popover",       label: "Popover",       icon: "◉" },
        BlockDef { id: "stat-card",     label: "Stat Card",     icon: "📊" },
        BlockDef { id: "empty-state",   label: "Empty State",   icon: "○" },
        BlockDef { id: "data-table",    label: "Data Table",    icon: "⊞" },
        BlockDef { id: "breadcrumb",    label: "Breadcrumb",    icon: "›" },
        BlockDef { id: "button-group",  label: "Button Group",  icon: "⬚" },
        BlockDef { id: "code-block",    label: "Code Block",    icon: "{}" },
        BlockDef { id: "command-panel", label: "Command Panel", icon: "⌘" },
        BlockDef { id: "field",         label: "Field",         icon: "▱" },
        BlockDef { id: "form",          label: "Form",          icon: "📝" },
        BlockDef { id: "form-actions",  label: "Form Actions",  icon: "↵" },
        BlockDef { id: "list",          label: "List",          icon: "≡" },
        BlockDef { id: "page-header",   label: "Page Header",   icon: "H" },
        BlockDef { id: "skeleton",      label: "Skeleton",      icon: "░" },
        BlockDef { id: "table",         label: "Table",         icon: "⊟" },
    ]
}
