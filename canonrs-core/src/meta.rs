//! @canon-level: strict
//! @canon-owner: primitives-team
//! ComponentMeta — metadata layer para AI/RAG/Decision Engine


/// Typed capability enum — substitui &["open-close"] strings hardcoded
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Capability {
    OpenClose,
    Selected,
    Active,
    Pressed,
    Disabled,
    Loading,
    Value,
    Multiple,
    Orientation,
    // Pure behaviors — sem props derivadas
    FocusTrap,
    KeyboardEsc,
    AriaModal,
    KeyboardArrows,
    Roving,
    Typeahead,
    VirtualScroll,
    DragDrop,
    Resize,
    Overflow,
    Animation,
}

impl Capability {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::OpenClose      => "open-close",
            Self::Selected       => "selected",
            Self::Active         => "active",
            Self::Pressed        => "pressed",
            Self::Disabled       => "disabled",
            Self::Loading        => "loading",
            Self::Value          => "value",
            Self::Multiple       => "multiple",
            Self::Orientation    => "orientation",
            Self::FocusTrap      => "focus-trap",
            Self::KeyboardEsc    => "keyboard-esc",
            Self::AriaModal      => "aria-modal",
            Self::KeyboardArrows => "keyboard-arrows",
            Self::Roving         => "roving",
            Self::Typeahead      => "typeahead",
            Self::VirtualScroll  => "virtual-scroll",
            Self::DragDrop       => "drag-drop",
            Self::Resize         => "resize",
            Self::Overflow       => "overflow",
            Self::Animation      => "animation",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ComponentMeta {
    pub id:   &'static str,
    pub name: &'static str,
    pub family: ComponentFamily,
    pub intent: &'static str,
    pub capabilities: &'static [Capability],
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
    id: "dialog",        name: "Dialog", family: ComponentFamily::Overlay,
    intent: "Display critical content requiring user interaction",
    capabilities: &[Capability::OpenClose, Capability::FocusTrap, Capability::KeyboardEsc, Capability::AriaModal],
    composable: true,
    required_parts: &["DialogContent", "DialogTitle"],
    optional_parts: &["DialogOverlay", "DialogDescription", "DialogClose"],
};
pub static ALERT_DIALOG_META: ComponentMeta = ComponentMeta {
    id: "alert-dialog",  name: "AlertDialog", family: ComponentFamily::Overlay,
    intent: "Confirm destructive or irreversible actions",
    capabilities: &[Capability::OpenClose, Capability::FocusTrap, Capability::KeyboardEsc, Capability::AriaModal],
    composable: true,
    required_parts: &["AlertDialogContent", "AlertDialogTitle", "AlertDialogAction"],
    optional_parts: &["AlertDialogOverlay", "AlertDialogDescription", "AlertDialogCancel"],
};
pub static DRAWER_META: ComponentMeta = ComponentMeta {
    id: "drawer",        name: "Drawer", family: ComponentFamily::Overlay,
    intent: "Slide-in panel for supplementary content",
    capabilities: &[Capability::OpenClose, Capability::FocusTrap, Capability::KeyboardEsc, Capability::AriaModal],
    composable: true,
    required_parts: &["DrawerContent"],
    optional_parts: &["DrawerOverlay"],
};
pub static SHEET_META: ComponentMeta = ComponentMeta {
    id: "sheet",         name: "Sheet", family: ComponentFamily::Overlay,
    intent: "Side panel for forms or navigation",
    capabilities: &[Capability::OpenClose, Capability::FocusTrap, Capability::KeyboardEsc],
    composable: true,
    required_parts: &["SheetContent"],
    optional_parts: &["SheetOverlay"],
};
pub static MODAL_META: ComponentMeta = ComponentMeta {
    id: "modal",         name: "Modal", family: ComponentFamily::Overlay,
    intent: "Generic modal container",
    capabilities: &[Capability::OpenClose, Capability::FocusTrap, Capability::AriaModal],
    composable: true,
    required_parts: &[],
    optional_parts: &[],
};
pub static HOVER_CARD_META: ComponentMeta = ComponentMeta {
    id: "hover-card",    name: "HoverCard", family: ComponentFamily::Overlay,
    intent: "Show rich preview on hover",
    capabilities: &[Capability::OpenClose],
    composable: true,
    required_parts: &["HoverCardTrigger", "HoverCardContent"],
    optional_parts: &[],
};
pub static POPOVER_META: ComponentMeta = ComponentMeta {
    id: "popover",       name: "Popover", family: ComponentFamily::Overlay,
    intent: "Show contextual floating content",
    capabilities: &[Capability::OpenClose, Capability::FocusTrap],
    composable: true,
    required_parts: &["PopoverTrigger", "PopoverContent"],
    optional_parts: &[],
};
pub static TOOLTIP_META: ComponentMeta = ComponentMeta {
    id: "tooltip",       name: "Tooltip", family: ComponentFamily::Overlay,
    intent: "Show brief label on hover/focus",
    capabilities: &[Capability::OpenClose],
    composable: true,
    required_parts: &["TooltipTrigger", "TooltipContent"],
    optional_parts: &["TooltipProvider"],
};

// ── INPUT ────────────────────────────────────────────────────────────────────
pub static BUTTON_META: ComponentMeta = ComponentMeta {
    id: "button",        name: "Button", family: ComponentFamily::Interactive,
    intent: "Trigger an action or event",
    capabilities: &[Capability::Disabled],
    composable: false, required_parts: &[], optional_parts: &[],
};
pub static INPUT_META: ComponentMeta = ComponentMeta {
    id: "input",         name: "Input", family: ComponentFamily::Input,
    intent: "Capture text or data from user",
    capabilities: &[Capability::Value, Capability::Disabled],
    composable: false, required_parts: &[], optional_parts: &[],
};
pub static TEXTAREA_META: ComponentMeta = ComponentMeta {
    id: "textarea",      name: "Textarea", family: ComponentFamily::Input,
    intent: "Capture multi-line text from user",
    capabilities: &[Capability::Value, Capability::Disabled],
    composable: false, required_parts: &[], optional_parts: &[],
};
pub static SELECT_META: ComponentMeta = ComponentMeta {
    id: "select",        name: "Select", family: ComponentFamily::Input,
    intent: "Choose one option from a list",
    capabilities: &[Capability::OpenClose, Capability::Disabled],
    composable: true,
    required_parts: &["SelectTrigger", "SelectContent", "SelectItem"],
    optional_parts: &["SelectValue", "SelectSeparator"],
};
pub static CHECKBOX_META: ComponentMeta = ComponentMeta {
    id: "checkbox",      name: "Checkbox", family: ComponentFamily::Input,
    intent: "Toggle a boolean value",
    capabilities: &[Capability::Disabled],
    composable: false, required_parts: &[], optional_parts: &[],
};
pub static RADIO_META: ComponentMeta = ComponentMeta {
    id: "radio",         name: "Radio", family: ComponentFamily::Input,
    intent: "Select one option from a group",
    capabilities: &[Capability::Disabled],
    composable: true,
    required_parts: &["RadioGroup"],
    optional_parts: &[],
};
pub static SWITCH_META: ComponentMeta = ComponentMeta {
    id: "switch",        name: "Switch", family: ComponentFamily::Input,
    intent: "Toggle between on and off states",
    capabilities: &[Capability::Disabled],
    composable: false, required_parts: &[], optional_parts: &[],
};
pub static SLIDER_META: ComponentMeta = ComponentMeta {
    id: "slider",        name: "Slider", family: ComponentFamily::Input,
    intent: "Select a value within a range",
    capabilities: &[Capability::Value, Capability::Disabled],
    composable: true,
    required_parts: &["SliderTrack", "SliderThumb"],
    optional_parts: &["SliderRange"],
};
pub static TOGGLE_META: ComponentMeta = ComponentMeta {
    id: "toggle",        name: "Toggle", family: ComponentFamily::Input,
    intent: "Toggle a pressed state",
    capabilities: &[Capability::Pressed, Capability::Disabled],
    composable: false, required_parts: &[], optional_parts: &[],
};
pub static COLOR_PICKER_META: ComponentMeta = ComponentMeta {
    id: "color-picker",  name: "ColorPicker", family: ComponentFamily::Input,
    intent: "Select a color value",
    capabilities: &[Capability::Value, Capability::Disabled],
    composable: true, required_parts: &[], optional_parts: &[],
};
pub static COMBOBOX_META: ComponentMeta = ComponentMeta {
    id: "combobox",      name: "Combobox", family: ComponentFamily::Input,
    intent: "Search and select from a list",
    capabilities: &[Capability::OpenClose, Capability::Disabled],
    composable: true,
    required_parts: &["SelectTrigger", "SelectContent"],
    optional_parts: &[],
};

// ── FEEDBACK ─────────────────────────────────────────────────────────────────
pub static ALERT_META: ComponentMeta = ComponentMeta {
    id: "alert",         name: "Alert", family: ComponentFamily::Feedback,
    intent: "Display important static messages",
    capabilities: &[],
    composable: true, required_parts: &[],
    optional_parts: &["AlertTitle", "AlertDescription"],
};
pub static TOAST_META: ComponentMeta = ComponentMeta {
    id: "toast",         name: "Toast", family: ComponentFamily::Feedback,
    intent: "Show brief non-blocking notifications",
    capabilities: &[Capability::OpenClose],
    composable: true,
    required_parts: &["ToastViewport"],
    optional_parts: &["ToastTitle", "ToastDescription", "ToastAction", "ToastClose"],
};
pub static BANNER_META: ComponentMeta = ComponentMeta {
    id: "banner",        name: "Banner", family: ComponentFamily::Feedback,
    intent: "Display persistent page-level messages",
    capabilities: &[Capability::OpenClose],
    composable: true, required_parts: &[],
    optional_parts: &["BannerContent", "BannerActions", "BannerClose"],
};
pub static CALLOUT_META: ComponentMeta = ComponentMeta {
    id: "callout",       name: "Callout", family: ComponentFamily::Feedback,
    intent: "Highlight important contextual information",
    capabilities: &[],
    composable: true, required_parts: &[],
    optional_parts: &["CalloutIcon", "CalloutTitle", "CalloutDescription"],
};
pub static INLINE_NOTICE_META: ComponentMeta = ComponentMeta {
    id: "inline-notice", name: "InlineNotice", family: ComponentFamily::Feedback,
    intent: "Show inline contextual feedback",
    capabilities: &[],
    composable: true, required_parts: &[],
    optional_parts: &["InlineNoticeIcon", "InlineNoticeContent"],
};
pub static PROGRESS_META: ComponentMeta = ComponentMeta {
    id: "progress",      name: "Progress", family: ComponentFamily::Feedback,
    intent: "Show completion of a task",
    capabilities: &[Capability::Value],
    composable: true,
    required_parts: &["ProgressIndicator"],
    optional_parts: &[],
};
pub static SPINNER_META: ComponentMeta = ComponentMeta {
    id: "spinner",       name: "Spinner", family: ComponentFamily::Feedback,
    intent: "Indicate loading or processing",
    capabilities: &[],
    composable: false, required_parts: &[], optional_parts: &[],
};
pub static SKELETON_META: ComponentMeta = ComponentMeta {
    id: "skeleton",      name: "Skeleton", family: ComponentFamily::Feedback,
    intent: "Show placeholder while content loads",
    capabilities: &[],
    composable: false, required_parts: &[], optional_parts: &[],
};
pub static PULSE_META: ComponentMeta = ComponentMeta {
    id: "pulse",         name: "Pulse", family: ComponentFamily::Feedback,
    intent: "Animated attention indicator",
    capabilities: &[],
    composable: false, required_parts: &[], optional_parts: &[],
};
pub static EMPTY_STATE_META: ComponentMeta = ComponentMeta {
    id: "empty-state",   name: "EmptyState", family: ComponentFamily::Feedback,
    intent: "Display when no content is available",
    capabilities: &[],
    composable: true, required_parts: &[],
    optional_parts: &[],
};
pub static ERROR_STATE_META: ComponentMeta = ComponentMeta {
    id: "error-state",   name: "ErrorState", family: ComponentFamily::Feedback,
    intent: "Display error condition to user",
    capabilities: &[],
    composable: true, required_parts: &[],
    optional_parts: &[],
};

// ── NAVIGATION ───────────────────────────────────────────────────────────────
pub static TABS_META: ComponentMeta = ComponentMeta {
    id: "tabs",          name: "Tabs", family: ComponentFamily::Navigation,
    intent: "Switch between related content panels",
    capabilities: &[Capability::Active],
    composable: true,
    required_parts: &["TabsList", "TabsTrigger", "TabsContent"],
    optional_parts: &[],
};
pub static BREADCRUMB_META: ComponentMeta = ComponentMeta {
    id: "breadcrumb",    name: "Breadcrumb", family: ComponentFamily::Navigation,
    intent: "Show current location in hierarchy",
    capabilities: &[],
    composable: true, required_parts: &[],
    optional_parts: &["BreadcrumbItem", "BreadcrumbLink", "BreadcrumbSeparator"],
};
pub static PAGINATION_META: ComponentMeta = ComponentMeta {
    id: "pagination",    name: "Pagination", family: ComponentFamily::Navigation,
    intent: "Navigate between pages of content",
    capabilities: &[Capability::Active, Capability::Disabled],
    composable: true, required_parts: &[],
    optional_parts: &["PaginationContent", "PaginationItem", "PaginationLink"],
};
pub static NAVIGATION_MENU_META: ComponentMeta = ComponentMeta {
    id: "navigation-menu", name: "NavigationMenu", family: ComponentFamily::Navigation,
    intent: "Primary site navigation with submenus",
    capabilities: &[Capability::OpenClose],
    composable: true,
    required_parts: &["NavigationMenuList", "NavigationMenuItem"],
    optional_parts: &["NavigationMenuTrigger", "NavigationMenuContent", "NavigationMenuLink"],
};
pub static SIDEBAR_META: ComponentMeta = ComponentMeta {
    id: "sidebar",       name: "Sidebar", family: ComponentFamily::Navigation,
    intent: "Vertical navigation panel",
    capabilities: &[Capability::OpenClose],
    composable: true,
    required_parts: &["SidebarContent"],
    optional_parts: &["SidebarHeader", "SidebarFooter", "SidebarMenu", "SidebarMenuItem"],
};
pub static NAV_ITEM_META: ComponentMeta = ComponentMeta {
    id: "nav-item",      name: "NavItem", family: ComponentFamily::Navigation,
    intent: "Single navigation link item",
    capabilities: &[Capability::Active, Capability::Disabled],
    composable: false, required_parts: &[], optional_parts: &[],
};

// ── LAYOUT ───────────────────────────────────────────────────────────────────
pub static CARD_META: ComponentMeta = ComponentMeta {
    id: "card",          name: "Card", family: ComponentFamily::Layout,
    intent: "Group related content in a container",
    capabilities: &[],
    composable: true, required_parts: &[],
    optional_parts: &["CardHeader", "CardTitle", "CardDescription", "CardContent", "CardFooter"],
};
pub static SEPARATOR_META: ComponentMeta = ComponentMeta {
    id: "separator",     name: "Separator", family: ComponentFamily::Layout,
    intent: "Visually divide content sections",
    capabilities: &[Capability::Orientation],
    composable: false, required_parts: &[], optional_parts: &[],
};
pub static RESIZABLE_META: ComponentMeta = ComponentMeta {
    id: "resizable",     name: "Resizable", family: ComponentFamily::Layout,
    intent: "Split panels with draggable divider",
    capabilities: &[Capability::Orientation, Capability::Resize],
    composable: true,
    required_parts: &["ResizablePanel", "ResizableHandle"],
    optional_parts: &[],
};
pub static SCROLL_AREA_META: ComponentMeta = ComponentMeta {
    id: "scroll-area",   name: "ScrollArea", family: ComponentFamily::Layout,
    intent: "Scrollable container with custom scrollbar",
    capabilities: &[],
    composable: false, required_parts: &[], optional_parts: &[],
};
pub static ASPECT_RATIO_META: ComponentMeta = ComponentMeta {
    id: "aspect-ratio",  name: "AspectRatio", family: ComponentFamily::Layout,
    intent: "Maintain consistent width/height ratio",
    capabilities: &[],
    composable: false, required_parts: &[], optional_parts: &[],
};
pub static COLLAPSIBLE_META: ComponentMeta = ComponentMeta {
    id: "collapsible",   name: "Collapsible", family: ComponentFamily::Layout,
    intent: "Show and hide content sections",
    capabilities: &[Capability::OpenClose],
    composable: true,
    required_parts: &["CollapsibleTrigger", "CollapsibleContent"],
    optional_parts: &[],
};
pub static ACCORDION_META: ComponentMeta = ComponentMeta {
    id: "accordion",     name: "Accordion", family: ComponentFamily::Layout,
    intent: "Expand and collapse content sections",
    capabilities: &[Capability::OpenClose, Capability::Multiple],
    composable: true,
    required_parts: &["AccordionItem", "AccordionTrigger", "AccordionContent"],
    optional_parts: &[],
};

// ── DATA DISPLAY ─────────────────────────────────────────────────────────────
pub static BADGE_META: ComponentMeta = ComponentMeta {
    id: "badge",         name: "Badge", family: ComponentFamily::DataDisplay,
    intent: "Display status, count or label",
    capabilities: &[],
    composable: false, required_parts: &[], optional_parts: &[],
};
pub static TABLE_META: ComponentMeta = ComponentMeta {
    id: "table",         name: "Table", family: ComponentFamily::DataDisplay,
    intent: "Display tabular data",
    capabilities: &[Capability::Selected],
    composable: true,
    required_parts: &["TableHeader", "TableBody", "TableRow", "TableHead", "TableCell"],
    optional_parts: &["TableFooter", "TableCaption", "TableWrapper"],
};
pub static DATA_TABLE_META: ComponentMeta = ComponentMeta {
    id: "data-table",    name: "DataTable", family: ComponentFamily::DataDisplay,
    intent: "Display sortable, filterable tabular data",
    capabilities: &[Capability::Multiple],
    composable: true, required_parts: &[], optional_parts: &[],
};
pub static STAT_META: ComponentMeta = ComponentMeta {
    id: "stat",          name: "Stat", family: ComponentFamily::DataDisplay,
    intent: "Display a key metric with label",
    capabilities: &[],
    composable: true, required_parts: &["StatValue", "StatLabel"],
    optional_parts: &["StatDelta", "StatIcon"],
};
pub static STATUS_DOT_META: ComponentMeta = ComponentMeta {
    id: "status-dot",    name: "StatusDot", family: ComponentFamily::DataDisplay,
    intent: "Indicate user presence or availability",
    capabilities: &[],
    composable: false, required_parts: &[], optional_parts: &[],
};
pub static AVATAR_META: ComponentMeta = ComponentMeta {
    id: "avatar",        name: "Avatar", family: ComponentFamily::DataDisplay,
    intent: "Display user profile image with fallback",
    capabilities: &[],
    composable: true,
    required_parts: &[],
    optional_parts: &["AvatarImage", "AvatarFallback"],
};
pub static CAROUSEL_META: ComponentMeta = ComponentMeta {
    id: "carousel",      name: "Carousel", family: ComponentFamily::DataDisplay,
    intent: "Cycle through items horizontally",
    capabilities: &[],
    composable: true,
    required_parts: &["CarouselTrack", "CarouselItem"],
    optional_parts: &["CarouselPrev", "CarouselNext", "CarouselIndicators"],
};
pub static CHART_META: ComponentMeta = ComponentMeta {
    id: "chart",         name: "Chart", family: ComponentFamily::DataDisplay,
    intent: "Visualize data graphically",
    capabilities: &[],
    composable: false, required_parts: &[], optional_parts: &[],
};
pub static TREE_META: ComponentMeta = ComponentMeta {
    id: "tree",          name: "Tree", family: ComponentFamily::DataDisplay,
    intent: "Display hierarchical data",
    capabilities: &[Capability::Selected],
    composable: true,
    required_parts: &["TreeItem"],
    optional_parts: &["TreeGroup"],
};
pub static VIRTUAL_LIST_META: ComponentMeta = ComponentMeta {
    id: "virtual-list",  name: "VirtualList", family: ComponentFamily::DataDisplay,
    intent: "Efficiently render large lists",
    capabilities: &[Capability::VirtualScroll],
    composable: false, required_parts: &[], optional_parts: &[],
};

// ── TYPOGRAPHY ───────────────────────────────────────────────────────────────
pub static LINK_META: ComponentMeta = ComponentMeta {
    id: "link",          name: "Link", family: ComponentFamily::Typography,
    intent: "Navigate to a URL or trigger action",
    capabilities: &[Capability::Disabled],
    composable: false, required_parts: &[], optional_parts: &[],
};
pub static KBD_META: ComponentMeta = ComponentMeta {
    id: "kbd",           name: "Kbd", family: ComponentFamily::Typography,
    intent: "Display keyboard shortcut",
    capabilities: &[],
    composable: false, required_parts: &[], optional_parts: &[],
};
pub static MARKDOWN_META: ComponentMeta = ComponentMeta {
    id: "markdown",      name: "Markdown", family: ComponentFamily::Typography,
    intent: "Render markdown content",
    capabilities: &[],
    composable: false, required_parts: &[], optional_parts: &[],
};

// ── INTERACTIVE ──────────────────────────────────────────────────────────────
pub static DROPDOWN_MENU_META: ComponentMeta = ComponentMeta {
    id: "dropdown-menu", name: "DropdownMenu", family: ComponentFamily::Interactive,
    intent: "Show contextual action menu",
    capabilities: &[Capability::OpenClose, Capability::Disabled],
    composable: true,
    required_parts: &["DropdownMenuTrigger", "DropdownMenuContent"],
    optional_parts: &["DropdownMenuItem", "DropdownMenuSeparator", "DropdownMenuGroup"],
};
pub static CONTEXT_MENU_META: ComponentMeta = ComponentMeta {
    id: "context-menu",  name: "ContextMenu", family: ComponentFamily::Interactive,
    intent: "Show menu on right-click",
    capabilities: &[Capability::OpenClose],
    composable: true,
    required_parts: &["ContextMenuTrigger", "ContextMenuContent"],
    optional_parts: &["ContextMenuItem", "ContextMenuSeparator"],
};
pub static MENUBAR_META: ComponentMeta = ComponentMeta {
    id: "menubar",       name: "Menubar", family: ComponentFamily::Interactive,
    intent: "Horizontal application menu bar",
    capabilities: &[Capability::OpenClose],
    composable: true,
    required_parts: &["MenubarMenu", "MenubarTrigger"],
    optional_parts: &["MenubarContent", "MenubarItem", "MenubarSeparator"],
};
pub static COMMAND_META: ComponentMeta = ComponentMeta {
    id: "command",       name: "Command", family: ComponentFamily::Interactive,
    intent: "Command palette for quick actions",
    capabilities: &[],
    composable: true,
    required_parts: &["CommandInput", "CommandList"],
    optional_parts: &["CommandItem", "CommandGroup", "CommandSeparator", "CommandEmpty"],
};
pub static TOOLBAR_META: ComponentMeta = ComponentMeta {
    id: "toolbar",       name: "Toolbar", family: ComponentFamily::Interactive,
    intent: "Group of action buttons or controls",
    capabilities: &[Capability::Orientation],
    composable: true, required_parts: &[],
    optional_parts: &["ToolbarSeparator"],
};
pub static TOGGLE_GROUP_META: ComponentMeta = ComponentMeta {
    id: "toggle-group",  name: "ToggleGroup", family: ComponentFamily::Interactive,
    intent: "Group of toggle buttons with single or multiple selection",
    capabilities: &[Capability::Multiple, Capability::Disabled],
    composable: true, required_parts: &[], optional_parts: &[],
};

// ── UTILITY ──────────────────────────────────────────────────────────────────
pub static ANIMATE_META: ComponentMeta = ComponentMeta {
    id: "animate",       name: "Animate", family: ComponentFamily::Utility,
    intent: "Apply CSS animations to children",
    capabilities: &[],
    composable: false, required_parts: &[], optional_parts: &[],
};
pub static LOADING_OVERLAY_META: ComponentMeta = ComponentMeta {
    id: "loading-overlay", name: "LoadingOverlay", family: ComponentFamily::Utility,
    intent: "Block UI during async operations",
    capabilities: &[Capability::OpenClose],
    composable: false, required_parts: &[], optional_parts: &[],
};
pub static ICON_META: ComponentMeta = ComponentMeta {
    id: "icon",          name: "Icon", family: ComponentFamily::Utility,
    intent: "Display an SVG icon",
    capabilities: &[],
    composable: false, required_parts: &[], optional_parts: &[],
};


/// Fallback meta para blocos estruturais sem ComponentMeta específico.
pub static GRID_META: ComponentMeta = ComponentMeta {
    id:             "grid",
    name:           "Grid",
    family:         ComponentFamily::Layout,
    intent:         "Layout with multiple columns using CSS grid",
    capabilities:   &[Capability::Multiple],
    composable:     true,
    required_parts: &[],
    optional_parts: &[],
};

pub static STACK_META: ComponentMeta = ComponentMeta {
    id:             "stack",
    name:           "Stack",
    family:         ComponentFamily::Layout,
    intent:         "Vertical or horizontal flex container",
    capabilities:   &[Capability::Orientation],
    composable:     true,
    required_parts: &[],
    optional_parts: &[],
};

pub static CONTAINER_META: ComponentMeta = ComponentMeta {
    id:             "container",
    name:           "Container",
    family:         ComponentFamily::Layout,
    intent:         "Max-width centered layout wrapper",
    capabilities:   &[],
    composable:     true,
    required_parts: &[],
    optional_parts: &[],
};

pub static COLUMNS_META: ComponentMeta = ComponentMeta {
    id:             "columns",
    name:           "Columns",
    family:         ComponentFamily::Layout,
    intent:         "Fixed column layout with configurable count",
    capabilities:   &[Capability::Multiple],
    composable:     true,
    required_parts: &[],
    optional_parts: &[],
};

pub static SIDEBAR_LAYOUT_META: ComponentMeta = ComponentMeta {
    id: "sidebar-layout", name: "SidebarLayout", family: ComponentFamily::Layout,
    intent: "Layout with fixed sidebar and main content", capabilities: &[],
    composable: true, required_parts: &[], optional_parts: &[],
};
pub static SPLIT_META: ComponentMeta = ComponentMeta {
    id: "split", name: "Split", family: ComponentFamily::Layout,
    intent: "Two-panel split layout", capabilities: &[],
    composable: true, required_parts: &[], optional_parts: &[],
};
pub static DETAIL_PANEL_META: ComponentMeta = ComponentMeta {
    id: "detail-panel", name: "DetailPanel", family: ComponentFamily::Layout,
    intent: "Master-detail panel layout", capabilities: &[],
    composable: true, required_parts: &[], optional_parts: &[],
};
pub static FOOTER_META: ComponentMeta = ComponentMeta {
    id: "footer", name: "Footer", family: ComponentFamily::Layout,
    intent: "Page footer region", capabilities: &[],
    composable: true, required_parts: &[], optional_parts: &[],
};
pub static HEADER_META: ComponentMeta = ComponentMeta {
    id: "header", name: "Header", family: ComponentFamily::Layout,
    intent: "Page header region", capabilities: &[],
    composable: true, required_parts: &[], optional_parts: &[],
};
pub static PAGE_HEADER_META: ComponentMeta = ComponentMeta {
    id: "page-header", name: "PageHeader", family: ComponentFamily::Layout,
    intent: "Page title and actions bar", capabilities: &[],
    composable: false, required_parts: &[], optional_parts: &[],
};
pub static TOOLBAR_BLOCK_META: ComponentMeta = ComponentMeta {
    id: "toolbar", name: "Toolbar", family: ComponentFamily::Layout,
    intent: "Action toolbar region", capabilities: &[],
    composable: true, required_parts: &[], optional_parts: &[],
};
pub static STAT_CARD_META: ComponentMeta = ComponentMeta {
    id: "stat-card", name: "StatCard", family: ComponentFamily::DataDisplay,
    intent: "KPI metric card", capabilities: &[],
    composable: false, required_parts: &[], optional_parts: &[],
};
pub static LIST_META: ComponentMeta = ComponentMeta {
    id: "list", name: "List", family: ComponentFamily::DataDisplay,
    intent: "Ordered or unordered list container", capabilities: &[],
    composable: true, required_parts: &[], optional_parts: &[],
};
pub static TIMELINE_META: ComponentMeta = ComponentMeta {
    id: "timeline", name: "Timeline", family: ComponentFamily::DataDisplay,
    intent: "Chronological event timeline", capabilities: &[],
    composable: true, required_parts: &[], optional_parts: &[],
};
pub static FILTER_BAR_META: ComponentMeta = ComponentMeta {
    id: "filter-bar", name: "FilterBar", family: ComponentFamily::Interactive,
    intent: "Search and filter controls bar", capabilities: &[],
    composable: true, required_parts: &[], optional_parts: &[],
};
pub static FORM_META: ComponentMeta = ComponentMeta {
    id: "form", name: "Form", family: ComponentFamily::Interactive,
    intent: "Form container with submit handling", capabilities: &[],
    composable: true, required_parts: &[], optional_parts: &[],
};
pub static WIZARD_META: ComponentMeta = ComponentMeta {
    id: "wizard", name: "Wizard", family: ComponentFamily::Interactive,
    intent: "Multi-step wizard flow", capabilities: &[],
    composable: true, required_parts: &[], optional_parts: &[],
};
pub static CODE_BLOCK_META: ComponentMeta = ComponentMeta {
    id: "code-block", name: "CodeBlock", family: ComponentFamily::DataDisplay,
    intent: "Syntax-highlighted code display", capabilities: &[],
    composable: false, required_parts: &[], optional_parts: &[],
};
pub static MARKDOWN_BLOCK_META: ComponentMeta = ComponentMeta {
    id: "markdown", name: "Markdown", family: ComponentFamily::DataDisplay,
    intent: "Rendered markdown content", capabilities: &[],
    composable: false, required_parts: &[], optional_parts: &[],
};
pub static COMMAND_PANEL_META: ComponentMeta = ComponentMeta {
    id: "command-panel", name: "CommandPanel", family: ComponentFamily::Interactive,
    intent: "Command palette overlay", capabilities: &[Capability::OpenClose],
    composable: false, required_parts: &[], optional_parts: &[],
};
pub static SECTION_META: ComponentMeta = ComponentMeta {
    id: "section", name: "Section", family: ComponentFamily::Layout,
    intent: "Page section wrapper", capabilities: &[],
    composable: true, required_parts: &[], optional_parts: &[],
};
pub static LAYOUT_BLOCK_META: ComponentMeta = ComponentMeta {
    id: "layout", name: "Layout", family: ComponentFamily::Layout,
    intent: "Root page layout shell", capabilities: &[],
    composable: true, required_parts: &[], optional_parts: &[],
};
pub static DASHBOARD_LAYOUT_META: ComponentMeta = ComponentMeta {
    id: "dashboard", name: "Dashboard", family: ComponentFamily::Layout,
    intent: "App shell with header, sidebar and main", capabilities: &[],
    composable: true, required_parts: &[], optional_parts: &[],
};
pub static MARKETING_LAYOUT_META: ComponentMeta = ComponentMeta {
    id: "marketing", name: "Marketing", family: ComponentFamily::Layout,
    intent: "Marketing page layout with hero", capabilities: &[],
    composable: true, required_parts: &[], optional_parts: &[],
};
pub static FULLSCREEN_LAYOUT_META: ComponentMeta = ComponentMeta {
    id: "fullscreen", name: "Fullscreen", family: ComponentFamily::Layout,
    intent: "Full viewport layout", capabilities: &[],
    composable: true, required_parts: &[], optional_parts: &[],
};
pub static SPLIT_VIEW_LAYOUT_META: ComponentMeta = ComponentMeta {
    id: "split-view", name: "SplitView", family: ComponentFamily::Layout,
    intent: "Side-by-side split view layout", capabilities: &[],
    composable: true, required_parts: &[], optional_parts: &[],
};
pub static PAGE_SINGLE_META: ComponentMeta = ComponentMeta {
    id: "page-single", name: "PageSingle", family: ComponentFamily::Layout,
    intent: "Single column page layout", capabilities: &[],
    composable: true, required_parts: &[], optional_parts: &[],
};
pub static PAGE_WITH_SIDEBAR_META: ComponentMeta = ComponentMeta {
    id: "page-with-sidebar", name: "PageWithSidebar", family: ComponentFamily::Layout,
    intent: "Page layout with left sidebar", capabilities: &[],
    composable: true, required_parts: &[], optional_parts: &[],
};
pub static PAGE_WITH_ASIDE_META: ComponentMeta = ComponentMeta {
    id: "page-with-aside", name: "PageWithAside", family: ComponentFamily::Layout,
    intent: "Page layout with right aside", capabilities: &[],
    composable: true, required_parts: &[], optional_parts: &[],
};
pub static PAGE_SIDEBAR_AND_ASIDE_META: ComponentMeta = ComponentMeta {
    id: "page-sidebar-and-aside", name: "PageSidebarAndAside", family: ComponentFamily::Layout,
    intent: "Page layout with sidebar and aside", capabilities: &[],
    composable: true, required_parts: &[], optional_parts: &[],
};

pub static WIZARD_LAYOUT_META: ComponentMeta = ComponentMeta {
    id: "wizard-layout", name: "WizardLayout", family: ComponentFamily::Layout,
    intent: "Multi-step wizard page layout", capabilities: &[],
    composable: true, required_parts: &[], optional_parts: &[],
};

#[deprecated(note = "EMPTY_META is a placeholder — every block must have a real ComponentMeta")]
pub static EMPTY_META: ComponentMeta = ComponentMeta {
    id:             "empty",
    name:           "Empty",
    family:         ComponentFamily::Layout,
    intent:         "structural placeholder",
    capabilities:   &[],
    composable:     false,
    required_parts: &[],
    optional_parts: &[],
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
        &GRID_META, &STACK_META, &CONTAINER_META, &COLUMNS_META,
        &SIDEBAR_LAYOUT_META, &SPLIT_META, &DETAIL_PANEL_META,
        &FOOTER_META, &HEADER_META, &PAGE_HEADER_META, &TOOLBAR_BLOCK_META,
        &STAT_CARD_META, &LIST_META, &TIMELINE_META, &FILTER_BAR_META,
        &FORM_META, &WIZARD_META, &CODE_BLOCK_META, &MARKDOWN_BLOCK_META,
        &COMMAND_PANEL_META, &SECTION_META, &LAYOUT_BLOCK_META,
        &DASHBOARD_LAYOUT_META, &MARKETING_LAYOUT_META, &FULLSCREEN_LAYOUT_META,
        &SPLIT_VIEW_LAYOUT_META, &PAGE_SINGLE_META, &PAGE_WITH_SIDEBAR_META,
        &PAGE_WITH_ASIDE_META, &PAGE_SIDEBAR_AND_ASIDE_META, &WIZARD_LAYOUT_META,
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

// ── CAPABILITY → PROPS MAPPING ───────────────────────────────────────────────

/// Prop types that can be derived from capabilities declared in ComponentMeta.
/// These are the minimal props implied by a capability — not visual/structural props.
#[derive(Debug, Clone, PartialEq)]
pub struct CapabilityPropDef {
    pub key:     &'static str,
    pub label:   &'static str,
    pub kind:    CapabilityPropKind,
    pub default: Option<&'static str>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CapabilityPropKind {
    Bool,
    String,
    Enum(&'static [&'static str]),
}

/// Maps a single capability string to the props it implies.
/// Pure behaviors (focus-trap, keyboard-esc, aria-modal) → no props.
/// State-carrying capabilities (open-close, selected, active) → typed props.
/// Type-safe capability → props. Match direto no enum, sem strings.
pub fn capability_to_props_enum(cap: Capability) -> &'static [CapabilityPropDef] {
    match cap {
        Capability::OpenClose => &[
            CapabilityPropDef { key: "open",         label: "Open",         kind: CapabilityPropKind::Bool, default: Some("false") },
            CapabilityPropDef { key: "default-open", label: "Default Open", kind: CapabilityPropKind::Bool, default: Some("false") },
        ],
        Capability::Selected => &[
            CapabilityPropDef { key: "selected", label: "Selected", kind: CapabilityPropKind::Bool, default: Some("false") },
        ],
        Capability::Active => &[
            CapabilityPropDef { key: "active", label: "Active", kind: CapabilityPropKind::Bool, default: Some("false") },
        ],
        Capability::Pressed => &[
            CapabilityPropDef { key: "pressed", label: "Pressed", kind: CapabilityPropKind::Bool, default: Some("false") },
        ],
        Capability::Disabled => &[
            CapabilityPropDef { key: "disabled", label: "Disabled", kind: CapabilityPropKind::Bool, default: Some("false") },
        ],
        Capability::Loading => &[
            CapabilityPropDef { key: "loading", label: "Loading", kind: CapabilityPropKind::Bool, default: Some("false") },
        ],
        Capability::Value => &[
            CapabilityPropDef { key: "value", label: "Value", kind: CapabilityPropKind::String, default: None },
        ],
        Capability::Multiple => &[
            CapabilityPropDef { key: "multiple", label: "Multiple", kind: CapabilityPropKind::Bool, default: Some("false") },
        ],
        Capability::Orientation => &[
            CapabilityPropDef { key: "orientation", label: "Orientation", kind: CapabilityPropKind::Enum(&["horizontal", "vertical"]), default: Some("horizontal") },
        ],
        // Pure behaviors — no props derived
        Capability::FocusTrap | Capability::KeyboardEsc | Capability::AriaModal |
        Capability::KeyboardArrows | Capability::Roving | Capability::Typeahead |
        Capability::VirtualScroll | Capability::DragDrop | Capability::Resize |
        Capability::Overflow | Capability::Animation => &[],
    }
}

/// Deriva props a partir de slice de Capability enums (type-safe, sem strings).
pub fn derive_props_from_capabilities(caps: &[Capability]) -> Vec<CapabilityPropDef> {
    let mut seen = std::collections::HashSet::new();
    let mut result = Vec::new();
    for cap in caps {
        for prop in capability_to_props_enum(*cap) {
            if seen.insert(prop.key) {
                result.push(prop.clone());
            }
        }
    }
    result
}

/// Derives all implied props from a ComponentMeta's capabilities.
/// Deduplicates by key — first capability wins if two imply the same prop.
pub fn derive_props_from_meta(meta: &ComponentMeta) -> Vec<CapabilityPropDef> {
    derive_props_from_capabilities(meta.capabilities)
}

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum LoadingState {
    #[default]
    Idle,
    Loading,
}

impl LoadingState {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Idle    => "idle",
            Self::Loading => "loading",
        }
    }
}
