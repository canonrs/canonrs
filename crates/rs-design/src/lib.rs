pub mod blocks;
pub mod layouts;
pub mod providers;
pub use providers::ThemeProvider;
pub mod primitives;
pub mod tokens;
pub mod ui;
pub mod commands;
mod themes;

pub use ui::button::Button;
pub use commands::*;
pub use ui::table::{Table, TableHeader, TableBody, TableFooter, TableRow, TableHead, TableCell, TableCaption};
pub use commands::*;
pub use ui::select::{Select, SelectOption};
pub use commands::*;
pub use ui::pagination::{Pagination, PaginationContent, PaginationItem, PaginationLink, PaginationPrevious, PaginationNext, PaginationEllipsis};
pub use commands::*;
pub use blocks::data_table_column_header::{DataTableColumnHeader, SortDirection};
pub use blocks::data_table::{DataTable, DataTableColumn};
pub use ui::sidebar::{SidebarContent, SidebarFooter, SidebarInset, SidebarTrigger};
pub use commands::*;
pub use ui::separator::Separator;
pub use commands::*;
pub use ui::sidebar::{SidebarProvider, Sidebar, SidebarHeader};
pub use commands::*;
pub use ui::sidebar::{SidebarMenu, SidebarMenuItem, SidebarMenuButton, SidebarGroup, SidebarGroupLabel, SidebarGroupContent};
pub use commands::*;
pub use ui::label::Label;
pub use commands::*;
pub use ui::slider::Slider;
pub use commands::*;
pub use ui::combobox::{Combobox, ComboboxOption};
pub use commands::*;
pub use ui::color_picker::ColorPicker;
pub use commands::*;
pub use ui::tabs::{Tabs, TabsList, TabsTrigger, TabsContent};
pub use commands::*;

/// Canon rules documentation
/// 
/// See `CANON.md` for overview and navigation.
#[cfg(doc)]
pub mod rules;
pub use blocks::DataTableViewOptions;

// DataTable extras
pub use blocks::{
    DataTableRowActions,
    RowAction,
    RowActionVariant,
};

// Virtual Table
pub use crate::ui::virtual_table::{VirtualTable, VirtualColumn, VirtualRow, ColumnAlign};

// Input and Select

// Input component
pub use crate::ui::input::Input;

// Theme Toggle

// Theme System
pub use crate::ui::theme_toggle::{ThemeToggle, ThemeToggleVariant, ToggleSize};
pub use crate::ui::theme_picker::ThemePicker;
pub use crate::providers::{ThemeMode, use_theme};
pub use crate::themes::ThemeEngine;
pub use crate::ui::theme_toggle::ThemeSettingsDropdown;
pub use crate::ui::button::ButtonVariant;

// Density Provider
pub use providers::{DensityProvider, DensityMode, use_density};

// Language Provider
pub use providers::{LanguageProvider, Language, TextDirection, use_language};

// Workflow Component
pub mod components;
pub use components::workflow::WorkflowDemo;
