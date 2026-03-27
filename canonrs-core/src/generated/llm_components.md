# CanonRS — UI Components

> AUTO-GENERATED — do not edit manually

---

## `accordion`

- **Label:** Accordion
- **Family:** layout
- **Category:** Navigation
- **Intent:** Expand and collapse content sections
- **Description:** Expandable accordion sections
- **Composable:** yes
- **Capabilities:** OpenClose, Multiple
- **Required parts:** AccordionItem, AccordionTrigger, AccordionContent
- **Tags:** accordion, collapsible, expand, sections, faq
- **Use when:** structuring page content, containers, grids
- **Often used with:** Breadcrumb, Pagination, Tabs, Sidebar, NavItem

---

## `alert`

- **Label:** Alert
- **Family:** feedback
- **Category:** Feedback
- **Intent:** Display important static messages
- **Description:** Alert message box
- **Composable:** yes
- **Optional parts:** AlertTitle, AlertDescription
- **Tags:** alert, warning, info, message, error
- **Use when:** showing status, alerts, notifications, loading states
- **Often used with:** Toast, Alert, Banner, Spinner, Skeleton, Progress

---

## `alert-dialog`

- **Label:** Alert Dialog
- **Family:** overlay
- **Category:** Overlay
- **Intent:** Confirm destructive actions with user
- **Description:** Alert dialog for critical confirmations
- **Composable:** yes
- **Capabilities:** OpenClose, FocusTrap, AriaModal
- **Required parts:** AlertDialogContent, AlertDialogTitle
- **Optional parts:** AlertDialogOverlay, AlertDialogDescription, AlertDialogAction
- **Tags:** alert-dialog, confirm, destructive, modal, overlay
- **Use when:** modal dialogs, drawers, popovers, tooltips
- **Often used with:** Dialog, Drawer, Sheet, Modal, Popover, Tooltip

---

## `animate`

- **Label:** Animate
- **Family:** utility
- **Category:** Display
- **Intent:** Apply CSS animations to children
- **Description:** Animation wrapper component
- **Composable:** no
- **Tags:** animate, animation, transition, motion
- **Use when:** helper components, wrappers, decorators
- **Often used with:** Badge, Avatar, Card, Stat, Icon, Chart

---

## `aspect-ratio`

- **Label:** Aspect Ratio
- **Family:** layout
- **Category:** Display
- **Intent:** Maintain consistent width/height ratio
- **Description:** Aspect ratio container
- **Composable:** no
- **Tags:** aspect-ratio, ratio, 16:9, 4:3, image, video
- **Use when:** structuring page content, containers, grids
- **Often used with:** Badge, Avatar, Card, Stat, Icon, Chart

---

## `avatar`

- **Label:** Avatar
- **Family:** data_display
- **Category:** Display
- **Intent:** Display user profile image with fallback
- **Description:** User avatar image
- **Composable:** yes
- **Optional parts:** AvatarImage, AvatarFallback
- **Tags:** avatar, photo, profile, user, image
- **Use when:** presenting data, lists, tables, charts, metrics
- **Often used with:** Badge, Avatar, Card, Stat, Icon, Chart

---

## `badge`

- **Label:** Badge
- **Family:** data_display
- **Category:** Display
- **Intent:** Display status, count or label
- **Description:** Status badge label
- **Composable:** no
- **Tags:** badge, tag, status, label, notification
- **Use when:** presenting data, lists, tables, charts, metrics
- **Often used with:** Badge, Avatar, Card, Stat, Icon, Chart

---

## `banner`

- **Label:** Banner
- **Family:** feedback
- **Category:** Feedback
- **Intent:** Display persistent page-level messages
- **Description:** Banner message
- **Composable:** yes
- **Capabilities:** OpenClose
- **Optional parts:** BannerContent, BannerActions, BannerClose
- **Tags:** banner, announcement, notification, top, message
- **Use when:** showing status, alerts, notifications, loading states
- **Often used with:** Toast, Alert, Banner, Spinner, Skeleton, Progress

---

## `breadcrumb`

- **Label:** Breadcrumb
- **Family:** navigation
- **Category:** Navigation
- **Intent:** Show current location in hierarchy
- **Description:** Navigation breadcrumb trail
- **Composable:** yes
- **Optional parts:** BreadcrumbItem, BreadcrumbLink, BreadcrumbSeparator
- **Tags:** breadcrumb, path, navigation, trail, location
- **Use when:** routing, menus, breadcrumbs, pagination
- **Often used with:** Breadcrumb, Pagination, Tabs, Sidebar, NavItem

---

## `button`

- **Label:** Button
- **Family:** interactive
- **Category:** Action
- **Intent:** Trigger an action or event
- **Description:** Action button with variant and size
- **Composable:** no
- **Capabilities:** Disabled
- **Tags:** button, action, submit, click, cta
- **Use when:** triggering actions, user input, form submission
- **Often used with:** Button, IconButton, ButtonGroup, DropdownMenu

---

## `button-group`

- **Label:** Button Group
- **Family:** utility
- **Category:** Action
- **Intent:** Group related action buttons together
- **Description:** Group of action buttons
- **Composable:** no
- **Tags:** button-group, buttons, group, actions, multiple
- **Use when:** helper components, wrappers, decorators
- **Often used with:** Button, IconButton, ButtonGroup, DropdownMenu

---

## `callout`

- **Label:** Callout
- **Family:** feedback
- **Category:** Feedback
- **Intent:** Highlight important contextual information
- **Description:** Callout info box
- **Composable:** yes
- **Optional parts:** CalloutIcon, CalloutTitle, CalloutDescription
- **Tags:** callout, highlight, info, note, warning, tip
- **Use when:** showing status, alerts, notifications, loading states
- **Often used with:** Toast, Alert, Banner, Spinner, Skeleton, Progress

---

## `card`

- **Label:** Card
- **Family:** layout
- **Category:** Display
- **Intent:** Group related content in a container
- **Description:** Card component
- **Composable:** yes
- **Optional parts:** CardHeader, CardTitle, CardDescription, CardContent, CardFooter
- **Tags:** card, container, group, content
- **Use when:** structuring page content, containers, grids
- **Often used with:** Badge, Avatar, Card, Stat, Icon, Chart

---

## `carousel`

- **Label:** Carousel
- **Family:** data_display
- **Category:** Display
- **Intent:** Cycle through items horizontally
- **Description:** Image carousel slider
- **Composable:** yes
- **Capabilities:** KeyboardArrows
- **Required parts:** CarouselTrack, CarouselItem
- **Optional parts:** CarouselPrev, CarouselNext, CarouselIndicators
- **Tags:** carousel, slider, gallery, images, slideshow
- **Use when:** presenting data, lists, tables, charts, metrics
- **Often used with:** Badge, Avatar, Card, Stat, Icon, Chart

---

## `chart`

- **Label:** Chart
- **Family:** data_display
- **Category:** Display
- **Intent:** Visualize data graphically
- **Description:** Data chart visualization
- **Composable:** no
- **Tags:** chart, graph, bar, line, pie, area, data, visualization
- **Use when:** presenting data, lists, tables, charts, metrics
- **Often used with:** Badge, Avatar, Card, Stat, Icon, Chart

---

## `checkbox`

- **Label:** Checkbox
- **Family:** input
- **Category:** Form
- **Intent:** Toggle a boolean value
- **Description:** Checkbox input
- **Composable:** no
- **Capabilities:** Disabled
- **Tags:** checkbox, check, tick, selection, multiple
- **Use when:** collecting user input, forms, search
- **Often used with:** Field, Label, Input, Select, Checkbox, Switch, Textarea

---

## `code-block`

- **Label:** Code Block
- **Family:** data_display
- **Category:** Display
- **Intent:** Display syntax-highlighted code
- **Description:** Syntax-highlighted code display
- **Composable:** no
- **Tags:** code-block, code, syntax, highlight, snippet, technical
- **Use when:** presenting data, lists, tables, charts, metrics
- **Often used with:** Badge, Avatar, Card, Stat, Icon, Chart

---

## `collapsible`

- **Label:** Collapsible
- **Family:** layout
- **Category:** Navigation
- **Intent:** Show and hide content sections
- **Description:** Collapsible section
- **Composable:** yes
- **Capabilities:** OpenClose
- **Required parts:** CollapsibleTrigger, CollapsibleContent
- **Tags:** collapsible, collapse, expand, hide, toggle
- **Use when:** structuring page content, containers, grids
- **Often used with:** Breadcrumb, Pagination, Tabs, Sidebar, NavItem

---

## `color-picker`

- **Label:** Color Picker
- **Family:** input
- **Category:** Form
- **Intent:** Select a color value
- **Description:** Color picker input
- **Composable:** yes
- **Capabilities:** Value, Disabled
- **Optional parts:** ColorPickerSwatch, ColorPickerInput
- **Tags:** color-picker, color, palette, rgb, hex
- **Use when:** collecting user input, forms, search
- **Often used with:** Field, Label, Input, Select, Checkbox, Switch, Textarea

---

## `combobox`

- **Label:** Combobox
- **Family:** input
- **Category:** Form
- **Intent:** Search and select from a list
- **Description:** Searchable combo box
- **Composable:** yes
- **Capabilities:** OpenClose, Disabled
- **Required parts:** SelectTrigger, SelectContent
- **Tags:** combobox, search, autocomplete, filter, combo
- **Use when:** collecting user input, forms, search
- **Often used with:** Field, Label, Input, Select, Checkbox, Switch, Textarea

---

## `command`

- **Label:** Command
- **Family:** interactive
- **Category:** Action
- **Intent:** Command palette for quick actions
- **Description:** Command palette
- **Composable:** yes
- **Capabilities:** OpenClose, Typeahead
- **Required parts:** CommandInput, CommandList
- **Optional parts:** CommandItem, CommandGroup, CommandSeparator, CommandEmpty
- **Tags:** command, palette, spotlight, search, shortcut
- **Use when:** triggering actions, user input, form submission
- **Often used with:** Button, IconButton, ButtonGroup, DropdownMenu

---

## `confirm-dialog`

- **Label:** Confirm Dialog
- **Family:** overlay
- **Category:** Overlay
- **Intent:** Ask user to confirm an action
- **Description:** Confirmation dialog
- **Composable:** no
- **Capabilities:** OpenClose, FocusTrap, AriaModal
- **Tags:** confirm-dialog, confirm, confirmation, cancel, ok
- **Use when:** modal dialogs, drawers, popovers, tooltips
- **Often used with:** Dialog, Drawer, Sheet, Modal, Popover, Tooltip

---

## `context-menu`

- **Label:** Context Menu
- **Family:** interactive
- **Category:** Action
- **Intent:** Show menu on right-click
- **Description:** Right-click context menu
- **Composable:** yes
- **Capabilities:** OpenClose
- **Required parts:** ContextMenuTrigger, ContextMenuContent
- **Optional parts:** ContextMenuItem, ContextMenuSeparator
- **Tags:** context-menu, right-click, menu, options, contextual
- **Use when:** triggering actions, user input, form submission
- **Often used with:** Button, IconButton, ButtonGroup, DropdownMenu

---

## `copy-button`

- **Label:** Copy Button
- **Family:** utility
- **Category:** Action
- **Intent:** Copy text to clipboard on click
- **Description:** Clipboard copy button
- **Composable:** no
- **Tags:** copy-button, copy, clipboard, button, action
- **Use when:** helper components, wrappers, decorators
- **Often used with:** Button, IconButton, ButtonGroup, DropdownMenu

---

## `data-table`

- **Label:** Data Table
- **Family:** data_display
- **Category:** Data
- **Intent:** Display sortable, filterable tabular data
- **Description:** Sortable data table component
- **Composable:** yes
- **Capabilities:** Multiple
- **Optional parts:** DataTablePagination, DataTableToolbar, DataTableColumn
- **Tags:** data-table, table, data, grid, sortable, filterable
- **Use when:** presenting data, lists, tables, charts, metrics
- **Often used with:** DataTable, DataTableColumn, Pagination, EmptyTable

---

## `dialog`

- **Label:** Dialog
- **Family:** overlay
- **Category:** Overlay
- **Intent:** Display critical content requiring user interaction
- **Description:** Modal dialog component
- **Composable:** yes
- **Capabilities:** OpenClose, FocusTrap, KeyboardEsc, AriaModal
- **Required parts:** DialogContent, DialogTitle
- **Optional parts:** DialogOverlay, DialogDescription, DialogClose
- **Tags:** dialog, modal, popup, window, overlay, confirmation
- **Use when:** modal dialogs, drawers, popovers, tooltips
- **Often used with:** Dialog, Drawer, Sheet, Modal, Popover, Tooltip

---

## `doc-progress`

- **Label:** Doc Progress
- **Family:** utility
- **Category:** Display
- **Intent:** Indicate reading progress in a document
- **Description:** Document progress indicator
- **Composable:** no
- **Tags:** doc-progress, document, progress, reading, steps
- **Use when:** helper components, wrappers, decorators
- **Often used with:** Badge, Avatar, Card, Stat, Icon, Chart

---

## `drawer`

- **Label:** Drawer
- **Family:** overlay
- **Category:** Overlay
- **Intent:** Slide-in panel for supplementary content
- **Description:** Slide-out drawer component
- **Composable:** yes
- **Capabilities:** OpenClose, FocusTrap, KeyboardEsc, AriaModal
- **Required parts:** DrawerContent
- **Optional parts:** DrawerOverlay
- **Tags:** drawer, slide, lateral, panel, mobile
- **Use when:** modal dialogs, drawers, popovers, tooltips
- **Often used with:** Dialog, Drawer, Sheet, Modal, Popover, Tooltip

---

## `dropdown-menu`

- **Label:** Dropdown Menu
- **Family:** interactive
- **Category:** Action
- **Intent:** Show contextual action menu
- **Description:** Dropdown menu
- **Composable:** yes
- **Capabilities:** OpenClose, Disabled
- **Required parts:** DropdownMenuTrigger, DropdownMenuContent
- **Optional parts:** DropdownMenuItem, DropdownMenuSeparator, DropdownMenuGroup
- **Tags:** dropdown-menu, dropdown, menu, options, actions
- **Use when:** triggering actions, user input, form submission
- **Often used with:** Button, IconButton, ButtonGroup, DropdownMenu

---

## `empty-state`

- **Label:** Empty State
- **Family:** feedback
- **Category:** Feedback
- **Intent:** Display when no content is available
- **Description:** Empty state placeholder
- **Composable:** yes
- **Optional parts:** EmptyStateIcon, EmptyStateTitle, EmptyStateDescription, EmptyStateAction
- **Tags:** empty-state, empty, no-data, placeholder, zero-state
- **Use when:** showing status, alerts, notifications, loading states
- **Often used with:** Toast, Alert, Banner, Spinner, Skeleton, Progress

---

## `empty-table`

- **Label:** Empty Table
- **Family:** utility
- **Category:** Display
- **Intent:** Show empty state inside a table
- **Description:** Empty table state display
- **Composable:** no
- **Tags:** empty-table, empty, no-records, no-data
- **Use when:** helper components, wrappers, decorators
- **Often used with:** Badge, Avatar, Card, Stat, Icon, Chart

---

## `error-state`

- **Label:** Error State
- **Family:** feedback
- **Category:** Feedback
- **Intent:** Display error condition to user
- **Description:** Error state display
- **Composable:** yes
- **Optional parts:** ErrorStateIcon, ErrorStateTitle, ErrorStateDescription, ErrorStateAction
- **Tags:** error-state, error, failure, problem, try-again
- **Use when:** showing status, alerts, notifications, loading states
- **Often used with:** Toast, Alert, Banner, Spinner, Skeleton, Progress

---

## `field`

- **Label:** Field
- **Family:** utility
- **Category:** Form
- **Intent:** Wrap a form input with label and error
- **Description:** Form field wrapper with label and error
- **Composable:** no
- **Tags:** field, label, error, form, wrapper
- **Use when:** helper components, wrappers, decorators
- **Often used with:** Field, Label, Input, Select, Checkbox, Switch, Textarea

---

## `form`

- **Label:** Form
- **Family:** interactive
- **Category:** Form
- **Intent:** Form container with submit handling
- **Description:** Form component
- **Composable:** yes
- **Tags:** form, submit, validation, input, fields
- **Use when:** triggering actions, user input, form submission
- **Often used with:** Field, Label, Input, Select, Checkbox, Switch, Textarea

---

## `form-error-summary`

- **Label:** Form Error Summary
- **Family:** utility
- **Category:** Form
- **Intent:** Summarize form validation errors
- **Description:** Form validation error summary
- **Composable:** no
- **Tags:** form-error-summary, error, validation, form, summary
- **Use when:** helper components, wrappers, decorators
- **Often used with:** Field, Label, Input, Select, Checkbox, Switch, Textarea

---

## `hover-card`

- **Label:** Hover Card
- **Family:** overlay
- **Category:** Overlay
- **Intent:** Show rich preview on hover
- **Description:** Hover card popup
- **Composable:** yes
- **Capabilities:** OpenClose
- **Required parts:** HoverCardTrigger, HoverCardContent
- **Tags:** hover-card, preview, card, hover, popup
- **Use when:** modal dialogs, drawers, popovers, tooltips
- **Often used with:** Dialog, Drawer, Sheet, Modal, Popover, Tooltip

---

## `icon`

- **Label:** Icon
- **Family:** utility
- **Category:** Display
- **Intent:** Display an SVG icon
- **Description:** SVG icon display
- **Composable:** no
- **Tags:** icon, svg, symbol, visual
- **Use when:** helper components, wrappers, decorators
- **Often used with:** Badge, Avatar, Card, Stat, Icon, Chart

---

## `icon-button`

- **Label:** Icon Button
- **Family:** interactive
- **Category:** Action
- **Intent:** Trigger an action with an icon button
- **Description:** Button with icon only
- **Composable:** no
- **Capabilities:** Disabled
- **Tags:** icon-button, icon, action, close, delete, edit
- **Use when:** triggering actions, user input, form submission
- **Often used with:** Button, IconButton, ButtonGroup, DropdownMenu

---

## `inline-notice`

- **Label:** Inline Notice
- **Family:** feedback
- **Category:** Feedback
- **Intent:** Show inline contextual feedback
- **Description:** Inline notice message
- **Composable:** yes
- **Optional parts:** InlineNoticeIcon, InlineNoticeContent
- **Tags:** inline-notice, notice, warning, inline, message, info
- **Use when:** showing status, alerts, notifications, loading states
- **Often used with:** Toast, Alert, Banner, Spinner, Skeleton, Progress

---

## `input`

- **Label:** Input
- **Family:** input
- **Category:** Form
- **Intent:** Capture text or data from user
- **Description:** Text input field
- **Composable:** no
- **Capabilities:** Value, Disabled
- **Tags:** input, field, text, form
- **Use when:** collecting user input, forms, search
- **Often used with:** Field, Label, Input, Select, Checkbox, Switch, Textarea

---

## `input-group`

- **Label:** Input Group
- **Family:** utility
- **Category:** Form
- **Intent:** Group input with prefix or suffix elements
- **Description:** Input group with addons
- **Composable:** no
- **Tags:** input-group, prefix, suffix, addon, field
- **Use when:** helper components, wrappers, decorators
- **Often used with:** Field, Label, Input, Select, Checkbox, Switch, Textarea

---

## `input-otp`

- **Label:** OTP Input
- **Family:** utility
- **Category:** Form
- **Intent:** Capture one-time password codes
- **Description:** One-time password input
- **Composable:** no
- **Tags:** input-otp, otp, code, verification, sms, token, pin
- **Use when:** helper components, wrappers, decorators
- **Often used with:** Field, Label, Input, Select, Checkbox, Switch, Textarea

---

## `kbd`

- **Label:** Kbd
- **Family:** typography
- **Category:** Display
- **Intent:** Display keyboard shortcut
- **Description:** Keyboard shortcut display
- **Composable:** no
- **Tags:** kbd, keyboard, shortcut, ctrl, cmd
- **Use when:** text display, labels, headings, links
- **Often used with:** Badge, Avatar, Card, Stat, Icon, Chart

---

## `label`

- **Label:** Label
- **Family:** utility
- **Category:** Form
- **Intent:** Associate a label with a form control
- **Description:** Form label component
- **Composable:** no
- **Tags:** label, form, accessibility, input
- **Use when:** helper components, wrappers, decorators
- **Often used with:** Field, Label, Input, Select, Checkbox, Switch, Textarea

---

## `link`

- **Label:** Link
- **Family:** typography
- **Category:** Navigation
- **Intent:** Navigate to a URL or trigger action
- **Description:** Hyperlink
- **Composable:** no
- **Capabilities:** Disabled
- **Tags:** link, anchor, href, url, navigate, click
- **Use when:** text display, labels, headings, links
- **Often used with:** Breadcrumb, Pagination, Tabs, Sidebar, NavItem

---

## `list-item`

- **Label:** List Item
- **Family:** data_display
- **Category:** Display
- **Intent:** Display a single item in a list
- **Description:** Single list item with title and description
- **Composable:** yes
- **Capabilities:** Selected, Disabled
- **Optional parts:** ListItemTitle, ListItemDescription
- **Tags:** list-item, item, row, entry, element
- **Use when:** presenting data, lists, tables, charts, metrics
- **Often used with:** Badge, Avatar, Card, Stat, Icon, Chart

---

## `loading-overlay`

- **Label:** Loading Overlay
- **Family:** utility
- **Category:** Display
- **Intent:** Block UI during async operations
- **Description:** Full loading overlay
- **Composable:** no
- **Capabilities:** OpenClose
- **Tags:** loading-overlay, loading, overlay, wait, spinner, block
- **Use when:** helper components, wrappers, decorators
- **Often used with:** Badge, Avatar, Card, Stat, Icon, Chart

---

## `markdown`

- **Label:** Markdown
- **Family:** data_display
- **Category:** Display
- **Intent:** Render markdown content as HTML
- **Description:** Rendered markdown content
- **Composable:** no
- **Tags:** markdown, text, rich-text, content, document, blog
- **Use when:** presenting data, lists, tables, charts, metrics
- **Often used with:** Badge, Avatar, Card, Stat, Icon, Chart

---

## `menu`

- **Label:** Menu
- **Family:** utility
- **Category:** Navigation
- **Intent:** Static vertical menu list
- **Description:** Menu component
- **Composable:** no
- **Tags:** menu, list, options, actions, items
- **Use when:** helper components, wrappers, decorators
- **Often used with:** Breadcrumb, Pagination, Tabs, Sidebar, NavItem

---

## `menubar`

- **Label:** Menubar
- **Family:** interactive
- **Category:** Navigation
- **Intent:** Horizontal application menu bar
- **Description:** Menu bar navigation
- **Composable:** yes
- **Capabilities:** OpenClose
- **Required parts:** MenubarMenu, MenubarTrigger
- **Optional parts:** MenubarContent, MenubarItem, MenubarSeparator
- **Tags:** menubar, navigation, desktop, app
- **Use when:** triggering actions, user input, form submission
- **Often used with:** Breadcrumb, Pagination, Tabs, Sidebar, NavItem

---

## `modal`

- **Label:** Modal
- **Family:** overlay
- **Category:** Overlay
- **Intent:** Generic modal container
- **Description:** Modal window component
- **Composable:** yes
- **Capabilities:** OpenClose, FocusTrap, AriaModal
- **Optional parts:** ModalOverlay, ModalContent, ModalClose
- **Tags:** modal, window, popup, overlay, dialog
- **Use when:** modal dialogs, drawers, popovers, tooltips
- **Often used with:** Dialog, Drawer, Sheet, Modal, Popover, Tooltip

---

## `nav-item`

- **Label:** Nav Item
- **Family:** navigation
- **Category:** Navigation
- **Intent:** Single navigation link item
- **Description:** Single navigation item
- **Composable:** no
- **Capabilities:** Active, Disabled
- **Tags:** nav-item, link, menu-item, navigation, page
- **Use when:** routing, menus, breadcrumbs, pagination
- **Often used with:** Breadcrumb, Pagination, Tabs, Sidebar, NavItem

---

## `navigation-menu`

- **Label:** Navigation Menu
- **Family:** navigation
- **Category:** Navigation
- **Intent:** Primary site navigation with submenus
- **Description:** Navigation menu
- **Composable:** yes
- **Capabilities:** OpenClose
- **Required parts:** NavigationMenuList, NavigationMenuItem
- **Optional parts:** NavigationMenuTrigger, NavigationMenuContent, NavigationMenuLink
- **Tags:** navigation-menu, navigation, menu, nav, links, site, header
- **Use when:** routing, menus, breadcrumbs, pagination
- **Often used with:** Breadcrumb, Pagination, Tabs, Sidebar, NavItem

---

## `page-header`

- **Label:** Page Header
- **Family:** layout
- **Category:** Display
- **Intent:** Page title and actions bar
- **Description:** Page header with title and actions
- **Composable:** no
- **Tags:** page-header, title, heading, actions, breadcrumb
- **Use when:** structuring page content, containers, grids
- **Often used with:** Badge, Avatar, Card, Stat, Icon, Chart

---

## `pagination`

- **Label:** Pagination
- **Family:** navigation
- **Category:** Navigation
- **Intent:** Navigate between pages of content
- **Description:** Page navigation control
- **Composable:** yes
- **Capabilities:** Active, Disabled
- **Optional parts:** PaginationContent, PaginationItem, PaginationLink
- **Tags:** pagination, pages, next, prev, navigate
- **Use when:** routing, menus, breadcrumbs, pagination
- **Often used with:** Breadcrumb, Pagination, Tabs, Sidebar, NavItem

---

## `popover`

- **Label:** Popover
- **Family:** overlay
- **Category:** Overlay
- **Intent:** Show contextual floating content
- **Description:** Floating popover component
- **Composable:** yes
- **Capabilities:** OpenClose, FocusTrap
- **Required parts:** PopoverTrigger, PopoverContent
- **Tags:** popover, floating, tooltip, overlay, context
- **Use when:** modal dialogs, drawers, popovers, tooltips
- **Often used with:** Dialog, Drawer, Sheet, Modal, Popover, Tooltip

---

## `progress`

- **Label:** Progress
- **Family:** feedback
- **Category:** Feedback
- **Intent:** Show completion of a task
- **Description:** Progress bar indicator
- **Composable:** yes
- **Capabilities:** Value
- **Required parts:** ProgressIndicator
- **Tags:** progress, bar, loading, percentage, completion
- **Use when:** showing status, alerts, notifications, loading states
- **Often used with:** Toast, Alert, Banner, Spinner, Skeleton, Progress

---

## `pulse`

- **Label:** Pulse
- **Family:** feedback
- **Category:** Feedback
- **Intent:** Animated attention indicator
- **Description:** Pulse animation wrapper
- **Composable:** no
- **Tags:** pulse, animation, glow, attention, highlight, ping
- **Use when:** showing status, alerts, notifications, loading states
- **Often used with:** Toast, Alert, Banner, Spinner, Skeleton, Progress

---

## `radio`

- **Label:** Radio
- **Family:** input
- **Category:** Form
- **Intent:** Select one option from a group
- **Description:** Radio button input
- **Composable:** yes
- **Capabilities:** Disabled
- **Required parts:** RadioGroup
- **Tags:** radio, choice, exclusive, selection
- **Use when:** collecting user input, forms, search
- **Often used with:** Field, Label, Input, Select, Checkbox, Switch, Textarea

---

## `radio-group`

- **Label:** Radio Group
- **Family:** input
- **Category:** Form
- **Intent:** Group radio buttons for single selection
- **Description:** Group of radio buttons
- **Composable:** no
- **Capabilities:** Disabled
- **Tags:** radio-group, radio, group, options, alternatives, choice
- **Use when:** collecting user input, forms, search
- **Often used with:** Field, Label, Input, Select, Checkbox, Switch, Textarea

---

## `resizable`

- **Label:** Resizable
- **Family:** layout
- **Category:** Layout
- **Intent:** Split panels with draggable divider
- **Description:** Resizable panel component
- **Composable:** yes
- **Capabilities:** Orientation, Resize
- **Required parts:** ResizablePanel, ResizableHandle
- **Tags:** resizable, resize, panel, split, adjust
- **Use when:** structuring page content, containers, grids
- **Often used with:** Card, Container, Grid, Stack, Columns

---

## `scroll-area`

- **Label:** Scroll Area
- **Family:** layout
- **Category:** Layout
- **Intent:** Scrollable container with custom scrollbar
- **Description:** Scrollable area container
- **Composable:** no
- **Capabilities:** Overflow
- **Tags:** scroll-area, scroll, overflow, container, long-list
- **Use when:** structuring page content, containers, grids
- **Often used with:** Card, Container, Grid, Stack, Columns

---

## `select`

- **Label:** Select
- **Family:** input
- **Category:** Form
- **Intent:** Choose one option from a list
- **Description:** Dropdown select input
- **Composable:** yes
- **Capabilities:** OpenClose, Disabled
- **Required parts:** SelectTrigger, SelectContent, SelectItem
- **Optional parts:** SelectValue, SelectSeparator
- **Tags:** select, dropdown, choose, options, list, combo
- **Use when:** collecting user input, forms, search
- **Often used with:** Field, Label, Input, Select, Checkbox, Switch, Textarea

---

## `separator`

- **Label:** Separator
- **Family:** layout
- **Category:** Layout
- **Intent:** Visually divide content sections
- **Description:** Visual divider line
- **Composable:** no
- **Capabilities:** Orientation
- **Tags:** separator, divider, line, hr, section
- **Use when:** structuring page content, containers, grids
- **Often used with:** Card, Container, Grid, Stack, Columns

---

## `sheet`

- **Label:** Sheet
- **Family:** overlay
- **Category:** Overlay
- **Intent:** Side panel for forms or navigation
- **Description:** Sheet panel overlay
- **Composable:** yes
- **Capabilities:** OpenClose, FocusTrap, KeyboardEsc
- **Required parts:** SheetContent
- **Optional parts:** SheetOverlay
- **Tags:** sheet, panel, lateral, slide, drawer, mobile
- **Use when:** modal dialogs, drawers, popovers, tooltips
- **Often used with:** Dialog, Drawer, Sheet, Modal, Popover, Tooltip

---

## `sidebar`

- **Label:** Sidebar
- **Family:** navigation
- **Category:** Navigation
- **Intent:** Vertical navigation panel
- **Description:** Sidebar navigation component
- **Composable:** yes
- **Capabilities:** OpenClose
- **Required parts:** SidebarContent
- **Optional parts:** SidebarHeader, SidebarFooter, SidebarMenu, SidebarMenuItem
- **Tags:** sidebar, nav, navigation, links, left-panel
- **Use when:** routing, menus, breadcrumbs, pagination
- **Often used with:** Breadcrumb, Pagination, Tabs, Sidebar, NavItem

---

## `skeleton`

- **Label:** Skeleton
- **Family:** feedback
- **Category:** Feedback
- **Intent:** Show placeholder while content loads
- **Description:** Loading skeleton placeholder
- **Composable:** no
- **Tags:** skeleton, loading, placeholder, shimmer
- **Use when:** showing status, alerts, notifications, loading states
- **Often used with:** Toast, Alert, Banner, Spinner, Skeleton, Progress

---

## `slider`

- **Label:** Slider
- **Family:** input
- **Category:** Form
- **Intent:** Select a value within a range
- **Description:** Range slider input
- **Composable:** yes
- **Capabilities:** Value, Disabled
- **Required parts:** SliderTrack, SliderThumb
- **Optional parts:** SliderRange
- **Tags:** slider, range, interval, volume, value, drag
- **Use when:** collecting user input, forms, search
- **Often used with:** Field, Label, Input, Select, Checkbox, Switch, Textarea

---

## `spinner`

- **Label:** Spinner
- **Family:** feedback
- **Category:** Feedback
- **Intent:** Indicate loading or processing
- **Description:** Loading spinner
- **Composable:** no
- **Tags:** spinner, loading, circular, wait
- **Use when:** showing status, alerts, notifications, loading states
- **Often used with:** Toast, Alert, Banner, Spinner, Skeleton, Progress

---

## `stat`

- **Label:** Stat
- **Family:** data_display
- **Category:** Display
- **Intent:** Display a key metric with label
- **Description:** Metric stat display
- **Composable:** yes
- **Required parts:** StatValue, StatLabel
- **Optional parts:** StatDelta, StatIcon
- **Tags:** stat, metric, number, kpi, indicator, value
- **Use when:** presenting data, lists, tables, charts, metrics
- **Often used with:** Badge, Avatar, Card, Stat, Icon, Chart

---

## `status-dot`

- **Label:** Status Dot
- **Family:** data_display
- **Category:** Display
- **Intent:** Indicate user presence or availability
- **Description:** Status indicator dot
- **Composable:** no
- **Tags:** status-dot, status, indicator, online, offline, active
- **Use when:** presenting data, lists, tables, charts, metrics
- **Often used with:** Badge, Avatar, Card, Stat, Icon, Chart

---

## `switch`

- **Label:** Switch
- **Family:** input
- **Category:** Form
- **Intent:** Toggle between on and off states
- **Description:** Toggle switch on off
- **Composable:** no
- **Capabilities:** Disabled
- **Tags:** switch, toggle, on, off, activate
- **Use when:** collecting user input, forms, search
- **Often used with:** Field, Label, Input, Select, Checkbox, Switch, Textarea

---

## `table`

- **Label:** Table
- **Family:** data_display
- **Category:** Data
- **Intent:** Display tabular data
- **Description:** HTML table component
- **Composable:** yes
- **Capabilities:** Selected
- **Required parts:** TableHeader, TableBody, TableRow, TableHead, TableCell
- **Optional parts:** TableFooter, TableCaption, TableWrapper
- **Tags:** table, tabular, rows, columns, data
- **Use when:** presenting data, lists, tables, charts, metrics
- **Often used with:** DataTable, DataTableColumn, Pagination, EmptyTable

---

## `table-of-contents`

- **Label:** Table of Contents
- **Family:** utility
- **Category:** Navigation
- **Intent:** Navigate document sections via anchors
- **Description:** Document table of contents
- **Composable:** no
- **Tags:** table-of-contents, index, summary, anchors, navigation, document
- **Use when:** helper components, wrappers, decorators
- **Often used with:** Breadcrumb, Pagination, Tabs, Sidebar, NavItem

---

## `tabs`

- **Label:** Tabs
- **Family:** navigation
- **Category:** Navigation
- **Intent:** Switch between related content panels
- **Description:** Tabbed navigation
- **Composable:** yes
- **Capabilities:** Active
- **Required parts:** TabsList, TabsTrigger, TabsContent
- **Tags:** tabs, navigation, tab, sections, panels
- **Use when:** routing, menus, breadcrumbs, pagination
- **Often used with:** Breadcrumb, Pagination, Tabs, Sidebar, NavItem

---

## `textarea`

- **Label:** Textarea
- **Family:** input
- **Category:** Form
- **Intent:** Capture multi-line text from user
- **Description:** Multi-line text input
- **Composable:** no
- **Capabilities:** Value, Disabled
- **Tags:** textarea, multiline, comment, description, area
- **Use when:** collecting user input, forms, search
- **Often used with:** Field, Label, Input, Select, Checkbox, Switch, Textarea

---

## `toast`

- **Label:** Toast
- **Family:** feedback
- **Category:** Feedback
- **Intent:** Show brief non-blocking notifications
- **Description:** Toast notification message
- **Composable:** yes
- **Capabilities:** OpenClose
- **Required parts:** ToastViewport
- **Optional parts:** ToastTitle, ToastDescription, ToastAction, ToastClose
- **Tags:** toast, notification, snackbar, message, success, error
- **Use when:** showing status, alerts, notifications, loading states
- **Often used with:** Toast, Alert, Banner, Spinner, Skeleton, Progress

---

## `toggle`

- **Label:** Toggle
- **Family:** input
- **Category:** Form
- **Intent:** Toggle a pressed state
- **Description:** Toggle button
- **Composable:** no
- **Capabilities:** Pressed, Disabled
- **Tags:** toggle, activate, button, on, off, press
- **Use when:** collecting user input, forms, search
- **Often used with:** Field, Label, Input, Select, Checkbox, Switch, Textarea

---

## `toggle-group`

- **Label:** Toggle Group
- **Family:** interactive
- **Category:** Action
- **Intent:** Group of toggle buttons with single or multiple selection
- **Description:** Group of toggle buttons
- **Composable:** yes
- **Capabilities:** Multiple, Disabled
- **Tags:** toggle-group, toggle, group, buttons, options, selection
- **Use when:** triggering actions, user input, form submission
- **Often used with:** Button, IconButton, ButtonGroup, DropdownMenu

---

## `toolbar`

- **Label:** Toolbar
- **Family:** layout
- **Category:** Layout
- **Intent:** Action toolbar region
- **Description:** Action toolbar component
- **Composable:** yes
- **Tags:** toolbar, bar, actions, tools, editor, formatting
- **Use when:** structuring page content, containers, grids
- **Often used with:** Card, Container, Grid, Stack, Columns

---

## `tooltip`

- **Label:** Tooltip
- **Family:** overlay
- **Category:** Overlay
- **Intent:** Show brief label on hover/focus
- **Description:** Hover tooltip
- **Composable:** yes
- **Capabilities:** OpenClose
- **Required parts:** TooltipTrigger, TooltipContent
- **Optional parts:** TooltipProvider
- **Tags:** tooltip, hint, hover, info, help
- **Use when:** modal dialogs, drawers, popovers, tooltips
- **Often used with:** Dialog, Drawer, Sheet, Modal, Popover, Tooltip

---

## `tree`

- **Label:** Tree
- **Family:** data_display
- **Category:** Display
- **Intent:** Display hierarchical data
- **Description:** Tree view component
- **Composable:** yes
- **Capabilities:** Selected
- **Required parts:** TreeItem
- **Optional parts:** TreeGroup
- **Tags:** tree, hierarchy, nodes, structure, explorer
- **Use when:** presenting data, lists, tables, charts, metrics
- **Often used with:** Badge, Avatar, Card, Stat, Icon, Chart

---

## `virtual-list`

- **Label:** Virtual List
- **Family:** data_display
- **Category:** Display
- **Intent:** Efficiently render large lists
- **Description:** Virtualized list for large datasets
- **Composable:** no
- **Capabilities:** VirtualScroll
- **Tags:** virtual-list, virtualization, performance, large-volume
- **Use when:** presenting data, lists, tables, charts, metrics
- **Often used with:** Badge, Avatar, Card, Stat, Icon, Chart

---

