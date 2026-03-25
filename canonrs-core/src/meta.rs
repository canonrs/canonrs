//! @canon-level: strict
//! @canon-owner: primitives-team
//! ComponentMeta — metadata layer para AI/RAG/Decision Engine

#[derive(Debug, Clone, PartialEq)]
pub struct ComponentMeta {
    pub name: &'static str,
    pub family: ComponentFamily,
    pub intent: &'static str,
    pub capabilities: &'static [&'static str],
    pub composable: bool,
    pub required_parts: &'static [&'static str],
    pub optional_parts: &'static [&'static str],
}

#[derive(Debug, Clone, PartialEq)]
pub enum ComponentFamily {
    Overlay,
    Input,
    Feedback,
    Navigation,
    Layout,
    DataDisplay,
    Typography,
    Interactive,
    Utility,
}

impl ComponentFamily {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Overlay     => "overlay",
            Self::Input       => "input",
            Self::Feedback    => "feedback",
            Self::Navigation  => "navigation",
            Self::Layout      => "layout",
            Self::DataDisplay => "data-display",
            Self::Typography  => "typography",
            Self::Interactive => "interactive",
            Self::Utility     => "utility",
        }
    }
}

pub trait HasMeta {
    fn meta() -> &'static ComponentMeta;
}

// ── OVERLAY ─────────────────────────────────────────────────────────────────
pub static DIALOG_META: ComponentMeta = ComponentMeta {
    name: "Dialog", family: ComponentFamily::Overlay,
    intent: "Display critical content requiring user interaction",
    capabilities: &["open-close", "focus-trap", "keyboard-esc", "aria-modal"],
    composable: true,
    required_parts: &["DialogContent", "DialogTitle"],
    optional_parts: &["DialogOverlay", "DialogDescription", "DialogClose"],
};
pub static ALERT_DIALOG_META: ComponentMeta = ComponentMeta {
    name: "AlertDialog", family: ComponentFamily::Overlay,
    intent: "Confirm destructive or irreversible actions",
    capabilities: &["open-close", "focus-trap", "keyboard-esc", "aria-modal"],
    composable: true,
    required_parts: &["AlertDialogContent", "AlertDialogTitle", "AlertDialogAction"],
    optional_parts: &["AlertDialogOverlay", "AlertDialogDescription", "AlertDialogCancel"],
};
pub static DRAWER_META: ComponentMeta = ComponentMeta {
    name: "Drawer", family: ComponentFamily::Overlay,
    intent: "Slide-in panel for supplementary content",
    capabilities: &["open-close", "focus-trap", "keyboard-esc", "aria-modal"],
    composable: true,
    required_parts: &["DrawerContent"],
    optional_parts: &["DrawerOverlay"],
};
pub static SHEET_META: ComponentMeta = ComponentMeta {
    name: "Sheet", family: ComponentFamily::Overlay,
    intent: "Side panel for forms or navigation",
    capabilities: &["open-close", "focus-trap", "keyboard-esc", "slide-side"],
    composable: true,
    required_parts: &["SheetContent"],
    optional_parts: &["SheetOverlay"],
};
pub static MODAL_META: ComponentMeta = ComponentMeta {
    name: "Modal", family: ComponentFamily::Overlay,
    intent: "Generic modal container",
    capabilities: &["open-close", "focus-trap", "aria-modal"],
    composable: true,
    required_parts: &[],
    optional_parts: &[],
};
pub static HOVER_CARD_META: ComponentMeta = ComponentMeta {
    name: "HoverCard", family: ComponentFamily::Overlay,
    intent: "Show rich preview on hover",
    capabilities: &["open-close", "hover"],
    composable: true,
    required_parts: &["HoverCardTrigger", "HoverCardContent"],
    optional_parts: &[],
};
pub static POPOVER_META: ComponentMeta = ComponentMeta {
    name: "Popover", family: ComponentFamily::Overlay,
    intent: "Show contextual floating content",
    capabilities: &["open-close", "focus-trap"],
    composable: true,
    required_parts: &["PopoverTrigger", "PopoverContent"],
    optional_parts: &[],
};
pub static TOOLTIP_META: ComponentMeta = ComponentMeta {
    name: "Tooltip", family: ComponentFamily::Overlay,
    intent: "Show brief label on hover/focus",
    capabilities: &["open-close", "hover", "focus", "aria-describedby"],
    composable: true,
    required_parts: &["TooltipTrigger", "TooltipContent"],
    optional_parts: &["TooltipProvider"],
};

// ── INPUT ────────────────────────────────────────────────────────────────────
pub static BUTTON_META: ComponentMeta = ComponentMeta {
    name: "Button", family: ComponentFamily::Interactive,
    intent: "Trigger an action or event",
    capabilities: &["click", "disabled", "variant", "size"],
    composable: false, required_parts: &[], optional_parts: &[],
};
pub static INPUT_META: ComponentMeta = ComponentMeta {
    name: "Input", family: ComponentFamily::Input,
    intent: "Capture text or data from user",
    capabilities: &["value", "disabled", "placeholder", "variant", "size"],
    composable: false, required_parts: &[], optional_parts: &[],
};
pub static TEXTAREA_META: ComponentMeta = ComponentMeta {
    name: "Textarea", family: ComponentFamily::Input,
    intent: "Capture multi-line text from user",
    capabilities: &["value", "disabled", "readonly", "rows"],
    composable: false, required_parts: &[], optional_parts: &[],
};
pub static SELECT_META: ComponentMeta = ComponentMeta {
    name: "Select", family: ComponentFamily::Input,
    intent: "Choose one option from a list",
    capabilities: &["open-close", "keyboard", "disabled", "size"],
    composable: true,
    required_parts: &["SelectTrigger", "SelectContent", "SelectItem"],
    optional_parts: &["SelectValue", "SelectSeparator"],
};
pub static CHECKBOX_META: ComponentMeta = ComponentMeta {
    name: "Checkbox", family: ComponentFamily::Input,
    intent: "Toggle a boolean value",
    capabilities: &["checked", "disabled"],
    composable: false, required_parts: &[], optional_parts: &[],
};
pub static RADIO_META: ComponentMeta = ComponentMeta {
    name: "Radio", family: ComponentFamily::Input,
    intent: "Select one option from a group",
    capabilities: &["checked", "disabled", "group"],
    composable: true,
    required_parts: &["RadioGroup"],
    optional_parts: &[],
};
pub static SWITCH_META: ComponentMeta = ComponentMeta {
    name: "Switch", family: ComponentFamily::Input,
    intent: "Toggle between on and off states",
    capabilities: &["checked", "disabled"],
    composable: false, required_parts: &[], optional_parts: &[],
};
pub static SLIDER_META: ComponentMeta = ComponentMeta {
    name: "Slider", family: ComponentFamily::Input,
    intent: "Select a value within a range",
    capabilities: &["value", "min", "max", "disabled", "keyboard"],
    composable: true,
    required_parts: &["SliderTrack", "SliderThumb"],
    optional_parts: &["SliderRange"],
};
pub static TOGGLE_META: ComponentMeta = ComponentMeta {
    name: "Toggle", family: ComponentFamily::Input,
    intent: "Toggle a pressed state",
    capabilities: &["pressed", "disabled"],
    composable: false, required_parts: &[], optional_parts: &[],
};
pub static COLOR_PICKER_META: ComponentMeta = ComponentMeta {
    name: "ColorPicker", family: ComponentFamily::Input,
    intent: "Select a color value",
    capabilities: &["value", "disabled"],
    composable: true, required_parts: &[], optional_parts: &[],
};
pub static COMBOBOX_META: ComponentMeta = ComponentMeta {
    name: "Combobox", family: ComponentFamily::Input,
    intent: "Search and select from a list",
    capabilities: &["open-close", "search", "keyboard", "disabled"],
    composable: true,
    required_parts: &["SelectTrigger", "SelectContent"],
    optional_parts: &[],
};

// ── FEEDBACK ─────────────────────────────────────────────────────────────────
pub static ALERT_META: ComponentMeta = ComponentMeta {
    name: "Alert", family: ComponentFamily::Feedback,
    intent: "Display important static messages",
    capabilities: &["variant", "aria-live", "aria-atomic"],
    composable: true, required_parts: &[],
    optional_parts: &["AlertTitle", "AlertDescription"],
};
pub static TOAST_META: ComponentMeta = ComponentMeta {
    name: "Toast", family: ComponentFamily::Feedback,
    intent: "Show brief non-blocking notifications",
    capabilities: &["open-close", "variant", "aria-live", "auto-dismiss"],
    composable: true,
    required_parts: &["ToastViewport"],
    optional_parts: &["ToastTitle", "ToastDescription", "ToastAction", "ToastClose"],
};
pub static BANNER_META: ComponentMeta = ComponentMeta {
    name: "Banner", family: ComponentFamily::Feedback,
    intent: "Display persistent page-level messages",
    capabilities: &["variant", "open-close", "aria-live"],
    composable: true, required_parts: &[],
    optional_parts: &["BannerContent", "BannerActions", "BannerClose"],
};
pub static CALLOUT_META: ComponentMeta = ComponentMeta {
    name: "Callout", family: ComponentFamily::Feedback,
    intent: "Highlight important contextual information",
    capabilities: &["variant"],
    composable: true, required_parts: &[],
    optional_parts: &["CalloutIcon", "CalloutTitle", "CalloutDescription"],
};
pub static INLINE_NOTICE_META: ComponentMeta = ComponentMeta {
    name: "InlineNotice", family: ComponentFamily::Feedback,
    intent: "Show inline contextual feedback",
    capabilities: &["variant"],
    composable: true, required_parts: &[],
    optional_parts: &["InlineNoticeIcon", "InlineNoticeContent"],
};
pub static PROGRESS_META: ComponentMeta = ComponentMeta {
    name: "Progress", family: ComponentFamily::Feedback,
    intent: "Show completion of a task",
    capabilities: &["value", "aria-progressbar"],
    composable: true,
    required_parts: &["ProgressIndicator"],
    optional_parts: &[],
};
pub static SPINNER_META: ComponentMeta = ComponentMeta {
    name: "Spinner", family: ComponentFamily::Feedback,
    intent: "Indicate loading or processing",
    capabilities: &["size", "paused", "aria-busy"],
    composable: false, required_parts: &[], optional_parts: &[],
};
pub static SKELETON_META: ComponentMeta = ComponentMeta {
    name: "Skeleton", family: ComponentFamily::Feedback,
    intent: "Show placeholder while content loads",
    capabilities: &["aria-busy"],
    composable: false, required_parts: &[], optional_parts: &[],
};
pub static PULSE_META: ComponentMeta = ComponentMeta {
    name: "Pulse", family: ComponentFamily::Feedback,
    intent: "Animated attention indicator",
    capabilities: &["animate"],
    composable: false, required_parts: &[], optional_parts: &[],
};
pub static EMPTY_STATE_META: ComponentMeta = ComponentMeta {
    name: "EmptyState", family: ComponentFamily::Feedback,
    intent: "Display when no content is available",
    capabilities: &[],
    composable: true, required_parts: &[],
    optional_parts: &[],
};
pub static ERROR_STATE_META: ComponentMeta = ComponentMeta {
    name: "ErrorState", family: ComponentFamily::Feedback,
    intent: "Display error condition to user",
    capabilities: &[],
    composable: true, required_parts: &[],
    optional_parts: &[],
};

// ── NAVIGATION ───────────────────────────────────────────────────────────────
pub static TABS_META: ComponentMeta = ComponentMeta {
    name: "Tabs", family: ComponentFamily::Navigation,
    intent: "Switch between related content panels",
    capabilities: &["active-state", "keyboard", "aria-tablist"],
    composable: true,
    required_parts: &["TabsList", "TabsTrigger", "TabsContent"],
    optional_parts: &[],
};
pub static BREADCRUMB_META: ComponentMeta = ComponentMeta {
    name: "Breadcrumb", family: ComponentFamily::Navigation,
    intent: "Show current location in hierarchy",
    capabilities: &["aria-current"],
    composable: true, required_parts: &[],
    optional_parts: &["BreadcrumbItem", "BreadcrumbLink", "BreadcrumbSeparator"],
};
pub static PAGINATION_META: ComponentMeta = ComponentMeta {
    name: "Pagination", family: ComponentFamily::Navigation,
    intent: "Navigate between pages of content",
    capabilities: &["active-state", "disabled", "aria-current"],
    composable: true, required_parts: &[],
    optional_parts: &["PaginationContent", "PaginationItem", "PaginationLink"],
};
pub static NAVIGATION_MENU_META: ComponentMeta = ComponentMeta {
    name: "NavigationMenu", family: ComponentFamily::Navigation,
    intent: "Primary site navigation with submenus",
    capabilities: &["open-close", "keyboard", "hover"],
    composable: true,
    required_parts: &["NavigationMenuList", "NavigationMenuItem"],
    optional_parts: &["NavigationMenuTrigger", "NavigationMenuContent", "NavigationMenuLink"],
};
pub static SIDEBAR_META: ComponentMeta = ComponentMeta {
    name: "Sidebar", family: ComponentFamily::Navigation,
    intent: "Vertical navigation panel",
    capabilities: &["open-close", "collapsible"],
    composable: true,
    required_parts: &["SidebarContent"],
    optional_parts: &["SidebarHeader", "SidebarFooter", "SidebarMenu", "SidebarMenuItem"],
};
pub static NAV_ITEM_META: ComponentMeta = ComponentMeta {
    name: "NavItem", family: ComponentFamily::Navigation,
    intent: "Single navigation link item",
    capabilities: &["active-state", "disabled"],
    composable: false, required_parts: &[], optional_parts: &[],
};

// ── LAYOUT ───────────────────────────────────────────────────────────────────
pub static CARD_META: ComponentMeta = ComponentMeta {
    name: "Card", family: ComponentFamily::Layout,
    intent: "Group related content in a container",
    capabilities: &["composable"],
    composable: true, required_parts: &[],
    optional_parts: &["CardHeader", "CardTitle", "CardDescription", "CardContent", "CardFooter"],
};
pub static SEPARATOR_META: ComponentMeta = ComponentMeta {
    name: "Separator", family: ComponentFamily::Layout,
    intent: "Visually divide content sections",
    capabilities: &["orientation", "decorative"],
    composable: false, required_parts: &[], optional_parts: &[],
};
pub static RESIZABLE_META: ComponentMeta = ComponentMeta {
    name: "Resizable", family: ComponentFamily::Layout,
    intent: "Split panels with draggable divider",
    capabilities: &["drag", "resize", "orientation"],
    composable: true,
    required_parts: &["ResizablePanel", "ResizableHandle"],
    optional_parts: &[],
};
pub static SCROLL_AREA_META: ComponentMeta = ComponentMeta {
    name: "ScrollArea", family: ComponentFamily::Layout,
    intent: "Scrollable container with custom scrollbar",
    capabilities: &["scroll"],
    composable: false, required_parts: &[], optional_parts: &[],
};
pub static ASPECT_RATIO_META: ComponentMeta = ComponentMeta {
    name: "AspectRatio", family: ComponentFamily::Layout,
    intent: "Maintain consistent width/height ratio",
    capabilities: &[],
    composable: false, required_parts: &[], optional_parts: &[],
};
pub static COLLAPSIBLE_META: ComponentMeta = ComponentMeta {
    name: "Collapsible", family: ComponentFamily::Layout,
    intent: "Show and hide content sections",
    capabilities: &["open-close", "keyboard"],
    composable: true,
    required_parts: &["CollapsibleTrigger", "CollapsibleContent"],
    optional_parts: &[],
};
pub static ACCORDION_META: ComponentMeta = ComponentMeta {
    name: "Accordion", family: ComponentFamily::Layout,
    intent: "Expand and collapse content sections",
    capabilities: &["open-close", "single", "multiple", "collapsible", "keyboard"],
    composable: true,
    required_parts: &["AccordionItem", "AccordionTrigger", "AccordionContent"],
    optional_parts: &[],
};

// ── DATA DISPLAY ─────────────────────────────────────────────────────────────
pub static BADGE_META: ComponentMeta = ComponentMeta {
    name: "Badge", family: ComponentFamily::DataDisplay,
    intent: "Display status, count or label",
    capabilities: &["variant"],
    composable: false, required_parts: &[], optional_parts: &[],
};
pub static TABLE_META: ComponentMeta = ComponentMeta {
    name: "Table", family: ComponentFamily::DataDisplay,
    intent: "Display tabular data",
    capabilities: &["striped", "selected-row", "aria-table"],
    composable: true,
    required_parts: &["TableHeader", "TableBody", "TableRow", "TableHead", "TableCell"],
    optional_parts: &["TableFooter", "TableCaption", "TableWrapper"],
};
pub static DATA_TABLE_META: ComponentMeta = ComponentMeta {
    name: "DataTable", family: ComponentFamily::DataDisplay,
    intent: "Display sortable, filterable tabular data",
    capabilities: &["sort", "filter", "pagination", "selection"],
    composable: true, required_parts: &[], optional_parts: &[],
};
pub static STAT_META: ComponentMeta = ComponentMeta {
    name: "Stat", family: ComponentFamily::DataDisplay,
    intent: "Display a key metric with label",
    capabilities: &[],
    composable: true, required_parts: &["StatValue", "StatLabel"],
    optional_parts: &["StatDelta", "StatIcon"],
};
pub static STATUS_DOT_META: ComponentMeta = ComponentMeta {
    name: "StatusDot", family: ComponentFamily::DataDisplay,
    intent: "Indicate user presence or availability",
    capabilities: &["variant"],
    composable: false, required_parts: &[], optional_parts: &[],
};
pub static AVATAR_META: ComponentMeta = ComponentMeta {
    name: "Avatar", family: ComponentFamily::DataDisplay,
    intent: "Display user profile image with fallback",
    capabilities: &["fallback"],
    composable: true,
    required_parts: &[],
    optional_parts: &["AvatarImage", "AvatarFallback"],
};
pub static CAROUSEL_META: ComponentMeta = ComponentMeta {
    name: "Carousel", family: ComponentFamily::DataDisplay,
    intent: "Cycle through items horizontally",
    capabilities: &["prev-next", "indicators", "keyboard", "aria-roledescription"],
    composable: true,
    required_parts: &["CarouselTrack", "CarouselItem"],
    optional_parts: &["CarouselPrev", "CarouselNext", "CarouselIndicators"],
};
pub static CHART_META: ComponentMeta = ComponentMeta {
    name: "Chart", family: ComponentFamily::DataDisplay,
    intent: "Visualize data graphically",
    capabilities: &["responsive"],
    composable: false, required_parts: &[], optional_parts: &[],
};
pub static TREE_META: ComponentMeta = ComponentMeta {
    name: "Tree", family: ComponentFamily::DataDisplay,
    intent: "Display hierarchical data",
    capabilities: &["expand-collapse", "selected", "keyboard", "aria-tree"],
    composable: true,
    required_parts: &["TreeItem"],
    optional_parts: &["TreeGroup"],
};
pub static VIRTUAL_LIST_META: ComponentMeta = ComponentMeta {
    name: "VirtualList", family: ComponentFamily::DataDisplay,
    intent: "Efficiently render large lists",
    capabilities: &["virtual-scroll"],
    composable: false, required_parts: &[], optional_parts: &[],
};

// ── TYPOGRAPHY ───────────────────────────────────────────────────────────────
pub static LINK_META: ComponentMeta = ComponentMeta {
    name: "Link", family: ComponentFamily::Typography,
    intent: "Navigate to a URL or trigger action",
    capabilities: &["variant", "external", "disabled"],
    composable: false, required_parts: &[], optional_parts: &[],
};
pub static KBD_META: ComponentMeta = ComponentMeta {
    name: "Kbd", family: ComponentFamily::Typography,
    intent: "Display keyboard shortcut",
    capabilities: &[],
    composable: false, required_parts: &[], optional_parts: &[],
};
pub static MARKDOWN_META: ComponentMeta = ComponentMeta {
    name: "Markdown", family: ComponentFamily::Typography,
    intent: "Render markdown content",
    capabilities: &[],
    composable: false, required_parts: &[], optional_parts: &[],
};

// ── INTERACTIVE ──────────────────────────────────────────────────────────────
pub static DROPDOWN_MENU_META: ComponentMeta = ComponentMeta {
    name: "DropdownMenu", family: ComponentFamily::Interactive,
    intent: "Show contextual action menu",
    capabilities: &["open-close", "keyboard", "disabled-item"],
    composable: true,
    required_parts: &["DropdownMenuTrigger", "DropdownMenuContent"],
    optional_parts: &["DropdownMenuItem", "DropdownMenuSeparator", "DropdownMenuGroup"],
};
pub static CONTEXT_MENU_META: ComponentMeta = ComponentMeta {
    name: "ContextMenu", family: ComponentFamily::Interactive,
    intent: "Show menu on right-click",
    capabilities: &["open-close", "keyboard", "right-click"],
    composable: true,
    required_parts: &["ContextMenuTrigger", "ContextMenuContent"],
    optional_parts: &["ContextMenuItem", "ContextMenuSeparator"],
};
pub static MENUBAR_META: ComponentMeta = ComponentMeta {
    name: "Menubar", family: ComponentFamily::Interactive,
    intent: "Horizontal application menu bar",
    capabilities: &["open-close", "keyboard"],
    composable: true,
    required_parts: &["MenubarMenu", "MenubarTrigger"],
    optional_parts: &["MenubarContent", "MenubarItem", "MenubarSeparator"],
};
pub static COMMAND_META: ComponentMeta = ComponentMeta {
    name: "Command", family: ComponentFamily::Interactive,
    intent: "Command palette for quick actions",
    capabilities: &["search", "keyboard", "group"],
    composable: true,
    required_parts: &["CommandInput", "CommandList"],
    optional_parts: &["CommandItem", "CommandGroup", "CommandSeparator", "CommandEmpty"],
};
pub static TOOLBAR_META: ComponentMeta = ComponentMeta {
    name: "Toolbar", family: ComponentFamily::Interactive,
    intent: "Group of action buttons or controls",
    capabilities: &["orientation", "keyboard", "aria-toolbar"],
    composable: true, required_parts: &[],
    optional_parts: &["ToolbarSeparator"],
};
pub static TOGGLE_GROUP_META: ComponentMeta = ComponentMeta {
    name: "ToggleGroup", family: ComponentFamily::Interactive,
    intent: "Group of toggle buttons with single or multiple selection",
    capabilities: &["single", "multiple", "disabled"],
    composable: true, required_parts: &[], optional_parts: &[],
};

// ── UTILITY ──────────────────────────────────────────────────────────────────
pub static ANIMATE_META: ComponentMeta = ComponentMeta {
    name: "Animate", family: ComponentFamily::Utility,
    intent: "Apply CSS animations to children",
    capabilities: &["animate"],
    composable: false, required_parts: &[], optional_parts: &[],
};
pub static LOADING_OVERLAY_META: ComponentMeta = ComponentMeta {
    name: "LoadingOverlay", family: ComponentFamily::Utility,
    intent: "Block UI during async operations",
    capabilities: &["open-close", "aria-busy"],
    composable: false, required_parts: &[], optional_parts: &[],
};
pub static ICON_META: ComponentMeta = ComponentMeta {
    name: "Icon", family: ComponentFamily::Utility,
    intent: "Display an SVG icon",
    capabilities: &["size", "aria-hidden"],
    composable: false, required_parts: &[], optional_parts: &[],
};

/// Retorna todos os metas — usado por RAG/Decision Engine
pub fn all_meta() -> Vec<&'static ComponentMeta> {
    vec![
        &DIALOG_META, &ALERT_DIALOG_META, &DRAWER_META, &SHEET_META,
        &MODAL_META, &HOVER_CARD_META, &POPOVER_META, &TOOLTIP_META,
        &BUTTON_META, &INPUT_META, &TEXTAREA_META, &SELECT_META,
        &CHECKBOX_META, &RADIO_META, &SWITCH_META, &SLIDER_META,
        &TOGGLE_META, &COLOR_PICKER_META, &COMBOBOX_META,
        &ALERT_META, &TOAST_META, &BANNER_META, &CALLOUT_META,
        &INLINE_NOTICE_META, &PROGRESS_META, &SPINNER_META,
        &SKELETON_META, &PULSE_META, &EMPTY_STATE_META, &ERROR_STATE_META,
        &TABS_META, &BREADCRUMB_META, &PAGINATION_META,
        &NAVIGATION_MENU_META, &SIDEBAR_META, &NAV_ITEM_META,
        &CARD_META, &SEPARATOR_META, &RESIZABLE_META, &SCROLL_AREA_META,
        &ASPECT_RATIO_META, &COLLAPSIBLE_META, &ACCORDION_META,
        &BADGE_META, &TABLE_META, &DATA_TABLE_META, &STAT_META,
        &STATUS_DOT_META, &AVATAR_META, &CAROUSEL_META, &CHART_META,
        &TREE_META, &VIRTUAL_LIST_META,
        &LINK_META, &KBD_META, &MARKDOWN_META,
        &DROPDOWN_MENU_META, &CONTEXT_MENU_META, &MENUBAR_META,
        &COMMAND_META, &TOOLBAR_META, &TOGGLE_GROUP_META,
        &ANIMATE_META, &LOADING_OVERLAY_META, &ICON_META,
    ]
}

// ── STATE ENUMS POR DOMÍNIO ──────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum VisibilityState {
    #[default]
    Closed,
    Open,
}
impl VisibilityState {
    pub fn as_str(&self) -> &'static str {
        match self { Self::Open => "open", Self::Closed => "closed" }
    }
}
impl From<bool> for VisibilityState {
    fn from(open: bool) -> Self {
        if open { Self::Open } else { Self::Closed }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum ActivityState {
    #[default]
    Inactive,
    Active,
}
impl ActivityState {
    pub fn as_str(&self) -> &'static str {
        match self { Self::Active => "active", Self::Inactive => "inactive" }
    }
}
impl From<bool> for ActivityState {
    fn from(active: bool) -> Self {
        if active { Self::Active } else { Self::Inactive }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum SelectionState {
    #[default]
    Unselected,
    Selected,
}
impl SelectionState {
    pub fn as_str(&self) -> &'static str {
        match self { Self::Selected => "selected", Self::Unselected => "unselected" }
    }
}
impl From<bool> for SelectionState {
    fn from(selected: bool) -> Self {
        if selected { Self::Selected } else { Self::Unselected }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum ToggleState {
    #[default]
    Off,
    On,
}
impl ToggleState {
    pub fn as_str(&self) -> &'static str {
        match self { Self::On => "on", Self::Off => "off" }
    }
}
impl From<bool> for ToggleState {
    fn from(pressed: bool) -> Self {
        if pressed { Self::On } else { Self::Off }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum NavigationState {
    #[default]
    Inactive,
    Current,
}
impl NavigationState {
    pub fn as_str(&self) -> &'static str {
        match self { Self::Current => "current", Self::Inactive => "inactive" }
    }
}
impl From<bool> for NavigationState {
    fn from(current: bool) -> Self {
        if current { Self::Current } else { Self::Inactive }
    }
}

// ── TO DATA ATTR TRAIT ───────────────────────────────────────────────────────

/// Centraliza a saída DOM de qualquer state enum.
/// Garante que data-rs-state sempre vem de um tipo, nunca de string literal.
pub trait ToDataAttr {
    fn to_data_attr(&self) -> (&'static str, &'static str);
}

impl ToDataAttr for VisibilityState {
    fn to_data_attr(&self) -> (&'static str, &'static str) {
        ("data-rs-state", self.as_str())
    }
}
impl ToDataAttr for ActivityState {
    fn to_data_attr(&self) -> (&'static str, &'static str) {
        ("data-rs-state", self.as_str())
    }
}
impl ToDataAttr for SelectionState {
    fn to_data_attr(&self) -> (&'static str, &'static str) {
        ("data-rs-state", self.as_str())
    }
}
impl ToDataAttr for ToggleState {
    fn to_data_attr(&self) -> (&'static str, &'static str) {
        ("data-rs-state", self.as_str())
    }
}
impl ToDataAttr for NavigationState {
    fn to_data_attr(&self) -> (&'static str, &'static str) {
        ("data-rs-state", self.as_str())
    }
}

// ── STATE KIND (unificador sem ambiguidade) ───────────────────────────────────

/// Wrapper unificado para passar qualquer state enum por referência comum.
/// Resolve ambiguidade sem colapsar domínios distintos num enum flat.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StateKind {
    Visibility(VisibilityState),
    Activity(ActivityState),
    Selection(SelectionState),
    Toggle(ToggleState),
    Navigation(NavigationState),
}

impl StateKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Visibility(s)  => s.as_str(),
            Self::Activity(s)    => s.as_str(),
            Self::Selection(s)   => s.as_str(),
            Self::Toggle(s)      => s.as_str(),
            Self::Navigation(s)  => s.as_str(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum DisabledState {
    #[default]
    Enabled,
    Disabled,
}
impl DisabledState {
    pub fn as_str(&self) -> &'static str {
        match self { Self::Enabled => "enabled", Self::Disabled => "disabled" }
    }
    pub fn as_bool(&self) -> bool { *self == Self::Disabled }
    pub fn aria(&self) -> Option<&'static str> {
        if self.as_bool() { Some("true") } else { None }
    }
}
impl From<bool> for DisabledState {
    fn from(d: bool) -> Self { if d { Self::Disabled } else { Self::Enabled } }
}
