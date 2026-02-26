// ─── ActiveLayout ─────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ActiveLayout {
    Dashboard, Marketing, Fullscreen, SplitView, Wizard,
    Section, PageSingle, PageWithSidebar, PageWithAside, PageSidebarAndAside,
}

impl ActiveLayout {
    pub fn id(&self) -> &'static str {
        match self {
            Self::Dashboard => "dashboard", Self::Marketing => "marketing",
            Self::Fullscreen => "fullscreen", Self::SplitView => "split-view",
            Self::Wizard => "wizard", Self::Section => "section",
            Self::PageSingle => "page-single", Self::PageWithSidebar => "page-with-sidebar",
            Self::PageWithAside => "page-with-aside", Self::PageSidebarAndAside => "page-sidebar-and-aside",
        }
    }
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
