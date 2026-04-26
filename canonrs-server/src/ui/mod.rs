// ── UI Modules ────────────────────────────────────────────────────────────────
// Rule: only boundary exports. Never import _ui.rs directly.

pub mod accordion; pub mod alert; pub mod alert_dialog; pub mod animate;
pub mod aspect_ratio; pub mod avatar; pub mod badge; pub mod banner;
pub mod breadcrumb; pub mod button; pub mod button_group; pub mod callout;
pub mod card; pub mod carousel; pub mod chart; pub mod checkbox;
pub mod code_block; pub mod collapsible; pub mod color_picker; pub mod combobox;
pub mod command; pub mod confirm_dialog; pub mod context_menu; pub mod copy_button;
pub mod data_table; pub mod dialog; pub mod doc_progress; pub mod drawer;
pub mod dropdown_menu; pub mod empty_state; pub mod empty_table; pub mod error_state;
pub mod field; pub mod form; pub mod form_error_summary; pub mod hero;
pub mod hover_card; pub mod icon; pub mod icon_button; pub mod inline_meta;
pub mod inline_notice; pub mod input; pub mod input_group; pub mod input_otp;
pub mod kbd; pub mod label; pub mod link; pub mod link_group;
pub mod list_item; pub mod loading_overlay; pub mod logo; pub mod markdown;
pub mod menu; pub mod menubar; pub mod modal; pub mod nav_item;
pub mod navigation_menu; pub mod page_header; pub mod pagination; pub mod popover;
pub mod progress; pub mod pulse; pub mod radio; pub mod radio_group;
pub mod resizable; pub mod scroll_area; pub mod section; pub mod select;
pub mod separator; pub mod sheet; pub mod sidebar; pub mod skeleton;
pub mod slider; pub mod spinner; pub mod stat; pub mod status_dot;
pub mod switch; pub mod table; pub mod table_of_contents; pub mod tabs;
pub mod textarea; pub mod toast; pub mod toggle; pub mod toggle_group;
pub mod toolbar; pub mod tooltip; pub mod tree; pub mod virtual_list;

// ── Boundary Exports ──────────────────────────────────────────────────────────

// Accordion
pub use accordion::accordion_boundary::{Accordion, AccordionItem, AccordionTrigger, AccordionContent};
// Alert
pub use alert::alert_boundary::Alert;
// Alert Dialog
pub use alert_dialog::alert_dialog_boundary::AlertDialog;
// Animate
pub use animate::animate_boundary::Animate;
// Aspect Ratio
pub use aspect_ratio::aspect_ratio_boundary::AspectRatio;
// Avatar
pub use avatar::avatar_boundary::{Avatar, AvatarImage, AvatarFallback};
// Badge
pub use badge::badge_boundary::Badge;
pub use badge::BadgeShowcasePreview;
// Banner
pub use banner::banner_boundary::Banner;
// Breadcrumb
pub use breadcrumb::breadcrumb_boundary::{Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbPage, BreadcrumbSeparator};
// Button
pub use button::button_boundary::Button;
pub use canonrs_core::primitives::{ButtonVariant, ButtonSize, ButtonType, ButtonStateHint};
pub use button::ButtonPreview;
// Button Group
pub use button_group::button_group_boundary::ButtonGroup;
// Callout
pub use callout::callout_boundary::Callout;
// Card
pub use card::card_boundary::{Card, CardHeader, CardTitle, CardDescription, CardContent};
// Carousel
pub use carousel::carousel_boundary::{Carousel, CarouselTrack, CarouselItem, CarouselPrev, CarouselNext};
// Chart
pub use chart::chart_boundary::Chart;
// Checkbox
pub use checkbox::checkbox_boundary::Checkbox;
// Code Block
pub use code_block::code_block_boundary::CodeBlock;
// Collapsible
pub use collapsible::collapsible_boundary::{Collapsible, CollapsibleTrigger, CollapsibleContent};
// Color Picker
pub use color_picker::color_picker_boundary::{ColorPicker, ColorPickerSwatch, ColorPickerDisplay, ColorPickerSwatches};
// Combobox
pub use combobox::combobox_boundary::Combobox;
// Command
pub use command::command_boundary::{Command, CommandItem};
// Confirm Dialog
pub use confirm_dialog::confirm_dialog_boundary::{ConfirmDialog, ConfirmDialogTrigger, ConfirmDialogPortal, ConfirmDialogOverlay, ConfirmDialogContent, ConfirmDialogTitle, ConfirmDialogDescription, ConfirmDialogFooter, ConfirmDialogCancel, ConfirmDialogConfirm, ConfirmDialogVariant};
// Context Menu
pub use context_menu::context_menu_boundary::{ContextMenu, ContextMenuTrigger, ContextMenuContent, ContextMenuItem};
// Copy Button
pub use copy_button::copy_button_boundary::CopyButton;
// Data Table
pub use data_table::data_table_boundary::DataTable;
// Dialog
pub use dialog::dialog_boundary::{Dialog, DialogTrigger, DialogPortal, DialogOverlay, DialogContent, DialogTitle, DialogDescription, DialogClose, DialogFooter};
// Doc Progress
pub use doc_progress::doc_progress_boundary::{DocProgress, DocProgressSlot};
// Drawer
pub use drawer::drawer_boundary::Drawer;
// Dropdown Menu
pub use dropdown_menu::dropdown_menu_boundary::{DropdownMenu, DropdownMenuTrigger, DropdownMenuContent, DropdownMenuGroup, DropdownMenuItem};
// Empty State
pub use empty_state::empty_state_boundary::{EmptyState, EmptyStateIcon, EmptyStateTitle, EmptyStateDescription, EmptyStateAction};
pub use canonrs_core::primitives::EmptyStateVariant;
// Empty Table
pub use empty_table::empty_table_boundary::EmptyTable;
// Error State
pub use error_state::error_state_boundary::{ErrorState, ErrorStateIcon, ErrorStateTitle, ErrorStateDescription, ErrorStateActions};
// Field
pub use field::field_boundary::Field;
// Form
pub use form::form_boundary::{Form, FormSection, FormActions, FormField, FormLabel};
// Form Error Summary
pub use form_error_summary::form_error_summary_boundary::FormErrorSummary;
// Hero
pub use hero::hero_boundary::{HeroTitle, HeroSubtitle, HeroDescription, HeroMedia, HeroActions};
// Hover Card
pub use hover_card::hover_card_boundary::{HoverCard, HoverCardTrigger, HoverCardContent};
// Icon
pub use icon::icon_boundary::Icon;
// Icon Button
pub use icon_button::icon_button_boundary::IconButton;
// Inline Meta
pub use inline_meta::inline_meta_boundary::{InlineMeta, InlineMetaLabel, InlineMetaValue};
// Inline Notice
pub use inline_notice::inline_notice_boundary::InlineNotice;
pub use canonrs_core::primitives::InlineNoticeVariant;
// Input
pub use input::input_boundary::Input;
// Input Group
pub use input_group::input_group_boundary::{InputGroup, InputGroupPrefix, InputGroupSuffix};
// Input OTP
pub use input_otp::input_otp_boundary::InputOtp;
// Kbd
pub use kbd::kbd_boundary::{Kbd, KbdGroup, KbdSeparator};
// Label
pub use label::label_boundary::Label;
// Link
pub use link::link_boundary::Link;
// Link Group
pub use link_group::link_group_boundary::LinkGroup;
// List Item
pub use list_item::list_item_boundary::{List, ListItem, ListItemTitle, ListItemDescription};
// Loading Overlay
pub use loading_overlay::loading_overlay_boundary::LoadingOverlay;
// Logo
pub use logo::logo_boundary::Logo;
// Markdown
pub use markdown::markdown_boundary::{MarkdownSurface, MarkdownLayout, MarkdownContent, MarkdownTOC};
// Menu
pub use menu::menu_boundary::{Menu, MenuItem};
// Menubar
pub use menubar::menubar_boundary::{Menubar, MenubarMenu, MenubarTrigger, MenubarContent, MenubarItem};
// Modal
pub use modal::modal_boundary::{Modal, ModalTrigger, ModalPortal, ModalOverlay, ModalContent};
// Nav Item
pub use nav_item::nav_item_boundary::{NavItem, NavGroup};
// Navigation Menu
pub use navigation_menu::navigation_menu_boundary::{NavigationMenu, NavigationMenuItem, NavigationMenuTrigger, NavigationMenuContent, NavigationMenuLink};
// Page Header
pub use page_header::page_header_boundary::{PageHeader, PageHeaderBreadcrumbs, PageHeaderContent, PageHeaderTitle, PageHeaderDescription};
// Pagination
pub use pagination::pagination_boundary::{Pagination, PaginationContent, PaginationItem, PaginationLink, PaginationPrevious};
// Popover
pub use popover::popover_boundary::{Popover, PopoverContent, PopoverTrigger};
// Progress
pub use progress::progress_boundary::Progress;
// Pulse
pub use pulse::pulse_boundary::Pulse;
// Radio
pub use radio::radio_boundary::{RadioGroup, Radio, RadioGroupItem};
// Resizable
pub use resizable::resizable_boundary::{Resizable, ResizablePanel, ResizableHandle};
// Scroll Area
pub use scroll_area::scroll_area_boundary::ScrollArea;
// Section
pub use section::section_boundary::{SectionHeader, SectionTitle, SectionSubtitle, SectionBadge, SectionActions};
// Select
pub use select::select_boundary::Select;
// Separator
pub use separator::separator_boundary::Separator;
// Sheet
pub use sheet::sheet_boundary::{Sheet, SheetOverlay, SheetContent};
// Sidebar
pub use sidebar::sidebar_boundary::{Sidebar, SidebarHeader, SidebarContent, SidebarFooter, SidebarMenu};
// Skeleton
pub use skeleton::skeleton_boundary::Skeleton;
// Slider
pub use slider::slider_boundary::{Slider, SliderWithMarks};
// Spinner
pub use spinner::spinner_boundary::Spinner;
// Stat
pub use stat::stat_boundary::{Stat, StatHeader, StatBody, StatValue, StatLabel};
// Status Dot
pub use status_dot::status_dot_boundary::StatusDot;
// Switch
pub use switch::switch_boundary::Switch;
// Table
pub use table::table_boundary::{Table, TableHeader, TableBody, TableFooter, TableRow};
// Table of Contents
pub use table_of_contents::table_of_contents_boundary::TableOfContents;
// Tabs
pub use tabs::tabs_boundary::{TabsRoot, TabsListBoundary, TabsTrigger, TabsContent};
// Textarea
pub use textarea::textarea_boundary::Textarea;
// Toast
pub use toast::toast_boundary::{Toast, ToastViewport};
pub use canonrs_core::primitives::ToastVariant;
// Toggle
pub use toggle::toggle_boundary::Toggle;
// Toggle Group
pub use toggle_group::toggle_group_boundary::{ToggleGroup, ToggleGroupItem};
// Toolbar
pub use toolbar::toolbar_boundary::{Toolbar, ToolbarItem, ToolbarSeparator};
// Tooltip
pub use tooltip::tooltip_boundary::{TooltipProvider, Tooltip, TooltipTrigger, TooltipContent};
// Tree
pub use tree::tree_boundary::{Tree, TreeItem, TreeGroup};
// Virtual List
pub use virtual_list::virtual_list_boundary::VirtualList;

// ── Showcase Previews (catalog only) ─────────────────────────────────────────
pub use button_group::ButtonGroupShowcasePreview;
pub use icon_button::IconButtonShowcasePreview;
pub use copy_button::CopyButtonShowcasePreview;
pub use link::LinkShowcasePreview;
pub use select::SelectShowcasePreview;
pub use combobox::ComboboxShowcasePreview;
pub use radio::RadioShowcasePreview;
pub use radio_group::RadioGroupShowcasePreview;
pub use color_picker::ColorPickerShowcasePreview;
pub use slider::SliderShowcasePreview;
pub use tabs::TabsShowcasePreview;
pub use table_of_contents::TableOfContentsShowcasePreview;
pub use accordion::AccordionShowcasePreview;
pub use collapsible::CollapsibleShowcasePreview;
pub use dropdown_menu::DropdownMenuShowcasePreview;
pub use context_menu::ContextMenuShowcasePreview;
pub use menubar::MenubarShowcasePreview;
pub use menu::MenuShowcasePreview;
pub use command::CommandShowcasePreview;
pub use navigation_menu::NavigationMenuShowcasePreview;
pub use sidebar::SidebarShowcasePreview;
pub use nav_item::NavItemShowcasePreview;
pub use breadcrumb::BreadcrumbShowcasePreview;
pub use pagination::PaginationShowcasePreview;
pub use link_group::LinkGroupShowcasePreview;
pub use toast::ToastShowcasePreview;
pub use alert::AlertShowcasePreview;
pub use banner::BannerShowcasePreview;
pub use callout::CalloutShowcasePreview;
pub use inline_notice::InlineNoticeShowcasePreview;
pub use status_dot::StatusDotShowcasePreview;
pub use dialog::DialogShowcasePreview;
pub use alert_dialog::AlertDialogShowcasePreview;
pub use drawer::DrawerShowcasePreview;
pub use sheet::SheetShowcasePreview;
pub use modal::ModalShowcasePreview;
pub use confirm_dialog::ConfirmDialogShowcasePreview;
pub use tooltip::TooltipShowcasePreview;
pub use hover_card::HoverCardShowcasePreview;
pub use popover::PopoverShowcasePreview;
pub use form::FormShowcasePreview;
pub use input::InputShowcasePreview;
pub use input_group::InputGroupShowcasePreview;
pub use input_otp::InputOtpShowcasePreview;
pub use textarea::TextareaShowcasePreview;
pub use field::FieldShowcasePreview;
pub use label::LabelShowcasePreview;
pub use checkbox::CheckboxShowcasePreview;
pub use form_error_summary::FormErrorSummaryShowcasePreview;
pub use table::TableShowcasePreview;
pub use data_table::DataTableStaticShowcasePreview;
pub use virtual_list::VirtualListShowcasePreview;
pub use empty_table::EmptyTableShowcasePreview;
pub use tree::TreeShowcasePreview;
pub use list_item::ListItemShowcasePreview;
pub use switch::SwitchShowcasePreview;
pub use toggle::ToggleShowcasePreview;
pub use toggle_group::ToggleGroupShowcasePreview;
pub use progress::ProgressShowcasePreview;
pub use spinner::SpinnerShowcasePreview;
pub use skeleton::SkeletonShowcasePreview;
pub use pulse::PulseShowcasePreview;
pub use loading_overlay::LoadingOverlayShowcasePreview;
pub use doc_progress::DocProgressShowcasePreview;
pub use card::CardShowcasePreview;
pub use resizable::ResizableShowcasePreview;
pub use scroll_area::ScrollAreaShowcasePreview;
pub use aspect_ratio::AspectRatioShowcasePreview;
pub use page_header::PageHeaderShowcasePreview;
pub use toolbar::ToolbarShowcasePreview;
pub use separator::SeparatorShowcasePreview;
pub use avatar::AvatarShowcasePreview;
pub use icon::IconShowcasePreview;
pub use logo::LogoShowcasePreview;
pub use code_block::CodeBlockShowcasePreview;
pub use markdown::MarkdownShowcasePreview;
pub use chart::ChartShowcasePreview;
pub use stat::StatShowcasePreview;
pub use inline_meta::InlineMetaShowcasePreview;
pub use kbd::KbdShowcasePreview;
pub use carousel::CarouselShowcasePreview;
pub use empty_state::EmptyStateShowcasePreview;
pub use error_state::ErrorStateShowcasePreview;
pub use animate::AnimateShowcasePreview;
