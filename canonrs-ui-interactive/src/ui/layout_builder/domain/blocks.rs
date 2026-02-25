use super::constraints::{AcceptRule, BlockDef, BlockRegion, ComponentDef, NodeCategory};
use once_cell::sync::Lazy;
use std::collections::HashMap;

pub static BLOCK_REGISTRY: Lazy<HashMap<&'static str, BlockDef>> = Lazy::new(|| {
    all_blocks().into_iter().map(|b| (b.id, b)).collect()
});

pub static COMPONENT_REGISTRY: Lazy<HashMap<&'static str, ComponentDef>> = Lazy::new(|| {
    all_components().into_iter().map(|c| (c.id, c)).collect()
});

pub fn get_block(id: &str) -> Option<&'static BlockDef> { BLOCK_REGISTRY.get(id) }
pub fn get_component(id: &str) -> Option<&'static ComponentDef> { COMPONENT_REGISTRY.get(id) }

const LAYOUT_ACCEPTS: &[AcceptRule] = &[
    AcceptRule::Category(NodeCategory::Layout), AcceptRule::Category(NodeCategory::Content),
    AcceptRule::Category(NodeCategory::Nav),    AcceptRule::Category(NodeCategory::Text),
];
const CONTENT_ACCEPTS: &[AcceptRule] = &[
    AcceptRule::Category(NodeCategory::Content), AcceptRule::Category(NodeCategory::Nav),
    AcceptRule::Category(NodeCategory::FormField), AcceptRule::Category(NodeCategory::Text),
];
const FORM_ACCEPTS: &[AcceptRule] = &[
    AcceptRule::Category(NodeCategory::FormField), AcceptRule::Category(NodeCategory::Content),
];
const NAV_ACCEPTS: &[AcceptRule] = &[
    AcceptRule::Category(NodeCategory::Nav), AcceptRule::Category(NodeCategory::Content),
    AcceptRule::Category(NodeCategory::Text),
];
const ACTIONS_ACCEPTS: &[AcceptRule] = &[
    AcceptRule::Category(NodeCategory::FormField), AcceptRule::Category(NodeCategory::Nav),
    AcceptRule::Block("button"), AcceptRule::Block("button-group"),
];
const HEADER_ACCEPTS: &[AcceptRule] = &[
    AcceptRule::Category(NodeCategory::Text), AcceptRule::Category(NodeCategory::Nav),
    AcceptRule::Block("badge"), AcceptRule::Block("avatar"),
    AcceptRule::Block("button"), AcceptRule::Block("button-group"),
    AcceptRule::Block("breadcrumb"), AcceptRule::Block("page-header"),
];
const FOOTER_ACCEPTS: &[AcceptRule] = &[
    AcceptRule::Category(NodeCategory::Nav), AcceptRule::Category(NodeCategory::Text),
    AcceptRule::Block("button"), AcceptRule::Block("button-group"),
    AcceptRule::Block("breadcrumb"),
];

const THREE_PANEL_REGIONS: &[BlockRegion] = &[
    BlockRegion::new("header",  "Header",  HEADER_ACCEPTS),
    BlockRegion::new("content", "Content", LAYOUT_ACCEPTS),
    BlockRegion::new("footer",  "Footer",  FOOTER_ACCEPTS),
];
const HEADER_BODY_FOOTER_REGIONS: &[BlockRegion] = &[
    BlockRegion::new("header", "Header", HEADER_ACCEPTS),
    BlockRegion::new("body",   "Body",   LAYOUT_ACCEPTS),
    BlockRegion::new("footer", "Footer", FOOTER_ACCEPTS),
];
const LCR_REGIONS: &[BlockRegion] = &[
    BlockRegion::new("left",   "Left",   NAV_ACCEPTS),
    BlockRegion::new("center", "Center", CONTENT_ACCEPTS),
    BlockRegion::new("right",  "Right",  NAV_ACCEPTS),
];
const FOOTER_REGIONS: &[BlockRegion] = &[
    BlockRegion::new("left",   "Left",   CONTENT_ACCEPTS),
    BlockRegion::new("center", "Center", CONTENT_ACCEPTS),
    BlockRegion::new("right",  "Right",  FOOTER_ACCEPTS),
];
const FORM_REGIONS: &[BlockRegion] = &[
    BlockRegion::new("body",    "Fields",  FORM_ACCEPTS),
    BlockRegion::new("actions", "Actions", ACTIONS_ACCEPTS),
];
const SINGLE_CONTENT_REGIONS: &[BlockRegion] = &[
    BlockRegion::new("content", "Content", LAYOUT_ACCEPTS),
];
const OVERLAY_REGIONS: &[BlockRegion] = &[
    BlockRegion::new("header", "Header", HEADER_ACCEPTS),
    BlockRegion::new("body",   "Body",   LAYOUT_ACCEPTS),
    BlockRegion::new("footer", "Footer", FOOTER_ACCEPTS),
];

pub fn all_blocks() -> Vec<BlockDef> {
    use NodeCategory::*;
    vec![
        BlockDef { id: "header",        label: "Header",        icon: "▬",  category: Page,      is_container: true,  regions: LCR_REGIONS },
        BlockDef { id: "footer",        label: "Footer",        icon: "▬",  category: Page,      is_container: true,  regions: FOOTER_REGIONS },
        BlockDef { id: "card",          label: "Card",          icon: "▭",  category: Layout,    is_container: true,  regions: THREE_PANEL_REGIONS },
        BlockDef { id: "section",       label: "Section",       icon: "▤",  category: Layout,    is_container: true,  regions: HEADER_BODY_FOOTER_REGIONS },
        BlockDef { id: "dialog",        label: "Dialog",        icon: "◻",  category: Overlay,   is_container: true,  regions: OVERLAY_REGIONS },
        BlockDef { id: "drawer",        label: "Drawer",        icon: "▷",  category: Overlay,   is_container: true,  regions: OVERLAY_REGIONS },
        BlockDef { id: "form",          label: "Form",          icon: "📝", category: Form,      is_container: true,  regions: FORM_REGIONS },
        BlockDef { id: "popover",       label: "Popover",       icon: "◉",  category: Overlay,   is_container: true,  regions: SINGLE_CONTENT_REGIONS },
        BlockDef { id: "alert",         label: "Alert",         icon: "⚠",  category: Content,   is_container: false, regions: &[] },
        BlockDef { id: "callout",       label: "Callout",       icon: "💬", category: Content,   is_container: false, regions: &[] },
        BlockDef { id: "stat-card",     label: "Stat Card",     icon: "📊", category: Content,   is_container: false, regions: &[] },
        BlockDef { id: "empty-state",   label: "Empty State",   icon: "○",  category: Content,   is_container: false, regions: &[] },
        BlockDef { id: "data-table",    label: "Data Table",    icon: "⊞",  category: Content,   is_container: false, regions: &[] },
        BlockDef { id: "code-block",    label: "Code Block",    icon: "{}", category: Content,   is_container: false, regions: &[] },
        BlockDef { id: "list",          label: "List",          icon: "≡",  category: Content,   is_container: false, regions: &[] },
        BlockDef { id: "skeleton",      label: "Skeleton",      icon: "░",  category: Content,   is_container: false, regions: &[] },
        BlockDef { id: "table",         label: "Table",         icon: "⊟",  category: Content,   is_container: false, regions: &[] },
        BlockDef { id: "toolbar",       label: "Toolbar",       icon: "⚙",  category: Nav,       is_container: false, regions: &[] },
        BlockDef { id: "breadcrumb",    label: "Breadcrumb",    icon: "›",  category: Nav,       is_container: false, regions: &[] },
        BlockDef { id: "button-group",  label: "Button Group",  icon: "⬚",  category: Nav,       is_container: false, regions: &[] },
        BlockDef { id: "page-header",   label: "Page Header",   icon: "H",  category: Nav,       is_container: false, regions: &[] },
        BlockDef { id: "field",         label: "Field",         icon: "▱",  category: FormField, is_container: false, regions: &[] },
        BlockDef { id: "form-actions",  label: "Form Actions",  icon: "↵",  category: FormField, is_container: false, regions: &[] },
        BlockDef { id: "command-panel", label: "Command Panel", icon: "⌘",  category: Overlay,   is_container: false, regions: &[] },
        BlockDef { id: "markdown",      label: "Markdown",      icon: "M",  category: Content,   is_container: false, regions: &[] },
        BlockDef { id: "text-h1",       label: "Heading 1",     icon: "H1", category: Text,      is_container: false, regions: &[] },
        BlockDef { id: "text-h2",       label: "Heading 2",     icon: "H2", category: Text,      is_container: false, regions: &[] },
        BlockDef { id: "text-h3",       label: "Heading 3",     icon: "H3", category: Text,      is_container: false, regions: &[] },
        BlockDef { id: "text-p",        label: "Paragraph",     icon: "¶",  category: Text,      is_container: false, regions: &[] },
        BlockDef { id: "text-label",    label: "Label",         icon: "T",  category: Text,      is_container: false, regions: &[] },
        BlockDef { id: "text-caption",  label: "Caption",       icon: "t",  category: Text,      is_container: false, regions: &[] },
    ]
}

pub fn all_components() -> Vec<ComponentDef> {
    use NodeCategory::*;
    vec![
        ComponentDef { id: "accordion",       label: "Accordion",       icon: "≡",   category: Layout,    description: "Expandable sections" },
        ComponentDef { id: "carousel",        label: "Carousel",        icon: "◁▷",  category: Layout,    description: "Rotating content" },
        ComponentDef { id: "collapsible",     label: "Collapsible",     icon: "▼",   category: Layout,    description: "Toggle content visibility" },
        ComponentDef { id: "resizable",       label: "Resizable",       icon: "◧",   category: Layout,    description: "Resizable panels" },
        ComponentDef { id: "separator",       label: "Separator",       icon: "—",   category: Layout,    description: "Visual divider" },
        ComponentDef { id: "aspect-ratio",    label: "Aspect Ratio",    icon: "▭",   category: Layout,    description: "Fixed ratio container" },
        ComponentDef { id: "scroll-area",     label: "Scroll Area",     icon: "↕",   category: Layout,    description: "Scrollable container" },
        ComponentDef { id: "tabs",            label: "Tabs",            icon: "⊟",   category: Nav,       description: "Tabbed navigation" },
        ComponentDef { id: "sidebar",         label: "Sidebar",         icon: "▐",   category: Nav,       description: "Side navigation panel" },
        ComponentDef { id: "nav-item",        label: "Nav Item",        icon: "›",   category: Nav,       description: "Single navigation link" },
        ComponentDef { id: "navigation-menu", label: "Nav Menu",        icon: "☰",   category: Nav,       description: "Navigation menu" },
        ComponentDef { id: "menubar",         label: "Menubar",         icon: "▬",   category: Nav,       description: "Horizontal menu bar" },
        ComponentDef { id: "menu",            label: "Menu",            icon: "≡",   category: Nav,       description: "Dropdown menu" },
        ComponentDef { id: "context-menu",    label: "Context Menu",    icon: "⋮",   category: Nav,       description: "Right-click menu" },
        ComponentDef { id: "dropdown-menu",   label: "Dropdown",        icon: "▾",   category: Nav,       description: "Dropdown trigger" },
        ComponentDef { id: "pagination",      label: "Pagination",      icon: "«»",  category: Nav,       description: "Page navigation" },
        ComponentDef { id: "tree",            label: "Tree",            icon: "⊢",   category: Nav,       description: "Hierarchical list" },
        ComponentDef { id: "avatar",          label: "Avatar",          icon: "◉",   category: Content,   description: "User avatar image" },
        ComponentDef { id: "badge",           label: "Badge",           icon: "◎",   category: Content,   description: "Status badge" },
        ComponentDef { id: "banner",          label: "Banner",          icon: "▬",   category: Content,   description: "Announcement banner" },
        ComponentDef { id: "stat",            label: "Stat",            icon: "📊",  category: Content,   description: "Single metric display" },
        ComponentDef { id: "status-dot",      label: "Status Dot",      icon: "●",   category: Content,   description: "Status indicator" },
        ComponentDef { id: "progress",        label: "Progress",        icon: "▰",   category: Content,   description: "Progress bar" },
        ComponentDef { id: "spinner",         label: "Spinner",         icon: "◌",   category: Content,   description: "Loading spinner" },
        ComponentDef { id: "skeleton",        label: "Skeleton",        icon: "░",   category: Content,   description: "Loading skeleton" },
        ComponentDef { id: "pulse",           label: "Pulse",           icon: "◎",   category: Content,   description: "Pulse animation" },
        ComponentDef { id: "doc-progress",    label: "Doc Progress",    icon: "▰",   category: Content,   description: "Document progress" },
        ComponentDef { id: "markdown",        label: "Markdown",        icon: "M",   category: Content,   description: "Markdown renderer" },
        ComponentDef { id: "link",            label: "Link",            icon: "🔗",  category: Content,   description: "Hyperlink" },
        ComponentDef { id: "kbd",             label: "Kbd",             icon: "⌨",   category: Content,   description: "Keyboard shortcut" },
        ComponentDef { id: "icon",            label: "Icon",            icon: "★",   category: Content,   description: "Icon element" },
        ComponentDef { id: "animate",         label: "Animate",         icon: "▷",   category: Content,   description: "Animation wrapper" },
        ComponentDef { id: "virtual-list",    label: "Virtual List",    icon: "≡",   category: Content,   description: "Virtualized list" },
        ComponentDef { id: "table-of-contents",label:"TOC",             icon: "≡",   category: Content,   description: "Table of contents" },
        ComponentDef { id: "button",          label: "Button",          icon: "⬚",   category: FormField, description: "Action button" },
        ComponentDef { id: "icon-button",     label: "Icon Button",     icon: "⊙",   category: FormField, description: "Icon-only button" },
        ComponentDef { id: "input",           label: "Input",           icon: "▱",   category: FormField, description: "Text input" },
        ComponentDef { id: "input-group",     label: "Input Group",     icon: "▱▱",  category: FormField, description: "Grouped inputs" },
        ComponentDef { id: "input-otp",       label: "OTP Input",       icon: "□□□", category: FormField, description: "One-time password" },
        ComponentDef { id: "textarea",        label: "Textarea",        icon: "▭",   category: FormField, description: "Multi-line input" },
        ComponentDef { id: "select",          label: "Select",          icon: "▾",   category: FormField, description: "Dropdown select" },
        ComponentDef { id: "combobox",        label: "Combobox",        icon: "▾",   category: FormField, description: "Searchable select" },
        ComponentDef { id: "checkbox",        label: "Checkbox",        icon: "☑",   category: FormField, description: "Checkbox input" },
        ComponentDef { id: "radio",           label: "Radio",           icon: "◉",   category: FormField, description: "Radio button" },
        ComponentDef { id: "radio-group",     label: "Radio Group",     icon: "◉◉",  category: FormField, description: "Radio button group" },
        ComponentDef { id: "switch",          label: "Switch",          icon: "⊙",   category: FormField, description: "Toggle switch" },
        ComponentDef { id: "toggle",          label: "Toggle",          icon: "⊡",   category: FormField, description: "Toggle button" },
        ComponentDef { id: "toggle-group",    label: "Toggle Group",    icon: "⊡⊡",  category: FormField, description: "Toggle button group" },
        ComponentDef { id: "slider",          label: "Slider",          icon: "─●─", category: FormField, description: "Range slider" },
        ComponentDef { id: "color-picker",    label: "Color Picker",    icon: "🎨",  category: FormField, description: "Color selection" },
        ComponentDef { id: "label",           label: "Label",           icon: "T",   category: FormField, description: "Form label" },
        ComponentDef { id: "copy-button",     label: "Copy Button",     icon: "⎘",   category: FormField, description: "Copy to clipboard" },
        ComponentDef { id: "tooltip",         label: "Tooltip",         icon: "💬",  category: Overlay,   description: "Hover tooltip" },
        ComponentDef { id: "hover-card",      label: "Hover Card",      icon: "▭",   category: Overlay,   description: "Hover preview card" },
        ComponentDef { id: "modal",           label: "Modal",           icon: "◻",   category: Overlay,   description: "Modal dialog" },
        ComponentDef { id: "sheet",           label: "Sheet",           icon: "▷",   category: Overlay,   description: "Side sheet" },
        ComponentDef { id: "toast",           label: "Toast",           icon: "📢",  category: Overlay,   description: "Toast notification" },
        ComponentDef { id: "alert-dialog",    label: "Alert Dialog",    icon: "⚠",   category: Overlay,   description: "Confirmation dialog" },
        ComponentDef { id: "confirm-dialog",  label: "Confirm Dialog",  icon: "?",   category: Overlay,   description: "Confirm action dialog" },
        ComponentDef { id: "loading-overlay", label: "Loading Overlay", icon: "◌",   category: Overlay,   description: "Full-screen loading" },
        ComponentDef { id: "inline-notice",   label: "Inline Notice",   icon: "ℹ",   category: Content,   description: "Inline notification" },
        ComponentDef { id: "error-state",     label: "Error State",     icon: "✕",   category: Content,   description: "Error display" },
        ComponentDef { id: "empty-table",     label: "Empty Table",     icon: "⊞",   category: Content,   description: "Empty table state" },
    ]
}
