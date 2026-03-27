// AUTO-GENERATED — do not edit. Edit components.toml instead.
#![allow(dead_code)]
use crate::meta_types::{ComponentMeta, ComponentFamily, Capability};

pub static ACCORDION_META: ComponentMeta = ComponentMeta {
    id: "accordion",
    name: "Accordion",
    family: ComponentFamily::Layout,
    intent: "Expand and collapse content sections",
    capabilities: &[Capability::OpenClose, Capability::Multiple],
    composable: true,
    required_parts: &["AccordionItem", "AccordionTrigger", "AccordionContent"],
    optional_parts: &[],
};

pub static ALERT_META: ComponentMeta = ComponentMeta {
    id: "alert",
    name: "Alert",
    family: ComponentFamily::Feedback,
    intent: "Display important static messages",
    capabilities: &[],
    composable: true,
    required_parts: &[],
    optional_parts: &["AlertTitle", "AlertDescription"],
};

pub static ALERT_DIALOG_META: ComponentMeta = ComponentMeta {
    id: "alert-dialog",
    name: "Alert Dialog",
    family: ComponentFamily::Overlay,
    intent: "Confirm destructive actions with user",
    capabilities: &[Capability::OpenClose, Capability::FocusTrap, Capability::AriaModal],
    composable: true,
    required_parts: &["AlertDialogContent", "AlertDialogTitle"],
    optional_parts: &["AlertDialogOverlay", "AlertDialogDescription", "AlertDialogAction"],
};

pub static ANIMATE_META: ComponentMeta = ComponentMeta {
    id: "animate",
    name: "Animate",
    family: ComponentFamily::Utility,
    intent: "Apply CSS animations to children",
    capabilities: &[],
    composable: false,
    required_parts: &[],
    optional_parts: &[],
};

pub static ASPECT_RATIO_META: ComponentMeta = ComponentMeta {
    id: "aspect-ratio",
    name: "Aspect Ratio",
    family: ComponentFamily::Layout,
    intent: "Maintain consistent width/height ratio",
    capabilities: &[],
    composable: false,
    required_parts: &[],
    optional_parts: &[],
};

pub static AVATAR_META: ComponentMeta = ComponentMeta {
    id: "avatar",
    name: "Avatar",
    family: ComponentFamily::DataDisplay,
    intent: "Display user profile image with fallback",
    capabilities: &[],
    composable: true,
    required_parts: &[],
    optional_parts: &["AvatarImage", "AvatarFallback"],
};

pub static BADGE_META: ComponentMeta = ComponentMeta {
    id: "badge",
    name: "Badge",
    family: ComponentFamily::DataDisplay,
    intent: "Display status, count or label",
    capabilities: &[],
    composable: false,
    required_parts: &[],
    optional_parts: &[],
};

pub static BANNER_META: ComponentMeta = ComponentMeta {
    id: "banner",
    name: "Banner",
    family: ComponentFamily::Feedback,
    intent: "Display persistent page-level messages",
    capabilities: &[Capability::OpenClose],
    composable: true,
    required_parts: &[],
    optional_parts: &["BannerContent", "BannerActions", "BannerClose"],
};

pub static BREADCRUMB_META: ComponentMeta = ComponentMeta {
    id: "breadcrumb",
    name: "Breadcrumb",
    family: ComponentFamily::Navigation,
    intent: "Show current location in hierarchy",
    capabilities: &[],
    composable: true,
    required_parts: &[],
    optional_parts: &["BreadcrumbItem", "BreadcrumbLink", "BreadcrumbSeparator"],
};

pub static BUTTON_META: ComponentMeta = ComponentMeta {
    id: "button",
    name: "Button",
    family: ComponentFamily::Interactive,
    intent: "Trigger an action or event",
    capabilities: &[Capability::Disabled],
    composable: false,
    required_parts: &[],
    optional_parts: &[],
};

pub static BUTTON_GROUP_META: ComponentMeta = ComponentMeta {
    id: "button-group",
    name: "Button Group",
    family: ComponentFamily::Utility,
    intent: "Group related action buttons together",
    capabilities: &[],
    composable: false,
    required_parts: &[],
    optional_parts: &[],
};

pub static CALLOUT_META: ComponentMeta = ComponentMeta {
    id: "callout",
    name: "Callout",
    family: ComponentFamily::Feedback,
    intent: "Highlight important contextual information",
    capabilities: &[],
    composable: true,
    required_parts: &[],
    optional_parts: &["CalloutIcon", "CalloutTitle", "CalloutDescription"],
};

pub static CARD_META: ComponentMeta = ComponentMeta {
    id: "card",
    name: "Card",
    family: ComponentFamily::Layout,
    intent: "Group related content in a container",
    capabilities: &[],
    composable: true,
    required_parts: &[],
    optional_parts: &["CardHeader", "CardTitle", "CardDescription", "CardContent", "CardFooter"],
};

pub static CAROUSEL_META: ComponentMeta = ComponentMeta {
    id: "carousel",
    name: "Carousel",
    family: ComponentFamily::DataDisplay,
    intent: "Cycle through items horizontally",
    capabilities: &[Capability::KeyboardArrows],
    composable: true,
    required_parts: &["CarouselTrack", "CarouselItem"],
    optional_parts: &["CarouselPrev", "CarouselNext", "CarouselIndicators"],
};

pub static CHART_META: ComponentMeta = ComponentMeta {
    id: "chart",
    name: "Chart",
    family: ComponentFamily::DataDisplay,
    intent: "Visualize data graphically",
    capabilities: &[],
    composable: false,
    required_parts: &[],
    optional_parts: &[],
};

pub static CHECKBOX_META: ComponentMeta = ComponentMeta {
    id: "checkbox",
    name: "Checkbox",
    family: ComponentFamily::Input,
    intent: "Toggle a boolean value",
    capabilities: &[Capability::Disabled],
    composable: false,
    required_parts: &[],
    optional_parts: &[],
};

pub static CODE_BLOCK_META: ComponentMeta = ComponentMeta {
    id: "code-block",
    name: "Code Block",
    family: ComponentFamily::DataDisplay,
    intent: "Display syntax-highlighted code",
    capabilities: &[],
    composable: false,
    required_parts: &[],
    optional_parts: &[],
};

pub static COLLAPSIBLE_META: ComponentMeta = ComponentMeta {
    id: "collapsible",
    name: "Collapsible",
    family: ComponentFamily::Layout,
    intent: "Show and hide content sections",
    capabilities: &[Capability::OpenClose],
    composable: true,
    required_parts: &["CollapsibleTrigger", "CollapsibleContent"],
    optional_parts: &[],
};

pub static COLOR_PICKER_META: ComponentMeta = ComponentMeta {
    id: "color-picker",
    name: "Color Picker",
    family: ComponentFamily::Input,
    intent: "Select a color value",
    capabilities: &[Capability::Value, Capability::Disabled],
    composable: true,
    required_parts: &[],
    optional_parts: &["ColorPickerSwatch", "ColorPickerInput"],
};

pub static COMBOBOX_META: ComponentMeta = ComponentMeta {
    id: "combobox",
    name: "Combobox",
    family: ComponentFamily::Input,
    intent: "Search and select from a list",
    capabilities: &[Capability::OpenClose, Capability::Disabled],
    composable: true,
    required_parts: &["SelectTrigger", "SelectContent"],
    optional_parts: &[],
};

pub static COMMAND_META: ComponentMeta = ComponentMeta {
    id: "command",
    name: "Command",
    family: ComponentFamily::Interactive,
    intent: "Command palette for quick actions",
    capabilities: &[Capability::OpenClose, Capability::Typeahead],
    composable: true,
    required_parts: &["CommandInput", "CommandList"],
    optional_parts: &["CommandItem", "CommandGroup", "CommandSeparator", "CommandEmpty"],
};

pub static CONFIRM_DIALOG_META: ComponentMeta = ComponentMeta {
    id: "confirm-dialog",
    name: "Confirm Dialog",
    family: ComponentFamily::Overlay,
    intent: "Ask user to confirm an action",
    capabilities: &[Capability::OpenClose, Capability::FocusTrap, Capability::AriaModal],
    composable: false,
    required_parts: &[],
    optional_parts: &[],
};

pub static CONTEXT_MENU_META: ComponentMeta = ComponentMeta {
    id: "context-menu",
    name: "Context Menu",
    family: ComponentFamily::Interactive,
    intent: "Show menu on right-click",
    capabilities: &[Capability::OpenClose],
    composable: true,
    required_parts: &["ContextMenuTrigger", "ContextMenuContent"],
    optional_parts: &["ContextMenuItem", "ContextMenuSeparator"],
};

pub static COPY_BUTTON_META: ComponentMeta = ComponentMeta {
    id: "copy-button",
    name: "Copy Button",
    family: ComponentFamily::Utility,
    intent: "Copy text to clipboard on click",
    capabilities: &[],
    composable: false,
    required_parts: &[],
    optional_parts: &[],
};

pub static DATA_TABLE_META: ComponentMeta = ComponentMeta {
    id: "data-table",
    name: "Data Table",
    family: ComponentFamily::DataDisplay,
    intent: "Display sortable, filterable tabular data",
    capabilities: &[Capability::Multiple],
    composable: true,
    required_parts: &[],
    optional_parts: &["DataTablePagination", "DataTableToolbar", "DataTableColumn"],
};

pub static DIALOG_META: ComponentMeta = ComponentMeta {
    id: "dialog",
    name: "Dialog",
    family: ComponentFamily::Overlay,
    intent: "Display critical content requiring user interaction",
    capabilities: &[Capability::OpenClose, Capability::FocusTrap, Capability::KeyboardEsc, Capability::AriaModal],
    composable: true,
    required_parts: &["DialogContent", "DialogTitle"],
    optional_parts: &["DialogOverlay", "DialogDescription", "DialogClose"],
};

pub static DOC_PROGRESS_META: ComponentMeta = ComponentMeta {
    id: "doc-progress",
    name: "Doc Progress",
    family: ComponentFamily::Utility,
    intent: "Indicate reading progress in a document",
    capabilities: &[],
    composable: false,
    required_parts: &[],
    optional_parts: &[],
};

pub static DRAWER_META: ComponentMeta = ComponentMeta {
    id: "drawer",
    name: "Drawer",
    family: ComponentFamily::Overlay,
    intent: "Slide-in panel for supplementary content",
    capabilities: &[Capability::OpenClose, Capability::FocusTrap, Capability::KeyboardEsc, Capability::AriaModal],
    composable: true,
    required_parts: &["DrawerContent"],
    optional_parts: &["DrawerOverlay"],
};

pub static DROPDOWN_MENU_META: ComponentMeta = ComponentMeta {
    id: "dropdown-menu",
    name: "Dropdown Menu",
    family: ComponentFamily::Interactive,
    intent: "Show contextual action menu",
    capabilities: &[Capability::OpenClose, Capability::Disabled],
    composable: true,
    required_parts: &["DropdownMenuTrigger", "DropdownMenuContent"],
    optional_parts: &["DropdownMenuItem", "DropdownMenuSeparator", "DropdownMenuGroup"],
};

pub static EMPTY_STATE_META: ComponentMeta = ComponentMeta {
    id: "empty-state",
    name: "Empty State",
    family: ComponentFamily::Feedback,
    intent: "Display when no content is available",
    capabilities: &[],
    composable: true,
    required_parts: &[],
    optional_parts: &["EmptyStateIcon", "EmptyStateTitle", "EmptyStateDescription", "EmptyStateAction"],
};

pub static EMPTY_TABLE_META: ComponentMeta = ComponentMeta {
    id: "empty-table",
    name: "Empty Table",
    family: ComponentFamily::Utility,
    intent: "Show empty state inside a table",
    capabilities: &[],
    composable: false,
    required_parts: &[],
    optional_parts: &[],
};

pub static ERROR_STATE_META: ComponentMeta = ComponentMeta {
    id: "error-state",
    name: "Error State",
    family: ComponentFamily::Feedback,
    intent: "Display error condition to user",
    capabilities: &[],
    composable: true,
    required_parts: &[],
    optional_parts: &["ErrorStateIcon", "ErrorStateTitle", "ErrorStateDescription", "ErrorStateAction"],
};

pub static FIELD_META: ComponentMeta = ComponentMeta {
    id: "field",
    name: "Field",
    family: ComponentFamily::Utility,
    intent: "Wrap a form input with label and error",
    capabilities: &[],
    composable: false,
    required_parts: &[],
    optional_parts: &[],
};

pub static FORM_META: ComponentMeta = ComponentMeta {
    id: "form",
    name: "Form",
    family: ComponentFamily::Interactive,
    intent: "Form container with submit handling",
    capabilities: &[],
    composable: true,
    required_parts: &[],
    optional_parts: &[],
};

pub static FORM_ERROR_SUMMARY_META: ComponentMeta = ComponentMeta {
    id: "form-error-summary",
    name: "Form Error Summary",
    family: ComponentFamily::Utility,
    intent: "Summarize form validation errors",
    capabilities: &[],
    composable: false,
    required_parts: &[],
    optional_parts: &[],
};

pub static HOVER_CARD_META: ComponentMeta = ComponentMeta {
    id: "hover-card",
    name: "Hover Card",
    family: ComponentFamily::Overlay,
    intent: "Show rich preview on hover",
    capabilities: &[Capability::OpenClose],
    composable: true,
    required_parts: &["HoverCardTrigger", "HoverCardContent"],
    optional_parts: &[],
};

pub static ICON_META: ComponentMeta = ComponentMeta {
    id: "icon",
    name: "Icon",
    family: ComponentFamily::Utility,
    intent: "Display an SVG icon",
    capabilities: &[],
    composable: false,
    required_parts: &[],
    optional_parts: &[],
};

pub static ICON_BUTTON_META: ComponentMeta = ComponentMeta {
    id: "icon-button",
    name: "Icon Button",
    family: ComponentFamily::Interactive,
    intent: "Trigger an action with an icon button",
    capabilities: &[Capability::Disabled],
    composable: false,
    required_parts: &[],
    optional_parts: &[],
};

pub static INLINE_NOTICE_META: ComponentMeta = ComponentMeta {
    id: "inline-notice",
    name: "Inline Notice",
    family: ComponentFamily::Feedback,
    intent: "Show inline contextual feedback",
    capabilities: &[],
    composable: true,
    required_parts: &[],
    optional_parts: &["InlineNoticeIcon", "InlineNoticeContent"],
};

pub static INPUT_META: ComponentMeta = ComponentMeta {
    id: "input",
    name: "Input",
    family: ComponentFamily::Input,
    intent: "Capture text or data from user",
    capabilities: &[Capability::Value, Capability::Disabled],
    composable: false,
    required_parts: &[],
    optional_parts: &[],
};

pub static INPUT_GROUP_META: ComponentMeta = ComponentMeta {
    id: "input-group",
    name: "Input Group",
    family: ComponentFamily::Utility,
    intent: "Group input with prefix or suffix elements",
    capabilities: &[],
    composable: false,
    required_parts: &[],
    optional_parts: &[],
};

pub static INPUT_OTP_META: ComponentMeta = ComponentMeta {
    id: "input-otp",
    name: "OTP Input",
    family: ComponentFamily::Utility,
    intent: "Capture one-time password codes",
    capabilities: &[],
    composable: false,
    required_parts: &[],
    optional_parts: &[],
};

pub static KBD_META: ComponentMeta = ComponentMeta {
    id: "kbd",
    name: "Kbd",
    family: ComponentFamily::Typography,
    intent: "Display keyboard shortcut",
    capabilities: &[],
    composable: false,
    required_parts: &[],
    optional_parts: &[],
};

pub static LABEL_META: ComponentMeta = ComponentMeta {
    id: "label",
    name: "Label",
    family: ComponentFamily::Utility,
    intent: "Associate a label with a form control",
    capabilities: &[],
    composable: false,
    required_parts: &[],
    optional_parts: &[],
};

pub static LINK_META: ComponentMeta = ComponentMeta {
    id: "link",
    name: "Link",
    family: ComponentFamily::Typography,
    intent: "Navigate to a URL or trigger action",
    capabilities: &[Capability::Disabled],
    composable: false,
    required_parts: &[],
    optional_parts: &[],
};

pub static LIST_ITEM_META: ComponentMeta = ComponentMeta {
    id: "list-item",
    name: "List Item",
    family: ComponentFamily::DataDisplay,
    intent: "Display a single item in a list",
    capabilities: &[Capability::Selected, Capability::Disabled],
    composable: true,
    required_parts: &[],
    optional_parts: &["ListItemTitle", "ListItemDescription"],
};

pub static LOADING_OVERLAY_META: ComponentMeta = ComponentMeta {
    id: "loading-overlay",
    name: "Loading Overlay",
    family: ComponentFamily::Utility,
    intent: "Block UI during async operations",
    capabilities: &[Capability::OpenClose],
    composable: false,
    required_parts: &[],
    optional_parts: &[],
};

pub static MARKDOWN_META: ComponentMeta = ComponentMeta {
    id: "markdown",
    name: "Markdown",
    family: ComponentFamily::DataDisplay,
    intent: "Render markdown content as HTML",
    capabilities: &[],
    composable: false,
    required_parts: &[],
    optional_parts: &[],
};

pub static MENU_META: ComponentMeta = ComponentMeta {
    id: "menu",
    name: "Menu",
    family: ComponentFamily::Utility,
    intent: "Static vertical menu list",
    capabilities: &[],
    composable: false,
    required_parts: &[],
    optional_parts: &[],
};

pub static MENUBAR_META: ComponentMeta = ComponentMeta {
    id: "menubar",
    name: "Menubar",
    family: ComponentFamily::Interactive,
    intent: "Horizontal application menu bar",
    capabilities: &[Capability::OpenClose],
    composable: true,
    required_parts: &["MenubarMenu", "MenubarTrigger"],
    optional_parts: &["MenubarContent", "MenubarItem", "MenubarSeparator"],
};

pub static MODAL_META: ComponentMeta = ComponentMeta {
    id: "modal",
    name: "Modal",
    family: ComponentFamily::Overlay,
    intent: "Generic modal container",
    capabilities: &[Capability::OpenClose, Capability::FocusTrap, Capability::AriaModal],
    composable: true,
    required_parts: &[],
    optional_parts: &["ModalOverlay", "ModalContent", "ModalClose"],
};

pub static NAV_ITEM_META: ComponentMeta = ComponentMeta {
    id: "nav-item",
    name: "Nav Item",
    family: ComponentFamily::Navigation,
    intent: "Single navigation link item",
    capabilities: &[Capability::Active, Capability::Disabled],
    composable: false,
    required_parts: &[],
    optional_parts: &[],
};

pub static NAVIGATION_MENU_META: ComponentMeta = ComponentMeta {
    id: "navigation-menu",
    name: "Navigation Menu",
    family: ComponentFamily::Navigation,
    intent: "Primary site navigation with submenus",
    capabilities: &[Capability::OpenClose],
    composable: true,
    required_parts: &["NavigationMenuList", "NavigationMenuItem"],
    optional_parts: &["NavigationMenuTrigger", "NavigationMenuContent", "NavigationMenuLink"],
};

pub static PAGE_HEADER_META: ComponentMeta = ComponentMeta {
    id: "page-header",
    name: "Page Header",
    family: ComponentFamily::Layout,
    intent: "Page title and actions bar",
    capabilities: &[],
    composable: false,
    required_parts: &[],
    optional_parts: &[],
};

pub static PAGINATION_META: ComponentMeta = ComponentMeta {
    id: "pagination",
    name: "Pagination",
    family: ComponentFamily::Navigation,
    intent: "Navigate between pages of content",
    capabilities: &[Capability::Active, Capability::Disabled],
    composable: true,
    required_parts: &[],
    optional_parts: &["PaginationContent", "PaginationItem", "PaginationLink"],
};

pub static POPOVER_META: ComponentMeta = ComponentMeta {
    id: "popover",
    name: "Popover",
    family: ComponentFamily::Overlay,
    intent: "Show contextual floating content",
    capabilities: &[Capability::OpenClose, Capability::FocusTrap],
    composable: true,
    required_parts: &["PopoverTrigger", "PopoverContent"],
    optional_parts: &[],
};

pub static PROGRESS_META: ComponentMeta = ComponentMeta {
    id: "progress",
    name: "Progress",
    family: ComponentFamily::Feedback,
    intent: "Show completion of a task",
    capabilities: &[Capability::Value],
    composable: true,
    required_parts: &["ProgressIndicator"],
    optional_parts: &[],
};

pub static PULSE_META: ComponentMeta = ComponentMeta {
    id: "pulse",
    name: "Pulse",
    family: ComponentFamily::Feedback,
    intent: "Animated attention indicator",
    capabilities: &[],
    composable: false,
    required_parts: &[],
    optional_parts: &[],
};

pub static RADIO_META: ComponentMeta = ComponentMeta {
    id: "radio",
    name: "Radio",
    family: ComponentFamily::Input,
    intent: "Select one option from a group",
    capabilities: &[Capability::Disabled],
    composable: true,
    required_parts: &["RadioGroup"],
    optional_parts: &[],
};

pub static RADIO_GROUP_META: ComponentMeta = ComponentMeta {
    id: "radio-group",
    name: "Radio Group",
    family: ComponentFamily::Input,
    intent: "Group radio buttons for single selection",
    capabilities: &[Capability::Disabled],
    composable: false,
    required_parts: &[],
    optional_parts: &[],
};

pub static RESIZABLE_META: ComponentMeta = ComponentMeta {
    id: "resizable",
    name: "Resizable",
    family: ComponentFamily::Layout,
    intent: "Split panels with draggable divider",
    capabilities: &[Capability::Orientation, Capability::Resize],
    composable: true,
    required_parts: &["ResizablePanel", "ResizableHandle"],
    optional_parts: &[],
};

pub static SCROLL_AREA_META: ComponentMeta = ComponentMeta {
    id: "scroll-area",
    name: "Scroll Area",
    family: ComponentFamily::Layout,
    intent: "Scrollable container with custom scrollbar",
    capabilities: &[Capability::Overflow],
    composable: false,
    required_parts: &[],
    optional_parts: &[],
};

pub static SELECT_META: ComponentMeta = ComponentMeta {
    id: "select",
    name: "Select",
    family: ComponentFamily::Input,
    intent: "Choose one option from a list",
    capabilities: &[Capability::OpenClose, Capability::Disabled],
    composable: true,
    required_parts: &["SelectTrigger", "SelectContent", "SelectItem"],
    optional_parts: &["SelectValue", "SelectSeparator"],
};

pub static SEPARATOR_META: ComponentMeta = ComponentMeta {
    id: "separator",
    name: "Separator",
    family: ComponentFamily::Layout,
    intent: "Visually divide content sections",
    capabilities: &[Capability::Orientation],
    composable: false,
    required_parts: &[],
    optional_parts: &[],
};

pub static SHEET_META: ComponentMeta = ComponentMeta {
    id: "sheet",
    name: "Sheet",
    family: ComponentFamily::Overlay,
    intent: "Side panel for forms or navigation",
    capabilities: &[Capability::OpenClose, Capability::FocusTrap, Capability::KeyboardEsc],
    composable: true,
    required_parts: &["SheetContent"],
    optional_parts: &["SheetOverlay"],
};

pub static SIDEBAR_META: ComponentMeta = ComponentMeta {
    id: "sidebar",
    name: "Sidebar",
    family: ComponentFamily::Navigation,
    intent: "Vertical navigation panel",
    capabilities: &[Capability::OpenClose],
    composable: true,
    required_parts: &["SidebarContent"],
    optional_parts: &["SidebarHeader", "SidebarFooter", "SidebarMenu", "SidebarMenuItem"],
};

pub static SKELETON_META: ComponentMeta = ComponentMeta {
    id: "skeleton",
    name: "Skeleton",
    family: ComponentFamily::Feedback,
    intent: "Show placeholder while content loads",
    capabilities: &[],
    composable: false,
    required_parts: &[],
    optional_parts: &[],
};

pub static SLIDER_META: ComponentMeta = ComponentMeta {
    id: "slider",
    name: "Slider",
    family: ComponentFamily::Input,
    intent: "Select a value within a range",
    capabilities: &[Capability::Value, Capability::Disabled],
    composable: true,
    required_parts: &["SliderTrack", "SliderThumb"],
    optional_parts: &["SliderRange"],
};

pub static SPINNER_META: ComponentMeta = ComponentMeta {
    id: "spinner",
    name: "Spinner",
    family: ComponentFamily::Feedback,
    intent: "Indicate loading or processing",
    capabilities: &[],
    composable: false,
    required_parts: &[],
    optional_parts: &[],
};

pub static STAT_META: ComponentMeta = ComponentMeta {
    id: "stat",
    name: "Stat",
    family: ComponentFamily::DataDisplay,
    intent: "Display a key metric with label",
    capabilities: &[],
    composable: true,
    required_parts: &["StatValue", "StatLabel"],
    optional_parts: &["StatDelta", "StatIcon"],
};

pub static STATUS_DOT_META: ComponentMeta = ComponentMeta {
    id: "status-dot",
    name: "Status Dot",
    family: ComponentFamily::DataDisplay,
    intent: "Indicate user presence or availability",
    capabilities: &[],
    composable: false,
    required_parts: &[],
    optional_parts: &[],
};

pub static SWITCH_META: ComponentMeta = ComponentMeta {
    id: "switch",
    name: "Switch",
    family: ComponentFamily::Input,
    intent: "Toggle between on and off states",
    capabilities: &[Capability::Disabled],
    composable: false,
    required_parts: &[],
    optional_parts: &[],
};

pub static TABLE_META: ComponentMeta = ComponentMeta {
    id: "table",
    name: "Table",
    family: ComponentFamily::DataDisplay,
    intent: "Display tabular data",
    capabilities: &[Capability::Selected],
    composable: true,
    required_parts: &["TableHeader", "TableBody", "TableRow", "TableHead", "TableCell"],
    optional_parts: &["TableFooter", "TableCaption", "TableWrapper"],
};

pub static TABLE_OF_CONTENTS_META: ComponentMeta = ComponentMeta {
    id: "table-of-contents",
    name: "Table of Contents",
    family: ComponentFamily::Utility,
    intent: "Navigate document sections via anchors",
    capabilities: &[],
    composable: false,
    required_parts: &[],
    optional_parts: &[],
};

pub static TABS_META: ComponentMeta = ComponentMeta {
    id: "tabs",
    name: "Tabs",
    family: ComponentFamily::Navigation,
    intent: "Switch between related content panels",
    capabilities: &[Capability::Active],
    composable: true,
    required_parts: &["TabsList", "TabsTrigger", "TabsContent"],
    optional_parts: &[],
};

pub static TEXTAREA_META: ComponentMeta = ComponentMeta {
    id: "textarea",
    name: "Textarea",
    family: ComponentFamily::Input,
    intent: "Capture multi-line text from user",
    capabilities: &[Capability::Value, Capability::Disabled],
    composable: false,
    required_parts: &[],
    optional_parts: &[],
};

pub static TOAST_META: ComponentMeta = ComponentMeta {
    id: "toast",
    name: "Toast",
    family: ComponentFamily::Feedback,
    intent: "Show brief non-blocking notifications",
    capabilities: &[Capability::OpenClose],
    composable: true,
    required_parts: &["ToastViewport"],
    optional_parts: &["ToastTitle", "ToastDescription", "ToastAction", "ToastClose"],
};

pub static TOGGLE_META: ComponentMeta = ComponentMeta {
    id: "toggle",
    name: "Toggle",
    family: ComponentFamily::Input,
    intent: "Toggle a pressed state",
    capabilities: &[Capability::Pressed, Capability::Disabled],
    composable: false,
    required_parts: &[],
    optional_parts: &[],
};

pub static TOGGLE_GROUP_META: ComponentMeta = ComponentMeta {
    id: "toggle-group",
    name: "Toggle Group",
    family: ComponentFamily::Interactive,
    intent: "Group of toggle buttons with single or multiple selection",
    capabilities: &[Capability::Multiple, Capability::Disabled],
    composable: true,
    required_parts: &[],
    optional_parts: &[],
};

pub static TOOLBAR_META: ComponentMeta = ComponentMeta {
    id: "toolbar",
    name: "Toolbar",
    family: ComponentFamily::Layout,
    intent: "Action toolbar region",
    capabilities: &[],
    composable: true,
    required_parts: &[],
    optional_parts: &[],
};

pub static TOOLTIP_META: ComponentMeta = ComponentMeta {
    id: "tooltip",
    name: "Tooltip",
    family: ComponentFamily::Overlay,
    intent: "Show brief label on hover/focus",
    capabilities: &[Capability::OpenClose],
    composable: true,
    required_parts: &["TooltipTrigger", "TooltipContent"],
    optional_parts: &["TooltipProvider"],
};

pub static TREE_META: ComponentMeta = ComponentMeta {
    id: "tree",
    name: "Tree",
    family: ComponentFamily::DataDisplay,
    intent: "Display hierarchical data",
    capabilities: &[Capability::Selected],
    composable: true,
    required_parts: &["TreeItem"],
    optional_parts: &["TreeGroup"],
};

pub static VIRTUAL_LIST_META: ComponentMeta = ComponentMeta {
    id: "virtual-list",
    name: "Virtual List",
    family: ComponentFamily::DataDisplay,
    intent: "Efficiently render large lists",
    capabilities: &[Capability::VirtualScroll],
    composable: false,
    required_parts: &[],
    optional_parts: &[],
};

pub fn all_meta_generated() -> Vec<&'static ComponentMeta> {
    vec![
        &ACCORDION_META,
        &ALERT_META,
        &ALERT_DIALOG_META,
        &ANIMATE_META,
        &ASPECT_RATIO_META,
        &AVATAR_META,
        &BADGE_META,
        &BANNER_META,
        &BREADCRUMB_META,
        &BUTTON_META,
        &BUTTON_GROUP_META,
        &CALLOUT_META,
        &CARD_META,
        &CAROUSEL_META,
        &CHART_META,
        &CHECKBOX_META,
        &CODE_BLOCK_META,
        &COLLAPSIBLE_META,
        &COLOR_PICKER_META,
        &COMBOBOX_META,
        &COMMAND_META,
        &CONFIRM_DIALOG_META,
        &CONTEXT_MENU_META,
        &COPY_BUTTON_META,
        &DATA_TABLE_META,
        &DIALOG_META,
        &DOC_PROGRESS_META,
        &DRAWER_META,
        &DROPDOWN_MENU_META,
        &EMPTY_STATE_META,
        &EMPTY_TABLE_META,
        &ERROR_STATE_META,
        &FIELD_META,
        &FORM_META,
        &FORM_ERROR_SUMMARY_META,
        &HOVER_CARD_META,
        &ICON_META,
        &ICON_BUTTON_META,
        &INLINE_NOTICE_META,
        &INPUT_META,
        &INPUT_GROUP_META,
        &INPUT_OTP_META,
        &KBD_META,
        &LABEL_META,
        &LINK_META,
        &LIST_ITEM_META,
        &LOADING_OVERLAY_META,
        &MARKDOWN_META,
        &MENU_META,
        &MENUBAR_META,
        &MODAL_META,
        &NAV_ITEM_META,
        &NAVIGATION_MENU_META,
        &PAGE_HEADER_META,
        &PAGINATION_META,
        &POPOVER_META,
        &PROGRESS_META,
        &PULSE_META,
        &RADIO_META,
        &RADIO_GROUP_META,
        &RESIZABLE_META,
        &SCROLL_AREA_META,
        &SELECT_META,
        &SEPARATOR_META,
        &SHEET_META,
        &SIDEBAR_META,
        &SKELETON_META,
        &SLIDER_META,
        &SPINNER_META,
        &STAT_META,
        &STATUS_DOT_META,
        &SWITCH_META,
        &TABLE_META,
        &TABLE_OF_CONTENTS_META,
        &TABS_META,
        &TEXTAREA_META,
        &TOAST_META,
        &TOGGLE_META,
        &TOGGLE_GROUP_META,
        &TOOLBAR_META,
        &TOOLTIP_META,
        &TREE_META,
        &VIRTUAL_LIST_META,
    ]
}
