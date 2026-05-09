//! canonrs-server::prelude — ponto de entrada unico
//!
//! use canonrs_server::prelude::*;

pub use crate::ui::prelude::*;
pub use crate::blocks::prelude::*;
pub use crate::layouts::*;
pub use canonrs_core::primitives::*;

pub use crate::ui::previews::*;

pub use crate::ui::markdown::{MarkdownSurface, MarkdownLayout, MarkdownContent, MarkdownTOC, RenderedMarkdown, TocPosition, render_markdown};
pub use crate::ui::table_of_contents::TableOfContents;

pub use crate::blocks::card::CardVariant;
pub use crate::ui::link_group::LinkGroupDirection;
pub use canonrs_core::primitives::layout::container::ContainerSize;
pub use canonrs_core::primitives::layout::center::CenterMode;
pub use canonrs_core::primitives::layout::center::CenterPrimitive;
pub use crate::ui::logo::LogoSize;
pub use canonrs_core::meta::ActivityState;
pub use crate::ui::select::{SelectTrigger, SelectValue, SelectContent, SelectItem, SelectSeparator};

// Desambiguação: UI boundaries sobrescrevem primitives do core
pub use crate::ui::button::button_boundary::Button;
pub use crate::ui::badge::badge_boundary::Badge;
pub use crate::blocks::page_header::page_header_block::PageHeader;

// ── Desambiguação explícita: UI boundaries sobrescrevem primitives do core ──
pub use crate::ui::sidebar::sidebar_boundary::{
    Sidebar, SidebarHeader, SidebarContent, SidebarFooter,
    SidebarMenu, SidebarMenuItem, SidebarMenuGroup,
    SidebarSeparator, SidebarGroupLabel, SidebarTrigger,
    SidebarGroup, SidebarGroupTrigger, SidebarGroupContent,
};

// ── Desambiguação completa: server boundaries sobrescrevem primitives ──
pub use crate::ui::input::input_boundary::Input;
pub use crate::ui::tabs::tabs_boundary::{TabsRoot, TabsRoot as Tabs, TabsTrigger, TabsContent};
pub use crate::ui::tabs::tabs_boundary::TabsListBoundary as TabsList;
pub use canonrs_core::table_of_contents::TocMode;
pub use crate::ui::table::table_boundary::{TableRow, TableCell, TableHeader, TableHead, TableBody};
pub use canonrs_core::separator::SeparatorOrientation;
pub use crate::ui::sidebar::SidebarPreviewInteractive;
pub use crate::ui::sidebar::SidebarPreviewWithAccordion;
pub use crate::ui::sidebar::SidebarPreviewWithBadges;
pub use crate::ui::sidebar::SidebarPreviewWithSearch;
pub use crate::ui::sidebar::SidebarPreviewGroupsCollapsible;
pub use crate::ui::sidebar::SidebarPreviewResponsive;
pub use crate::ui::sidebar::SidebarPreviewMultiLevel;
pub use crate::ui::sidebar::SidebarPreviewPinnable;
pub use crate::ui::sidebar::SidebarPreviewRailMode;
