//! Sidebar data types — NavItem, NavGroup, SidebarConfig

use crate::ui::badge::badge_boundary::Badge;
use canonrs_core::primitives::BadgeVariant;
use canonrs_core::primitives::SidebarVariant;
use canonrs_core::meta::VisibilityState;

#[derive(Clone, Debug)]
pub struct NavBadge {
    pub label: &'static str,
    pub variant: BadgeVariant,
}

#[derive(Clone, Debug)]
pub struct NavItem {
    pub icon: &'static str,
    pub label: &'static str,
    pub href: &'static str,
    pub active: bool,
    pub badge: Option<NavBadge>,
    pub children: Vec<NavItem>,
}

impl NavItem {
    pub fn new(icon: &'static str, label: &'static str, href: &'static str) -> Self {
        Self { icon, label, href, active: false, badge: None, children: vec![] }
    }
    pub fn active(mut self) -> Self { self.active = true; self }
    pub fn badge(mut self, label: &'static str, variant: BadgeVariant) -> Self {
        self.badge = Some(NavBadge { label, variant }); self
    }
    pub fn children(mut self, children: Vec<NavItem>) -> Self {
        self.children = children; self
    }
}

#[derive(Clone, Debug)]
pub struct NavGroup {
    pub icon: &'static str,
    pub label: &'static str,
    pub items: Vec<NavItem>,
    pub collapsible: bool,
}

impl NavGroup {
    pub fn new(label: &'static str, items: Vec<NavItem>) -> Self {
        Self { icon: "", label, items, collapsible: false }
    }
    pub fn icon(mut self, icon: &'static str) -> Self { self.icon = icon; self }
    pub fn collapsible(mut self) -> Self { self.collapsible = true; self }
}

#[derive(Clone, Debug)]
pub struct SidebarConfig {
    pub state: VisibilityState,
    pub variant: SidebarVariant,
    pub pinnable: bool,
    pub tooltips: bool,
    pub search: bool,
    pub responsive: bool,
    pub user_name: &'static str,
    pub user_email: &'static str,
    pub groups: Vec<NavGroup>,
}

impl Default for SidebarConfig {
    fn default() -> Self {
        Self {
            state: VisibilityState::Open,
            variant: SidebarVariant::Default,
            pinnable: false,
            tooltips: false,
            search: false,
            responsive: false,
            user_name: "John Doe",
            user_email: "john@canonrs.dev",
            groups: vec![],
        }
    }
}
