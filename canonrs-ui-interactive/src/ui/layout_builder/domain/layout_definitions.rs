use super::constraints::{NodeCategory, AcceptRule};

/// Definição formal de uma região de layout
#[derive(Debug)]
pub struct RegionDef {
    pub id: &'static str,
    pub label: &'static str,
    pub accepts: &'static [AcceptRule],
    pub max_children: Option<usize>,
    pub hint: &'static str,
}

/// Definição formal de um layout
#[derive(Debug)]
pub struct LayoutDef {
    pub id: &'static str,
    pub label: &'static str,
    pub icon: &'static str,
    pub description: &'static str,
    pub version: u32,
    pub regions: &'static [RegionDef],
}

static NAV_RULES: &[AcceptRule] = &[AcceptRule::Category(NodeCategory::Nav)];
static CONTENT_RULES: &[AcceptRule] = &[AcceptRule::Category(NodeCategory::Content)];

pub static DASHBOARD_DEF: LayoutDef = LayoutDef {
    id: "dashboard", label: "Dashboard", icon: "⬛", description: "App shell with sidebar navigation", version: 1,
    regions: &[
        RegionDef { id: "header",  label: "Header",  accepts: NAV_RULES,     max_children: Some(1), hint: "Nav, toolbar" },
        RegionDef { id: "sidebar", label: "Sidebar", accepts: NAV_RULES,     max_children: Some(1), hint: "Navigation menu" },
        RegionDef { id: "main",    label: "Main",    accepts: CONTENT_RULES, max_children: None,    hint: "Page content" },
    ],
};

pub static MARKETING_DEF: LayoutDef = LayoutDef {
    id: "marketing", label: "Marketing", icon: "🌐", description: "Public-facing marketing pages", version: 1,
    regions: &[
        RegionDef { id: "header", label: "Header", accepts: NAV_RULES,     max_children: Some(1), hint: "Logo, nav, actions" },
        RegionDef { id: "hero",   label: "Hero",   accepts: CONTENT_RULES, max_children: Some(1), hint: "Hero banner" },
        RegionDef { id: "main",   label: "Main",   accepts: CONTENT_RULES, max_children: None,    hint: "Page sections" },
        RegionDef { id: "footer", label: "Footer", accepts: NAV_RULES,     max_children: Some(1), hint: "Footer links" },
    ],
};

pub static FULLSCREEN_DEF: LayoutDef = LayoutDef {
    id: "fullscreen", label: "Fullscreen", icon: "⬜", description: "Focused editors and tools", version: 1,
    regions: &[
        RegionDef { id: "header", label: "Header", accepts: NAV_RULES,     max_children: Some(1), hint: "Toolbar" },
        RegionDef { id: "main",   label: "Main",   accepts: CONTENT_RULES, max_children: None,    hint: "Editor content" },
    ],
};

pub static SPLIT_VIEW_DEF: LayoutDef = LayoutDef {
    id: "split-view", label: "Split View", icon: "◫", description: "Side-by-side panels", version: 1,
    regions: &[
        RegionDef { id: "left",  label: "Left",  accepts: CONTENT_RULES, max_children: None, hint: "Context / branding" },
        RegionDef { id: "right", label: "Right", accepts: CONTENT_RULES, max_children: None, hint: "Action / form" },
    ],
};

pub static WIZARD_DEF: LayoutDef = LayoutDef {
    id: "wizard", label: "Wizard", icon: "🧙", description: "Multi-step guided flows", version: 1,
    regions: &[
        RegionDef { id: "header",  label: "Header",  accepts: NAV_RULES,     max_children: Some(1), hint: "Branding, title" },
        RegionDef { id: "stepper", label: "Stepper", accepts: NAV_RULES,     max_children: Some(1), hint: "Step progress" },
        RegionDef { id: "main",    label: "Main",    accepts: CONTENT_RULES, max_children: None,    hint: "Step content" },
        RegionDef { id: "footer",  label: "Footer",  accepts: NAV_RULES,     max_children: Some(1), hint: "Back / Next" },
    ],
};

pub static SECTION_DEF: LayoutDef = LayoutDef {
    id: "section", label: "Section", icon: "▤", description: "Content section with header/footer", version: 1,
    regions: &[
        RegionDef { id: "header", label: "Header", accepts: CONTENT_RULES, max_children: Some(1), hint: "Section title" },
        RegionDef { id: "main",   label: "Main",   accepts: CONTENT_RULES, max_children: None,    hint: "Section content" },
        RegionDef { id: "footer", label: "Footer", accepts: CONTENT_RULES, max_children: Some(1), hint: "Section footer" },
    ],
};

pub static PAGE_SINGLE_DEF: LayoutDef = LayoutDef {
    id: "page-single", label: "Page", icon: "📄", description: "Single column page", version: 1,
    regions: &[
        RegionDef { id: "main", label: "Main", accepts: CONTENT_RULES, max_children: None, hint: "Page content" },
    ],
};

pub static PAGE_WITH_SIDEBAR_DEF: LayoutDef = LayoutDef {
    id: "page-with-sidebar", label: "Page + Sidebar", icon: "▐", description: "Page with left sidebar", version: 1,
    regions: &[
        RegionDef { id: "sidebar", label: "Sidebar", accepts: NAV_RULES,     max_children: Some(1), hint: "Navigation" },
        RegionDef { id: "main",    label: "Main",    accepts: CONTENT_RULES, max_children: None,    hint: "Page content" },
    ],
};

pub static PAGE_WITH_ASIDE_DEF: LayoutDef = LayoutDef {
    id: "page-with-aside", label: "Page + Aside", icon: "▌", description: "Page with right aside", version: 1,
    regions: &[
        RegionDef { id: "main",  label: "Main",  accepts: CONTENT_RULES, max_children: None,    hint: "Page content" },
        RegionDef { id: "aside", label: "Aside", accepts: CONTENT_RULES, max_children: Some(1), hint: "Related content" },
    ],
};

pub static PAGE_SIDEBAR_ASIDE_DEF: LayoutDef = LayoutDef {
    id: "page-sidebar-and-aside", label: "Page + Sidebar + Aside", icon: "▐▌", description: "Full three-column layout", version: 1,
    regions: &[
        RegionDef { id: "sidebar", label: "Sidebar", accepts: NAV_RULES,     max_children: Some(1), hint: "Navigation" },
        RegionDef { id: "main",    label: "Main",    accepts: CONTENT_RULES, max_children: None,    hint: "Page content" },
        RegionDef { id: "aside",   label: "Aside",   accepts: CONTENT_RULES, max_children: Some(1), hint: "Related content" },
    ],
};

pub fn get_layout_def(id: &str) -> Option<&'static LayoutDef> {
    match id {
        "dashboard"              => Some(&DASHBOARD_DEF),
        "marketing"              => Some(&MARKETING_DEF),
        "fullscreen"             => Some(&FULLSCREEN_DEF),
        "split-view"             => Some(&SPLIT_VIEW_DEF),
        "wizard"                 => Some(&WIZARD_DEF),
        "section"                => Some(&SECTION_DEF),
        "page-single"            => Some(&PAGE_SINGLE_DEF),
        "page-with-sidebar"      => Some(&PAGE_WITH_SIDEBAR_DEF),
        "page-with-aside"        => Some(&PAGE_WITH_ASIDE_DEF),
        "page-sidebar-and-aside" => Some(&PAGE_SIDEBAR_ASIDE_DEF),
        _ => None,
    }
}
