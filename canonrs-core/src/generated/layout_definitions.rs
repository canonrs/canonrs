// AUTO-GENERATED
use crate::block_types::{LayoutDefinition, LayoutSlot};

const SECTION_SLOTS: &[LayoutSlot] = &[
    LayoutSlot { id: "header", description: "Section title area" },
    LayoutSlot { id: "content", description: "Section content" },
    LayoutSlot { id: "footer", description: "Section footer actions" },
];

pub static SECTION_LAYOUT: LayoutDefinition = LayoutDefinition {
    id: "section",
    label: "Section",
    description: "Self-contained section with header, content and footer",
    icon: "▤",
    slots: SECTION_SLOTS,
};

const PAGE_LAYOUT_SLOTS: &[LayoutSlot] = &[
    LayoutSlot { id: "sidebar", description: "Navigation sidebar" },
    LayoutSlot { id: "content", description: "Primary content" },
    LayoutSlot { id: "aside", description: "Contextual panel" },
];

pub static PAGE_LAYOUT_LAYOUT: LayoutDefinition = LayoutDefinition {
    id: "page-layout",
    label: "Page",
    description: "Page layout with optional sidebar and aside",
    icon: "▭",
    slots: PAGE_LAYOUT_SLOTS,
};

const WIZARD_LAYOUT_SLOTS: &[LayoutSlot] = &[
    LayoutSlot { id: "header", description: "header" },
    LayoutSlot { id: "stepper", description: "stepper" },
    LayoutSlot { id: "content", description: "content" },
    LayoutSlot { id: "footer", description: "footer" },
];

pub static WIZARD_LAYOUT_LAYOUT: LayoutDefinition = LayoutDefinition {
    id: "wizard-layout",
    label: "Wizard Layout",
    description: "wizard-layout layout",
    icon: "▭",
    slots: WIZARD_LAYOUT_SLOTS,
};

const SPLIT_VIEW_SLOTS: &[LayoutSlot] = &[
    LayoutSlot { id: "left", description: "Context or list panel" },
    LayoutSlot { id: "right", description: "Detail or action panel" },
];

pub static SPLIT_VIEW_LAYOUT: LayoutDefinition = LayoutDefinition {
    id: "split-view",
    label: "Split View",
    description: "Left context panel and right action/detail panel",
    icon: "◧",
    slots: SPLIT_VIEW_SLOTS,
};

const DASHBOARD_SLOTS: &[LayoutSlot] = &[
    LayoutSlot { id: "header", description: "Top navigation bar" },
    LayoutSlot { id: "sidebar", description: "Left navigation panel" },
    LayoutSlot { id: "content", description: "Primary content area" },
];

pub static DASHBOARD_LAYOUT: LayoutDefinition = LayoutDefinition {
    id: "dashboard",
    label: "Dashboard",
    description: "App shell with header, sidebar and main content area",
    icon: "⬛",
    slots: DASHBOARD_SLOTS,
};

const MARKETING_SLOTS: &[LayoutSlot] = &[
    LayoutSlot { id: "header", description: "Site header with navigation" },
    LayoutSlot { id: "hero", description: "Hero/banner section" },
    LayoutSlot { id: "content", description: "Main content sections" },
    LayoutSlot { id: "footer", description: "Site footer" },
];

pub static MARKETING_LAYOUT: LayoutDefinition = LayoutDefinition {
    id: "marketing",
    label: "Marketing",
    description: "Public page with header, hero, main content and footer",
    icon: "🌐",
    slots: MARKETING_SLOTS,
};

const FULLSCREEN_SLOTS: &[LayoutSlot] = &[
    LayoutSlot { id: "header", description: "Optional top bar" },
    LayoutSlot { id: "content", description: "Full canvas area" },
];

pub static FULLSCREEN_LAYOUT: LayoutDefinition = LayoutDefinition {
    id: "fullscreen",
    label: "Fullscreen",
    description: "Optional header with full canvas content area",
    icon: "⬜",
    slots: FULLSCREEN_SLOTS,
};

pub static LAYOUT_DEFINITIONS_STATIC: &[LayoutDefinition] = &[
    SECTION_LAYOUT,
    PAGE_LAYOUT_LAYOUT,
    WIZARD_LAYOUT_LAYOUT,
    SPLIT_VIEW_LAYOUT,
    DASHBOARD_LAYOUT,
    MARKETING_LAYOUT,
    FULLSCREEN_LAYOUT,
];
