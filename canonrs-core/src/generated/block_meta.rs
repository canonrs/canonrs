// AUTO-GENERATED
use crate::meta_types::{ComponentMeta, ComponentFamily};

pub static CONTAINER_META: ComponentMeta = ComponentMeta {
    id: "container",
    name: "Container",
    family: ComponentFamily::Layout,
    intent: "Max-width centered container",
    capabilities: &[],
    composable: true,
    required_parts: &["content"],
    optional_parts: &[],
};

pub static FOOTER_META: ComponentMeta = ComponentMeta {
    id: "footer",
    name: "Footer",
    family: ComponentFamily::Layout,
    intent: "Page footer block",
    capabilities: &[],
    composable: true,
    required_parts: &["left", "center", "right"],
    optional_parts: &[],
};

pub static STACK_META: ComponentMeta = ComponentMeta {
    id: "stack",
    name: "Stack",
    family: ComponentFamily::Layout,
    intent: "Flex stack container vertical or horizontal",
    capabilities: &[],
    composable: true,
    required_parts: &["items"],
    optional_parts: &[],
};

pub static HEADER_META: ComponentMeta = ComponentMeta {
    id: "header",
    name: "Header",
    family: ComponentFamily::Layout,
    intent: "Page header with left center right regions",
    capabilities: &[],
    composable: true,
    required_parts: &["logo", "nav", "center", "actions"],
    optional_parts: &[],
};

pub static GRID_META: ComponentMeta = ComponentMeta {
    id: "grid",
    name: "Grid",
    family: ComponentFamily::Layout,
    intent: "CSS grid layout with N columns",
    capabilities: &[],
    composable: true,
    required_parts: &["items"],
    optional_parts: &[],
};

pub static COLUMNS_META: ComponentMeta = ComponentMeta {
    id: "columns",
    name: "Columns",
    family: ComponentFamily::Layout,
    intent: "Two equal columns",
    capabilities: &[],
    composable: true,
    required_parts: &["columns"],
    optional_parts: &[],
};

pub static SIDEBAR_LAYOUT_META: ComponentMeta = ComponentMeta {
    id: "sidebar-layout",
    name: "Sidebar Layout",
    family: ComponentFamily::Layout,
    intent: "Block-level sidebar and main content",
    capabilities: &[],
    composable: true,
    required_parts: &["nav", "main"],
    optional_parts: &[],
};

pub static TIMELINE_META: ComponentMeta = ComponentMeta {
    id: "timeline",
    name: "Timeline",
    family: ComponentFamily::DataDisplay,
    intent: "Chronological timeline block",
    capabilities: &[],
    composable: true,
    required_parts: &["header", "items", "footer"],
    optional_parts: &[],
};

pub static COMMAND_PANEL_META: ComponentMeta = ComponentMeta {
    id: "command-panel",
    name: "Command Panel",
    family: ComponentFamily::Overlay,
    intent: "Command palette overlay block",
    capabilities: &[],
    composable: false,
    required_parts: &["search", "results", "footer"],
    optional_parts: &[],
};

pub static WIZARD_META: ComponentMeta = ComponentMeta {
    id: "wizard",
    name: "Wizard",
    family: ComponentFamily::Interactive,
    intent: "Multi-step form wizard block",
    capabilities: &[],
    composable: true,
    required_parts: &["steps", "body", "actions"],
    optional_parts: &[],
};

pub static SPLIT_META: ComponentMeta = ComponentMeta {
    id: "split",
    name: "Split",
    family: ComponentFamily::Layout,
    intent: "Aside and main two-panel block",
    capabilities: &[],
    composable: true,
    required_parts: &["aside", "main"],
    optional_parts: &[],
};

pub static DETAIL_PANEL_META: ComponentMeta = ComponentMeta {
    id: "detail-panel",
    name: "Detail Panel",
    family: ComponentFamily::Layout,
    intent: "Master-detail panel layout",
    capabilities: &[],
    composable: true,
    required_parts: &["aside", "content"],
    optional_parts: &[],
};

pub static FILTER_BAR_META: ComponentMeta = ComponentMeta {
    id: "filter-bar",
    name: "Filter Bar",
    family: ComponentFamily::DataDisplay,
    intent: "Filters and actions bar",
    capabilities: &[],
    composable: true,
    required_parts: &["filters", "actions"],
    optional_parts: &[],
};

pub static STAT_CARD_META: ComponentMeta = ComponentMeta {
    id: "stat-card",
    name: "Stat Card",
    family: ComponentFamily::DataDisplay,
    intent: "Metric stat display block",
    capabilities: &[],
    composable: false,
    required_parts: &["icon", "label", "value", "change"],
    optional_parts: &[],
};

pub static LIST_META: ComponentMeta = ComponentMeta {
    id: "list",
    name: "List",
    family: ComponentFamily::DataDisplay,
    intent: "Vertical list container",
    capabilities: &[],
    composable: true,
    required_parts: &["header", "items", "footer"],
    optional_parts: &[],
};

pub static SECTION_META: ComponentMeta = ComponentMeta {
    id: "section",
    name: "Section",
    family: ComponentFamily::Layout,
    intent: "Self-contained section with header, content and footer",
    capabilities: &[],
    composable: true,
    required_parts: &["header", "content", "footer"],
    optional_parts: &[],
};

pub static PAGE_LAYOUT_META: ComponentMeta = ComponentMeta {
    id: "page-layout",
    name: "Page Layout",
    family: ComponentFamily::Layout,
    intent: "Page layout with optional sidebar and aside",
    capabilities: &[],
    composable: true,
    required_parts: &["sidebar", "content", "aside"],
    optional_parts: &[],
};

pub static WIZARD_LAYOUT_META: ComponentMeta = ComponentMeta {
    id: "wizard-layout",
    name: "Wizard Layout",
    family: ComponentFamily::Layout,
    intent: "Multi-step form with header, stepper, content and footer",
    capabilities: &[],
    composable: true,
    required_parts: &["header", "stepper", "content", "footer"],
    optional_parts: &[],
};

pub static SPLIT_VIEW_META: ComponentMeta = ComponentMeta {
    id: "split-view",
    name: "Split View",
    family: ComponentFamily::Layout,
    intent: "Left context panel and right action/detail panel",
    capabilities: &[],
    composable: true,
    required_parts: &["left", "right"],
    optional_parts: &[],
};

pub static DASHBOARD_META: ComponentMeta = ComponentMeta {
    id: "dashboard",
    name: "Dashboard",
    family: ComponentFamily::Layout,
    intent: "App shell with header, sidebar and main content area",
    capabilities: &[],
    composable: true,
    required_parts: &["header", "sidebar", "content"],
    optional_parts: &[],
};

pub static MARKETING_META: ComponentMeta = ComponentMeta {
    id: "marketing",
    name: "Marketing",
    family: ComponentFamily::Layout,
    intent: "Public page with header, hero, main content and footer",
    capabilities: &[],
    composable: true,
    required_parts: &["header", "hero", "content", "footer"],
    optional_parts: &[],
};

pub static FULLSCREEN_META: ComponentMeta = ComponentMeta {
    id: "fullscreen",
    name: "Fullscreen",
    family: ComponentFamily::Layout,
    intent: "Optional header with full canvas content area",
    capabilities: &[],
    composable: true,
    required_parts: &["header", "content"],
    optional_parts: &[],
};

