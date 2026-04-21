# CanonRS — LLM Context

> AUTO-GENERATED — do not edit manually

---

# Critical Syntax Rules

## Slot Syntax — Layouts vs Blocks

Layout slots use `leptos::children::ToChildren::to_children()` — NOT `slot!()`:

```rust
// LAYOUTS — ToChildren::to_children()
content=leptos::children::ToChildren::to_children(|| view! { <MyComponent/> })

// BLOCKS — slot!() macro
body=slot!(|| view! { <MyComponent/> }.into_any())
```

## Import Paths

```rust
// Layout primitives — from canonrs:: root
use canonrs::{StackPrimitive, StackGap, FlexPrimitive, FlexJustify, FlexAlign, GridPrimitive, GridCols, GridGap, SpacerPrimitive};

// Container — exception
use canonrs::layout::container::{ContainerPrimitive, ContainerSize};

// Top-level enums
use canonrs::{BadgeVariant, ActivityState, InlineNoticeVariant, ToggleState};
```

## BadgeVariant values

Default, Primary, Success, Warning, Destructive, Outline
DOES NOT EXIST: Secondary — use Outline

## Type Inference — E0282

Complex layout slots cause E0282. Always extract to #[component]:

```rust
#[component] fn MyContent() -> impl IntoView { view! { <Stack>...</Stack> } }
content=leptos::children::ToChildren::to_children(|| view! { <MyContent/> })
```

## Primitive nesting — always wrap UI in Stack/Grid/Flex inside block slots

```rust
// CORRECT
body=slot!(|| view! { <StackPrimitive gap=StackGap::Md><Alert .../><Progress .../></StackPrimitive> }.into_any())
// WRONG — UI directly in slot without primitive
body=slot!(|| view! { <Alert .../><Progress .../> }.into_any())
```

---

# CanonRS — UI Components

> AUTO-GENERATED — do not edit manually

---

## `accordion`

- **Label:** Accordion
- **Category:** Navigation
- **Intent:** Expand and collapse content sections
- **Description:** Expandable accordion sections
- **Pain:** Accordion allows invalid multi-open logic without enforced selection mode
- **Promise:** Selection mode enforced via type, preventing invalid open states
- **Why:** AccordionSelection defines whether single or multiple items can be open. The primitive encodes state and behavior in data-rs attributes, ensuring consistent disclosure logic. VisibilityState guarantees SSR-safe open/closed state without runtime drift.
- **Rules:** CR-001, CR-004
- **Use cases:** faq sections, settings panels
- **Related:** collapsible
- **Capabilities:** OpenClose, Multiple
- **Required parts:** AccordionItem, AccordionTrigger, AccordionContent
- **States:** open, closed, disabled
- **Boundary type:** interaction
- **Import:** `use canonrs::ui::accordion::Accordion;`

### Migration

```rust
// ❌ Typical
view! {
  <div class="accordion">
    <button on:click=move |_| toggle(1)>"Item 1"</button>
    {if open == 1 { view! { <div>"Content"</div> } } else { view! {} }}
  </div>
}
```

```rust
// ✅ CanonRS
view! {
  <Accordion selection=AccordionSelection::Single>
    <AccordionItem>
      <AccordionTrigger>"Item 1"</AccordionTrigger>
      <AccordionContent>"Content"</AccordionContent>
    </AccordionItem>
  </Accordion>
}
```


---

## `alert`

- **Label:** Alert
- **Category:** Feedback
- **Intent:** Display important static messages
- **Description:** Alert message box
- **Pain:** Alerts use wrong ARIA roles causing accessibility issues silently
- **Promise:** Semantic state drives ARIA role and live region — variant is visual only
- **Why:** Semantic state (error/warning/success) is derived from variant at the primitive level and drives role and aria-live. Visual styling via data-rs-variant is separate from semantic contract. Accessibility is guaranteed at compile-time.
- **Rules:** CR-001, CR-004
- **Use cases:** error messages, status notifications
- **Related:** toast, banner, callout, inline_notice, status_dot
- **Optional parts:** AlertTitle, AlertDescription, AlertCloseButton
- **States:** open, closed
- **Boundary type:** init
- **Import:** `use canonrs::ui::alert::Alert;`

### Migration

```rust
// ❌ Typical
view! {
  <div class="alert alert-error">"Error occurred"</div>
}
```

```rust
// ✅ CanonRS
view! {
  <Alert variant=AlertVariant::Destructive>
    <AlertTitle>"Error"</AlertTitle>
    <AlertDescription>"Error occurred"</AlertDescription>
  </Alert>
}
```


---

## `alert-dialog`

- **Label:** Alert Dialog
- **Category:** Overlay
- **Intent:** Confirm destructive actions with user
- **Description:** Alert dialog for critical confirmations
- **Pain:** Destructive dialogs lack proper role and accessibility enforcement
- **Promise:** Alertdialog role and accessibility guaranteed by component contract
- **Why:** AlertDialog reuses Dialog but enforces role="alertdialog" and assertive aria-live. The specialized content primitive ensures critical actions are announced correctly. This prevents misuse of generic dialogs for destructive flows.
- **Rules:** CR-001, CR-004
- **Use cases:** delete confirmation, critical actions
- **Related:** dialog, drawer, sheet, modal, confirm_dialog, tooltip, hover_card, popover
- **Capabilities:** OpenClose, FocusTrap, AriaModal
- **Required parts:** AlertDialogContent, AlertDialogTitle
- **Optional parts:** AlertDialogOverlay, AlertDialogDescription, AlertDialogAction
- **States:** open, closed
- **Boundary type:** interaction
- **Import:** `use canonrs::ui::alert_dialog::AlertDialog;`

### Migration

```rust
// ❌ Typical
view! {
  <div class="modal">
    <p>"Delete account?"</p>
    <button>"Confirm"</button>
  </div>
}
```

```rust
// ✅ CanonRS
view! {
  <AlertDialog>
    <AlertDialogTrigger>"Delete"</AlertDialogTrigger>
    <AlertDialogPortal>
      <AlertDialogContent>
        <AlertDialogTitle>"Confirm"</AlertDialogTitle>
        <AlertDialogDescription>"This cannot be undone"</AlertDialogDescription>
      </AlertDialogContent>
    </AlertDialogPortal>
  </AlertDialog>
}
```


---

## `animate`

- **Label:** Animate
- **Category:** Display
- **Intent:** Apply CSS animations to children
- **Description:** Animation wrapper component
- **Pain:** Animations rely on fragile class names and inconsistent timing values
- **Promise:** Animation type and easing enforced through typed enums
- **Why:** AnimationName and AnimationEasing define allowed motion patterns. The primitive encodes animation parameters into data-rs attributes, avoiding class-based drift. This ensures consistent animation behavior across SSR and client.
- **Rules:** CR-001, CR-004
- **Use cases:** page transitions, modal animations
- **Related:** empty_state, error_state
- **States:** active, inactive
- **Boundary type:** init
- **Import:** `use canonrs::ui::animate::Animate;`

### Migration

```rust
// ❌ Typical
view! {
  <div class="fade-in ease-in-out duration-300">"Content"</div>
}
```

```rust
// ✅ CanonRS
view! {
  <Animate animation=AnimationName::FadeIn duration="300ms">
    "Content"
  </Animate>
}
```


---

## `aspect-ratio`

- **Label:** Aspect Ratio
- **Category:** Display
- **Intent:** Maintain consistent width/height ratio
- **Description:** Aspect ratio container
- **Pain:** Aspect ratios break on resize causing layout shift and inconsistent rendering
- **Promise:** Aspect ratio enforced structurally with no layout drift
- **Why:** AspectRatioPrimitive encodes width/height ratio in data attributes. The structure guarantees consistent layout regardless of content size. This eliminates runtime calculations and ensures SSR-safe rendering.
- **Rules:** CR-001, CR-004
- **Use cases:** video containers, image previews
- **Related:** card, resizable, scroll_area, page_header, toolbar, separator
- **Boundary type:** passthrough
- **Import:** `use canonrs::ui::aspect_ratio::AspectRatio;`

### Migration

```rust
// ❌ Typical
view! {
  <div style="position:relative;padding-top:56.25%">
    <img src="img.png" style="position:absolute;width:100%;height:100%" />
  </div>
}
```

```rust
// ✅ CanonRS
view! {
  <AspectRatio ratio_w=16.0 ratio_h=9.0>
    <img src="img.png" />
  </AspectRatio>
}
```


---

## `avatar`

- **Label:** Avatar
- **Category:** Display
- **Intent:** Display user profile image with fallback
- **Description:** User avatar image
- **Pain:** Avatar fallback logic breaks when image fails to load
- **Promise:** Image and fallback visibility controlled by state system
- **Why:** AvatarImage and AvatarFallback use VisibilityState to control rendering. The system ensures fallback is shown when image is unavailable. This avoids manual conditional logic and guarantees consistent behavior.
- **Rules:** CR-001, CR-004
- **Use cases:** user profile, team lists
- **Related:** icon, logo, code_block, markdown, chart, stat, inline_meta, kbd, badge, carousel
- **Optional parts:** AvatarImage, AvatarFallback
- **States:** loading, error
- **Boundary type:** init
- **Import:** `use canonrs::ui::avatar::Avatar;`

### Migration

```rust
// ❌ Typical
view! {
  <img src="user.png" on:error=move |_| show_fallback() />
}
```

```rust
// ✅ CanonRS
view! {
  <Avatar>
    <AvatarImage src="user.png" alt="User" />
    <AvatarFallback>"AB"</AvatarFallback>
  </Avatar>
}
```


---

## `badge`

- **Label:** Badge
- **Category:** Display
- **Intent:** Display status, count or label
- **Description:** Status badge label
- **Pain:** Badges mix interactive and static behavior without clear intent
- **Promise:** Interactivity explicitly defined and enforced by type
- **Why:** BadgeInteractivity defines whether the badge is static or interactive. The primitive encodes this into data attributes, preventing misuse. This ensures consistent semantics and avoids accidental clickable badges.
- **Rules:** CR-001, CR-004
- **Use cases:** status labels, notifications
- **Related:** avatar, icon, logo, code_block, markdown, chart, stat, inline_meta, kbd, carousel
- **Boundary type:** passthrough
- **Import:** `use canonrs::ui::badge::Badge;`

### Migration

```rust
// ❌ Typical
view! {
  <span class="badge clickable">"New"</span>
}
```

```rust
// ✅ CanonRS
view! {
  <Badge interactivity=BadgeInteractivity::Static>
    "New"
  </Badge>
}
```


---

## `banner`

- **Label:** Banner
- **Category:** Feedback
- **Intent:** Display persistent page-level messages
- **Description:** Banner message
- **Pain:** Banner messages lack consistent visibility and accessibility behavior
- **Promise:** Visibility and ARIA behavior enforced by state and variant
- **Why:** BannerVariant controls semantic role and aria-live behavior. VisibilityState ensures correct open/hidden state without runtime logic. This guarantees accessible, consistent page-level messaging.
- **Rules:** CR-001, CR-004
- **Use cases:** system announcements, warnings
- **Related:** toast, alert, callout, inline_notice, status_dot
- **Capabilities:** OpenClose
- **Optional parts:** BannerContent, BannerActions, BannerClose
- **States:** open, closed
- **Boundary type:** init
- **Import:** `use canonrs::ui::banner::Banner;`

### Migration

```rust
// ❌ Typical
view! {
  <div class="banner">"Maintenance scheduled"</div>
}
```

```rust
// ✅ CanonRS
view! {
  <Banner>
    <BannerContent>"Maintenance scheduled"</BannerContent>
  </Banner>
}
```


---

## `breadcrumb`

- **Label:** Breadcrumb
- **Category:** Navigation
- **Intent:** Show current location in hierarchy
- **Description:** Navigation breadcrumb trail
- **Pain:** Breadcrumbs fail to mark current page correctly for accessibility
- **Promise:** Current page state enforced via activity state mapping
- **Why:** ActivityState defines whether a breadcrumb link is active. The primitive maps this to aria-current automatically. This ensures correct navigation semantics without manual ARIA handling.
- **Rules:** CR-001, CR-004
- **Use cases:** navigation trails, hierarchy display
- **Related:** navigation_menu, sidebar, nav_item, pagination, link_group
- **Optional parts:** BreadcrumbItem, BreadcrumbLink, BreadcrumbSeparator
- **States:** active
- **Boundary type:** interaction
- **Import:** `use canonrs::ui::breadcrumb::Breadcrumb;`

### Migration

```rust
// ❌ Typical
view! {
  <nav>
    <a href="#">Home</a>
    <span>"/"</span>
    <a class="active">Page</a>
  </nav>
}
```

```rust
// ✅ CanonRS
view! {
  <Breadcrumb>
    <BreadcrumbItem>
      <BreadcrumbLink href="#">"Home"</BreadcrumbLink>
    </BreadcrumbItem>
    <BreadcrumbSeparator>"/"</BreadcrumbSeparator>
    <BreadcrumbItem>
      <BreadcrumbPage>"Page"</BreadcrumbPage>
    </BreadcrumbItem>
  </Breadcrumb>
}
```


---

## `button`

- **Label:** Button
- **Category:** Action
- **Intent:** Trigger an action or event
- **Description:** Action button with variant and size
- **Keywords:** button rust leptos, ssr safe button leptos, canonical button component rust, governed ui button
- **Pain:** Buttons rely on string classes causing inconsistent variants and states
- **Promise:** Variant and size enforced at compile-time via enums
- **Why:** ButtonVariant and ButtonSize define allowed visual and behavioral states. The primitive encodes these into data attributes, ensuring consistent rendering. This eliminates invalid combinations and style drift.
- **Rules:** CR-001, CR-004, CR-148
- **Use cases:** form submit, cta actions
- **Related:** button_group, icon_button, copy_button, link
- **Capabilities:** Disabled
- **States:** hover, active, focus, disabled
- **Boundary type:** init
- **Import:** `use canonrs::ui::button::Button;`

### Migration

```rust
// ❌ Typical
view! {
  <button class="btn btn-primary btn-md">"Submit"</button>
}
```

```rust
// ✅ CanonRS
view! {
  <Button variant=ButtonVariant::Primary size=ButtonSize::Md>
    "Submit"
  </Button>
}
```


---

## `button-group`

- **Label:** Button Group
- **Category:** Action
- **Intent:** Group related action buttons together
- **Description:** Group of action buttons
- **Pain:** Grouped buttons lose semantic grouping and accessibility context
- **Promise:** Group semantics and attachment enforced via component contract
- **Why:** ButtonGroupPrimitive defines role="group" and controlled attachment state via ToggleState. This ensures grouped actions are treated as a single logical unit. The contract guarantees consistent accessibility and visual cohesion.
- **Rules:** CR-001, CR-004
- **Use cases:** toolbar actions, segmented controls
- **Related:** button, icon_button, copy_button, link
- **States:** first, last
- **Boundary type:** passthrough
- **Import:** `use canonrs::ui::button_group::ButtonGroup;`

### Migration

```rust
// ❌ Typical
view! {
  <div class="btn-group">
    <button>"Left"</button>
    <button>"Right"</button>
  </div>
}
```

```rust
// ✅ CanonRS
view! {
  <ButtonGroup>
    <Button>"Left"</Button>
    <Button>"Right"</Button>
  </ButtonGroup>
}
```


---

## `callout`

- **Label:** Callout
- **Category:** Feedback
- **Intent:** Highlight important contextual information
- **Description:** Callout info box
- **Pain:** Callouts use inconsistent roles and lack semantic intent
- **Promise:** Semantic role and urgency enforced via variant
- **Why:** CalloutVariant determines role and aria-live behavior. The primitive encodes these semantics directly into the DOM. This guarantees consistent accessibility and meaning across all callouts.
- **Rules:** CR-001, CR-004
- **Use cases:** tips, warnings
- **Related:** toast, alert, banner, inline_notice, status_dot
- **Optional parts:** CalloutIcon, CalloutTitle, CalloutDescription
- **Boundary type:** passthrough
- **Import:** `use canonrs::ui::callout::Callout;`

### Migration

```rust
// ❌ Typical
view! {
  <div class="callout warning">"Be careful"</div>
}
```

```rust
// ✅ CanonRS
view! {
  <Callout variant=CalloutVariant::Warning>
    <CalloutTitle>"Warning"</CalloutTitle>
    <CalloutDescription>"Be careful"</CalloutDescription>
  </Callout>
}
```


---

## `card`

- **Label:** Card
- **Category:** Display
- **Intent:** Group related content in a container
- **Description:** Card component
- **Pain:** Content containers lack consistent structure and semantic regions
- **Promise:** Card structure enforced with defined regions and roles
- **Why:** CardPrimitive enforces a semantic region with structured subcomponents like header and content. This guarantees consistent layout composition. The contract prevents ad-hoc container misuse.
- **Rules:** CR-001, CR-004
- **Use cases:** dashboards, content grouping
- **Related:** resizable, scroll_area, aspect_ratio, page_header, toolbar, separator
- **Optional parts:** CardHeader, CardTitle, CardDescription, CardContent, CardFooter
- **Boundary type:** passthrough
- **Import:** `use canonrs::ui::card::Card;`

### Migration

```rust
// ❌ Typical
view! {
  <div class="card">
    <h3>"Title"</h3>
    <p>"Content"</p>
  </div>
}
```

```rust
// ✅ CanonRS
view! {
  <Card>
    <CardHeader>
      <CardTitle>"Title"</CardTitle>
    </CardHeader>
    <CardContent>"Content"</CardContent>
  </Card>
}
```


---

## `carousel`

- **Label:** Carousel
- **Category:** Display
- **Intent:** Cycle through items horizontally
- **Description:** Image carousel slider
- **Pain:** Carousels break accessibility and state synchronization across slides
- **Promise:** Slide state and navigation semantics enforced via structured primitives
- **Why:** CarouselPrimitive defines roles and slide semantics including aria labels and state. ActivityState and VisibilityState control active and hidden slides. This ensures accessibility and predictable slideshow behavior.
- **Rules:** CR-001, CR-004
- **Use cases:** image galleries, feature sliders
- **Related:** avatar, icon, logo, code_block, markdown, chart, stat, inline_meta, kbd, badge
- **Capabilities:** KeyboardArrows
- **Required parts:** CarouselTrack, CarouselItem
- **Optional parts:** CarouselPrev, CarouselNext, CarouselIndicators
- **States:** active, inactive
- **Boundary type:** interaction
- **Import:** `use canonrs::ui::carousel::Carousel;`

### Migration

```rust
// ❌ Typical
view! {
  <div class="carousel">
    <div class="slide active">"Slide 1"</div>
  </div>
}
```

```rust
// ✅ CanonRS
view! {
  <Carousel>
    <CarouselTrack>
      <CarouselItem>"Slide 1"</CarouselItem>
    </CarouselTrack>
  </Carousel>
}
```


---

## `chart`

- **Label:** Chart
- **Category:** Display
- **Intent:** Visualize data graphically
- **Description:** Data chart visualization
- **Pain:** Charts mix rendering logic and data, causing inconsistent behavior
- **Promise:** Chart structure and data binding enforced via contract
- **Why:** ChartPrimitive separates rendering (canvas) from data (data-rs attributes). ChartType enforces visualization type at compile-time. This guarantees consistent rendering and interaction patterns.
- **Rules:** CR-001, CR-004
- **Use cases:** analytics dashboards, data visualization
- **Related:** avatar, icon, logo, code_block, markdown, stat, inline_meta, kbd, badge, carousel
- **States:** loading, error
- **Boundary type:** interaction
- **Import:** `use canonrs::ui::chart::Chart;`

### Migration

```rust
// ❌ Typical
view! {
  <canvas id="chart"></canvas>
  <script>renderChart(data)</script>
}
```

```rust
// ✅ CanonRS
view! {
  <Chart data=data chart_type=ChartType::Line />
}
```


---

## `checkbox`

- **Label:** Checkbox
- **Category:** Form
- **Intent:** Toggle a boolean value
- **Description:** Checkbox input
- **Pain:** Checkbox state desyncs between UI and internal state
- **Promise:** Selection state (checked/unchecked/indeterminate) drives visual and aria-checked contract
- **Why:** CheckboxPrimitive maps ActivityState to checked and ARIA attributes. This ensures UI and accessibility stay in sync. The contract prevents mismatched boolean and DOM state.
- **Rules:** CR-001, CR-004
- **Use cases:** forms, multi-selection
- **Related:** form, input, input_group, input_otp, textarea, field, label, form_error_summary
- **Capabilities:** Disabled
- **States:** active, focus, disabled
- **Boundary type:** init
- **Import:** `use canonrs::ui::checkbox::Checkbox;`

### Migration

```rust
// ❌ Typical
view! {
  <input type="checkbox" checked=state />
}
```

```rust
// ✅ CanonRS
view! {
  <Checkbox checked=true>"Remember me"</Checkbox>
}
```


---

## `code-block`

- **Label:** Code Block
- **Category:** Display
- **Intent:** Display syntax-highlighted code
- **Description:** Syntax-highlighted code display
- **Pain:** Code blocks rely on client-side highlighting causing hydration mismatch
- **Promise:** SSR-safe syntax highlighting with deterministic DOM output
- **Why:** CodeBlockPrimitive supports SSR-safe rendering with precomputed HTML. The contract ensures no client mutation is required. This prevents hydration mismatch and ensures consistent output.
- **Rules:** CR-001, CR-004
- **Use cases:** docs, code snippets
- **Related:** avatar, icon, logo, markdown, chart, stat, inline_meta, kbd, badge, carousel
- **States:** copied
- **Boundary type:** passthrough
- **Import:** `use canonrs::ui::code_block::CodeBlock;`

### Migration

```rust
// ❌ Typical
view! {
  <pre><code>{highlight(code)}</code></pre>
}
```

```rust
// ✅ CanonRS
view! {
  <CodeBlock code="fn main() {}" language="rust" />
}
```


---

## `collapsible`

- **Label:** Collapsible
- **Category:** Navigation
- **Intent:** Show and hide content sections
- **Description:** Collapsible section
- **Pain:** Collapsible sections lose sync between trigger and content state
- **Promise:** Trigger and content state always synchronized via shared visibility state
- **Why:** CollapsiblePrimitive shares VisibilityState across trigger and content. This ensures both reflect the same open/closed state. The contract eliminates manual sync logic.
- **Rules:** CR-001, CR-004
- **Use cases:** faq sections, expandable panels
- **Related:** accordion
- **Capabilities:** OpenClose
- **Required parts:** CollapsibleTrigger, CollapsibleContent
- **States:** open, closed
- **Boundary type:** init
- **Import:** `use canonrs::ui::collapsible::Collapsible;`

### Migration

```rust
// ❌ Typical
view! {
  <button on:click=toggle>"Toggle"</button>
  {if open { view! { <div>"Content"</div> } }}
}
```

```rust
// ✅ CanonRS
view! {
  <Collapsible>
    <CollapsibleTrigger>"Toggle"</CollapsibleTrigger>
    <CollapsibleContent>"Content"</CollapsibleContent>
  </Collapsible>
}
```


---

## `color-picker`

- **Label:** Color Picker
- **Category:** Form
- **Intent:** Select a color value
- **Description:** Color picker input
- **Pain:** Color inputs lack consistent selection and accessibility behavior
- **Promise:** Color selection and state enforced via structured primitives
- **Why:** ColorPickerPrimitive uses SelectionState and VisibilityState for interaction control. The contract ensures consistent value handling and accessibility. This prevents ad-hoc color input implementations.
- **Rules:** CR-001, CR-004
- **Use cases:** theme customization, design tools
- **Related:** select, combobox, radio, radio_group, slider
- **Capabilities:** Value, Disabled
- **Optional parts:** ColorPickerSwatch, ColorPickerInput
- **States:** selected, disabled
- **Boundary type:** interaction
- **Import:** `use canonrs::ui::color_picker::ColorPicker;`

### Migration

```rust
// ❌ Typical
view! {
  <input type="color" value="#000" />
}
```

```rust
// ✅ CanonRS
view! {
  <ColorPicker value="#000000" />
}
```


---

## `combobox`

- **Label:** Combobox
- **Category:** Form
- **Intent:** Search and select from a list
- **Description:** Searchable combo box
- **Pain:** Searchable selects break ARIA roles and keyboard navigation
- **Promise:** Combobox roles and interaction fully enforced by structure
- **Why:** ComboboxPrimitive defines proper ARIA roles and input behavior. SelectionState and VisibilityState control dropdown interaction. This guarantees accessible and predictable combobox behavior.
- **Rules:** CR-001, CR-004
- **Use cases:** autocomplete, search dropdown
- **Related:** select, radio, radio_group, color_picker, slider
- **Capabilities:** OpenClose, Disabled
- **Required parts:** ComboboxTrigger, ComboboxList, ComboboxItem
- **States:** open, closed, selected, disabled
- **Boundary type:** interaction
- **Import:** `use canonrs::ui::combobox::Combobox;`

### Migration

```rust
// ❌ Typical
view! {
  <input type="text" />
  <ul><li>"Option"</li></ul>
}
```

```rust
// ✅ CanonRS
view! {
  <Combobox>
    <ComboboxTrigger>"Select..."</ComboboxTrigger>
    <ComboboxList>
      <ComboboxItem value="1">"Option"</ComboboxItem>
    </ComboboxList>
  </Combobox>
}
```


---

## `command`

- **Label:** Command
- **Category:** Action
- **Intent:** Command palette for quick actions
- **Description:** Command palette
- **Pain:** Command palette lacks keyboard navigation and ARIA consistency
- **Promise:** Command palette semantics and selection fully enforced
- **Why:** CommandPrimitive enforces listbox semantics with structured input, grouping and items. SelectionState and ActivityState control highlight and selection behavior. This guarantees consistent keyboard navigation and accessibility.
- **Rules:** CR-001, CR-004
- **Use cases:** command palette, quick navigation
- **Related:** dropdown_menu, context_menu, menubar, menu
- **Capabilities:** OpenClose, Typeahead
- **Required parts:** CommandInput, CommandList
- **Optional parts:** CommandItem, CommandGroup, CommandSeparator, CommandEmpty
- **States:** active
- **Boundary type:** init
- **Import:** `use canonrs::ui::command::Command;`

### Migration

```rust
// ❌ Typical
view! {
  <input placeholder="Search" />
  <div class="list">
    <div>"Item"</div>
  </div>
}
```

```rust
// ✅ CanonRS
view! {
  <Command>
    <CommandInput placeholder="Search..." />
    <CommandList>
      <CommandItem>"Item"</CommandItem>
    </CommandList>
  </Command>
}
```


---

## `confirm-dialog`

- **Label:** Confirm Dialog
- **Category:** Overlay
- **Intent:** Ask user to confirm an action
- **Description:** Confirmation dialog
- **Pain:** Confirmation dialogs miss ARIA roles and destructive intent signaling
- **Promise:** Confirmation semantics and intent enforced via variant and structure
- **Why:** ConfirmDialogPrimitive enforces role="alertdialog" with variant-driven semantics. VisibilityState controls lifecycle and accessibility attributes. This guarantees proper focus and urgency communication.
- **Rules:** CR-001, CR-004
- **Use cases:** delete confirmation, critical actions
- **Related:** dialog, alert_dialog, drawer, sheet, modal, tooltip, hover_card, popover
- **Capabilities:** OpenClose, FocusTrap, AriaModal
- **States:** open, closed
- **Boundary type:** interaction
- **Import:** `use canonrs::ui::confirm_dialog::ConfirmDialog;`

### Migration

```rust
// ❌ Typical
view! {
  <div class="modal">"Are you sure?"</div>
}
```

```rust
// ✅ CanonRS
view! {
  <ConfirmDialog />
}
```


---

## `context-menu`

- **Label:** Context Menu
- **Category:** Action
- **Intent:** Show menu on right-click
- **Description:** Right-click context menu
- **Pain:** Right-click menus lack consistent trigger and focus behavior
- **Promise:** Context menu interaction and roles enforced structurally
- **Why:** ContextMenuPrimitive defines trigger/content separation with ARIA roles. ActivityState and DisabledState ensure correct focus and navigation. This guarantees predictable contextual actions.
- **Rules:** CR-001, CR-004
- **Use cases:** file actions, contextual tools
- **Related:** dropdown_menu, menubar, menu, command
- **Capabilities:** OpenClose
- **Required parts:** ContextMenuTrigger, ContextMenuContent
- **Optional parts:** ContextMenuItem, ContextMenuSeparator
- **States:** open, closed
- **Boundary type:** interaction
- **Import:** `use canonrs::ui::context_menu::ContextMenu;`

### Migration

```rust
// ❌ Typical
view! {
  <div on:contextmenu=show_menu>
    <div class="menu">"Item"</div>
  </div>
}
```

```rust
// ✅ CanonRS
view! {
  <ContextMenu>
    <ContextMenuTrigger>
      <span>"Right-click"</span>
    </ContextMenuTrigger>
    <ContextMenuContent>
      <ContextMenuItem>"Item"</ContextMenuItem>
    </ContextMenuContent>
  </ContextMenu>
}
```


---

## `copy-button`

- **Label:** Copy Button
- **Category:** Action
- **Intent:** Copy text to clipboard on click
- **Description:** Clipboard copy button
- **Pain:** Copy buttons require manual state handling and feedback UI
- **Promise:** Copy state lifecycle fully encoded in DOM state machine
- **Why:** CopyButton encodes idle, copied, and error states via data-rs-state. Behavior layer handles transitions without JS wiring. This guarantees consistent feedback and eliminates manual state management.
- **Rules:** CR-001, CR-004
- **Use cases:** code snippets, share links
- **Related:** button, button_group, icon_button, link
- **States:** hover, copied, error
- **Boundary type:** interaction
- **Import:** `use canonrs::ui::copy_button::CopyButton;`

### Migration

```rust
// ❌ Typical
view! {
  <button on:click=copy>"Copy"</button>
}
```

```rust
// ✅ CanonRS
view! {
  <CopyButton id="copy" text="value" />
}
```


---

## `data-table`

- **Label:** Data Table
- **Category:** Data
- **Intent:** Display sortable, filterable tabular data
- **Description:** Sortable data table component
- **Pain:** Tables mix rendering, sorting and state with no structure
- **Promise:** Table structure and sorting semantics enforced at component level
- **Why:** DataTablePrimitive separates head, body and cells with explicit sort metadata. SelectionState and density are encoded as attributes. This guarantees consistent behavior across all tables.
- **Rules:** CR-001, CR-004
- **Use cases:** admin dashboards, data grids
- **Related:** table, virtual_list, empty_table, tree, list_item
- **Capabilities:** Multiple
- **Optional parts:** DataTablePagination, DataTableToolbar, DataTableColumn
- **States:** loading, error, selected
- **Boundary type:** interaction
- **Import:** `use canonrs::ui::data_table::DataTable;`

### Migration

```rust
// ❌ Typical
view! {
  <table>
    <tr><td>"A"</td></tr>
  </table>
}
```

```rust
// ✅ CanonRS
view! {
  <DataTableCore data=data columns=columns />
}
```


---

## `dialog`

- **Label:** Dialog
- **Category:** Overlay
- **Intent:** Display critical content requiring user interaction
- **Description:** Modal dialog component
- **Pain:** Dialogs break focus trap and accessibility roles
- **Promise:** Dialog accessibility and lifecycle enforced via primitives
- **Why:** DialogPrimitive defines overlay, portal and content with ARIA compliance. VisibilityState guarantees correct open/close synchronization. This ensures safe modal behavior across SSR and client.
- **Rules:** CR-001, CR-004
- **Use cases:** modals, forms
- **Related:** alert_dialog, drawer, sheet, modal, confirm_dialog, tooltip, hover_card, popover
- **Capabilities:** OpenClose, FocusTrap, KeyboardEsc, AriaModal
- **Required parts:** DialogContent, DialogTitle
- **Optional parts:** DialogOverlay, DialogDescription, DialogClose
- **States:** open, closed
- **Boundary type:** interaction
- **Import:** `use canonrs::ui::dialog::Dialog;`

### Migration

```rust
// ❌ Typical
view! {
  <div class="modal">"Content"</div>
}
```

```rust
// ✅ CanonRS
view! {
  <Dialog>
    <DialogTrigger>"Open"</DialogTrigger>
    <DialogPortal>
      <DialogOverlay />
      <DialogContent>
        <DialogTitle>"Title"</DialogTitle>
      </DialogContent>
    </DialogPortal>
  </Dialog>
}
```


---

## `doc-progress`

- **Label:** Doc Progress
- **Category:** Display
- **Intent:** Indicate reading progress in a document
- **Description:** Document progress indicator
- **Pain:** Scroll progress indicators require manual scroll tracking logic
- **Promise:** Progress tracking injected automatically via behavior layer
- **Why:** DocProgressPrimitive exposes progress via data attributes and ARIA. Portal variant allows injection anywhere in layout. This guarantees consistent scroll tracking without custom JS.
- **Rules:** CR-001, CR-004
- **Use cases:** docs reading, long articles
- **Related:** progress, spinner, skeleton, pulse, loading_overlay
- **States:** active, inactive
- **Boundary type:** init
- **Import:** `use canonrs::ui::doc_progress::DocProgress;`

### Migration

```rust
// ❌ Typical
window.onscroll = () => updateProgress();
```

```rust
// ✅ CanonRS
view! {
  <DocProgress />
}
```


---

## `drawer`

- **Label:** Drawer
- **Category:** Overlay
- **Intent:** Slide-in panel for supplementary content
- **Description:** Slide-out drawer component
- **Pain:** Slide panels lack consistent direction and accessibility semantics
- **Promise:** Drawer direction and visibility enforced via typed contract
- **Why:** DrawerPrimitive encodes side and visibility with DrawerSide and VisibilityState. ARIA attributes ensure accessibility compliance. This guarantees predictable slide behavior.
- **Rules:** CR-001, CR-004
- **Use cases:** mobile navigation, side panels
- **Related:** dialog, alert_dialog, sheet, modal, confirm_dialog, tooltip, hover_card, popover
- **Capabilities:** OpenClose, FocusTrap, KeyboardEsc, AriaModal
- **Required parts:** DrawerContent
- **Optional parts:** DrawerOverlay
- **States:** open, closed
- **Boundary type:** interaction
- **Import:** `use canonrs::ui::drawer::Drawer;`

### Migration

```rust
// ❌ Typical
view! {
  <div class="drawer left">"Content"</div>
}
```

```rust
// ✅ CanonRS
view! {
  <Drawer>
    <DrawerOverlay />
    <DrawerContent aria_labelledby="title" />
  </Drawer>
}
```


---

## `dropdown-menu`

- **Label:** Dropdown Menu
- **Category:** Action
- **Intent:** Show contextual action menu
- **Description:** Dropdown menu
- **Pain:** Dropdown menus break keyboard navigation and selection state
- **Promise:** Menu interaction and state fully encoded via primitives
- **Why:** DropdownMenuPrimitive defines trigger/content with ARIA roles. ToggleState and ActivityState manage selection and highlight. This guarantees consistent menu behavior.
- **Rules:** CR-001, CR-004
- **Use cases:** actions menu, user menu
- **Related:** context_menu, menubar, menu, command
- **Capabilities:** OpenClose, Disabled
- **Required parts:** DropdownMenuTrigger, DropdownMenuContent
- **Optional parts:** DropdownMenuItem, DropdownMenuSeparator, DropdownMenuGroup
- **States:** open, closed
- **Boundary type:** interaction
- **Import:** `use canonrs::ui::dropdown_menu::DropdownMenu;`

### Migration

```rust
// ❌ Typical
view! {
  <div class="dropdown">
    <button>"Open"</button>
    <div class="menu">"Item"</div>
  </div>
}
```

```rust
// ✅ CanonRS
view! {
  <DropdownMenu>
    <DropdownMenuTrigger>"Open"</DropdownMenuTrigger>
    <DropdownMenuContent>
      <DropdownMenuItem>"Item"</DropdownMenuItem>
    </DropdownMenuContent>
  </DropdownMenu>
}
```


---

## `empty-state`

- **Label:** Empty State
- **Category:** Feedback
- **Intent:** Display when no content is available
- **Description:** Empty state placeholder
- **Pain:** Empty states are inconsistent and lack semantic meaning
- **Promise:** Empty state intent and variant enforced via contract
- **Why:** EmptyStatePrimitive encodes variant and ARIA role="status". Variants ensure consistent messaging patterns. This guarantees predictable feedback across all empty states.
- **Rules:** CR-001, CR-004
- **Use cases:** no results, empty dashboards
- **Related:** error_state, animate
- **Optional parts:** EmptyStateIcon, EmptyStateTitle, EmptyStateDescription, EmptyStateAction
- **Boundary type:** passthrough
- **Import:** `use canonrs::ui::empty_state::EmptyState;`

### Migration

```rust
// ❌ Typical
view! {
  <div>"No data"</div>
}
```

```rust
// ✅ CanonRS
view! {
  <EmptyState>
    <EmptyStateTitle>"No data"</EmptyStateTitle>
  </EmptyState>
}
```


---

## `empty-table`

- **Label:** Empty Table
- **Category:** Display
- **Intent:** Show empty state inside a table
- **Description:** Empty table state display
- **Pain:** Empty table rows break layout and screen reader semantics
- **Promise:** Empty state rendered as valid table row with proper ARIA
- **Why:** EmptyTablePrimitive enforces correct table structure using tr and td with colspan. It preserves row semantics while exposing status via aria-live. This guarantees no layout or accessibility break inside tables.
- **Rules:** CR-001, CR-004
- **Use cases:** empty data grids, admin tables
- **Related:** table, data_table, virtual_list, tree, list_item
- **Boundary type:** passthrough
- **Import:** `use canonrs::ui::empty_table::EmptyTable;`

### Migration

```rust
// ❌ Typical
view! {
  <tbody>
    <div>"No data"</div>
  </tbody>
}
```

```rust
// ✅ CanonRS
view! {
  <tbody>
    <EmptyTable colspan=3u32 />
  </tbody>
}
```


---

## `error-state`

- **Label:** Error State
- **Category:** Feedback
- **Intent:** Display error condition to user
- **Description:** Error state display
- **Pain:** Error messages inconsistent and not announced to assistive technologies
- **Promise:** Error feedback always announced and structurally consistent
- **Why:** ErrorStatePrimitive enforces role="status" with aria-live="assertive". This guarantees immediate announcement of critical errors. Structure ensures consistent composition of icon, title and actions.
- **Rules:** CR-001, CR-004
- **Use cases:** api failures, form submission errors
- **Related:** empty_state, animate
- **Optional parts:** ErrorStateIcon, ErrorStateTitle, ErrorStateDescription, ErrorStateAction
- **States:** error
- **Boundary type:** passthrough
- **Import:** `use canonrs::ui::error_state::ErrorState;`

### Migration

```rust
// ❌ Typical
view! {
  <div class="error">"Something went wrong"</div>
}
```

```rust
// ✅ CanonRS
view! {
  <ErrorState>
    <ErrorStateTitle>"Error"</ErrorStateTitle>
  </ErrorState>
}
```


---

## `field`

- **Label:** Field
- **Category:** Form
- **Intent:** Wrap a form input with label and error
- **Description:** Form field wrapper with label and error
- **Pain:** Form fields lose validation and accessibility linkage between label and input
- **Promise:** Validation, label and error state unified in a single contract
- **Why:** FieldPrimitive encodes validation and disabled state into data attributes. FieldLabel and FieldError ensure correct ARIA mapping. This guarantees consistent field behavior and accessibility.
- **Rules:** CR-001, CR-004
- **Use cases:** forms, input validation
- **Related:** form, input, input_group, input_otp, textarea, label, checkbox, form_error_summary
- **States:** focus, disabled, error
- **Boundary type:** init
- **Import:** `use canonrs::ui::field::Field;`

### Migration

```rust
// ❌ Typical
view! {
  <label>"Email"</label>
  <input />
}
```

```rust
// ✅ CanonRS
view! {
  <Field>
    <FieldLabel>"Email"</FieldLabel>
  </Field>
}
```


---

## `form`

- **Label:** Form
- **Category:** Form
- **Intent:** Form container with submit handling
- **Description:** Form component
- **Pain:** Forms mix validation, submission and accessibility inconsistently
- **Promise:** Form lifecycle and validation state enforced at container level
- **Why:** FormPrimitive encodes validation, disabled and submission states into attributes. ARIA states like busy and invalid are derived automatically. This guarantees consistent form behavior across all inputs.
- **Rules:** CR-001, CR-004
- **Use cases:** user forms, checkout flows
- **Related:** input, input_group, input_otp, textarea, field, label, checkbox, form_error_summary
- **States:** loading, error
- **Boundary type:** init
- **Import:** `use canonrs::ui::form::Form;`

### Migration

```rust
// ❌ Typical
view! {
  <form>
    <input />
  </form>
}
```

```rust
// ✅ CanonRS
view! {
  <Form>
    <FormField>
      <FormLabel>"Name"</FormLabel>
    </FormField>
  </Form>
}
```


---

## `form-error-summary`

- **Label:** Form Error Summary
- **Category:** Form
- **Intent:** Summarize form validation errors
- **Description:** Form validation error summary
- **Pain:** Form errors scattered and not announced collectively
- **Promise:** All form errors announced together with structured summary
- **Why:** FormErrorSummaryPrimitive uses role="alert" with aria-atomic to ensure full error announcement. Errors are grouped and structured. This guarantees accessibility and visibility of all validation issues.
- **Rules:** CR-001, CR-004
- **Use cases:** form validation, multi-field errors
- **Related:** form, input, input_group, input_otp, textarea, field, label, checkbox
- **States:** error
- **Boundary type:** passthrough
- **Import:** `use canonrs::ui::form_error_summary::FormErrorSummary;`

### Migration

```rust
// ❌ Typical
view! {
  <div>"Error"</div>
}
```

```rust
// ✅ CanonRS
view! {
  <FormErrorSummary errors=vec![] />
}
```


---

## `hero-ui`

- **Label:** Hero UI
- **Category:** Display
- **Intent:** Semantic UI elements inside HeroBlock
- **Description:** Hero typography and label components
- **Import:** `use canonrs::ui::hero_ui::HeroUi;`

---

## `hover-card`

- **Label:** Hover Card
- **Category:** Overlay
- **Intent:** Show rich preview on hover
- **Description:** Hover card popup
- **Pain:** Hover previews break focus and visibility synchronization
- **Promise:** Hover state and positioning enforced via visibility contract
- **Why:** HoverCardPrimitive uses VisibilityState and side enums to control display. Trigger and content share synchronized state. This guarantees consistent hover behavior and accessibility.
- **Rules:** CR-001, CR-004
- **Use cases:** user previews, tooltips with content
- **Related:** dialog, alert_dialog, drawer, sheet, modal, confirm_dialog, tooltip, popover
- **Capabilities:** OpenClose
- **Required parts:** HoverCardTrigger, HoverCardContent
- **States:** open, closed
- **Boundary type:** interaction
- **Import:** `use canonrs::ui::hover_card::HoverCard;`

### Migration

```rust
// ❌ Typical
view! {
  <div on:mouseover=show>"Hover"</div>
}
```

```rust
// ✅ CanonRS
view! {
  <HoverCard>
    <HoverCardTrigger>"Hover"</HoverCardTrigger>
    <HoverCardContent>"Content"</HoverCardContent>
  </HoverCard>
}
```


---

## `icon`

- **Label:** Icon
- **Category:** Display
- **Intent:** Display an SVG icon
- **Description:** SVG icon display
- **Pain:** Icons lack consistent sizing and variant handling
- **Promise:** Icon size and variant enforced via typed enums
- **Why:** Icon component encodes size and variant through enums. These map directly to data attributes. This guarantees consistent rendering and eliminates class-based inconsistencies.
- **Rules:** CR-001, CR-004
- **Use cases:** buttons, status indicators
- **Related:** avatar, logo, code_block, markdown, chart, stat, inline_meta, kbd, badge, carousel
- **Boundary type:** passthrough
- **Import:** `use canonrs::ui::icon::Icon;`

### Migration

```rust
// ❌ Typical
view! {
  <svg class="icon-lg">"★"</svg>
}
```

```rust
// ✅ CanonRS
view! {
  <Icon size=IconSize::Md>"★"</Icon>
}
```


---

## `icon-button`

- **Label:** Icon Button
- **Category:** Action
- **Intent:** Trigger an action with an icon button
- **Description:** Button with icon only
- **Pain:** Icon buttons miss accessibility labels and loading state handling
- **Promise:** Accessibility and loading state enforced in button contract
- **Why:** IconButtonPrimitive requires aria-label and encodes loading and disabled states. ARIA attributes are derived automatically. This guarantees accessible and predictable behavior.
- **Rules:** CR-001, CR-004
- **Use cases:** close actions, toolbar buttons
- **Related:** button, button_group, copy_button, link
- **Capabilities:** Disabled
- **States:** hover, active, focus, disabled, loading
- **Boundary type:** init
- **Import:** `use canonrs::ui::icon_button::IconButton;`

### Migration

```rust
// ❌ Typical
view! {
  <button><svg /></button>
}
```

```rust
// ✅ CanonRS
view! {
  <IconButton aria_label="Close">"×"</IconButton>
}
```


---

## `inline-meta`

- **Label:** Inline Meta
- **Category:** Display
- **Intent:** Display structured metadata inline (label + value pairs)
- **Description:** Inline metadata display for stats, versions, dates and counts
- **Pain:** Inline metadata mixes labels and values without structure
- **Promise:** Metadata pairs structured and consistently rendered
- **Why:** InlineMetaPrimitive separates label and value into explicit primitives. This enforces consistent layout and semantics. It prevents ad-hoc inline data rendering.
- **Rules:** CR-001, CR-004
- **Use cases:** stats display, metadata labels
- **Related:** avatar, icon, logo, code_block, markdown, chart, stat, kbd, badge, carousel
- **Optional parts:** InlineMetaLabel, InlineMetaValue
- **Boundary type:** passthrough
- **Import:** `use canonrs::ui::inline_meta::InlineMeta;`

### Migration

```rust
// ❌ Typical
view! {
  <span>"Rules: 284"</span>
}
```

```rust
// ✅ CanonRS
view! {
  <InlineMeta>
    <InlineMetaLabel>"Rules"</InlineMetaLabel>
    <InlineMetaValue>"284"</InlineMetaValue>
  </InlineMeta>
}
```


---

## `inline-notice`

- **Label:** Inline Notice
- **Category:** Feedback
- **Intent:** Show inline contextual feedback
- **Description:** Inline notice message
- **Pain:** Inline messages use wrong ARIA roles and urgency levels
- **Promise:** Role and aria-live automatically enforced by variant
- **Why:** InlineNoticeVariant controls both semantic role and aria-live behavior. Error uses alert/assertive while others use status/polite. This guarantees correct urgency signaling without manual ARIA decisions.
- **Rules:** CR-001, CR-004
- **Use cases:** form inline errors, contextual hints
- **Related:** toast, alert, banner, callout, status_dot
- **Optional parts:** InlineNoticeIcon, InlineNoticeContent
- **States:** error
- **Boundary type:** passthrough
- **Import:** `use canonrs::ui::inline_notice::InlineNotice;`

### Migration

```rust
// ❌ Typical
view! {
  <div class="notice error">"Error"</div>
}
```

```rust
// ✅ CanonRS
view! {
  <InlineNotice variant=InlineNoticeVariant::Error>
    <InlineNoticeContent>"Error"</InlineNoticeContent>
  </InlineNotice>
}
```


---

## `input`

- **Label:** Input
- **Category:** Form
- **Intent:** Capture text or data from user
- **Description:** Text input field
- **Pain:** Inputs accept invalid visual states without consistent validation mapping
- **Promise:** Variant and size strictly constrained via typed enums
- **Why:** InputVariant and InputSize define allowed visual states at compile-time. DisabledState is mapped to both DOM and ARIA. This guarantees consistent rendering and prevents invalid combinations.
- **Rules:** CR-001, CR-004
- **Use cases:** forms, search fields
- **Related:** form, input_group, input_otp, textarea, field, label, checkbox, form_error_summary
- **Capabilities:** Value, Disabled
- **States:** focus, disabled
- **Boundary type:** passthrough
- **Import:** `use canonrs::ui::input::Input;`

### Migration

```rust
// ❌ Typical
view! {
  <input class="input error large" />
}
```

```rust
// ✅ CanonRS
view! {
  <Input variant=InputVariant::Error size=InputSize::Lg />
}
```


---

## `input-group`

- **Label:** Input Group
- **Category:** Form
- **Intent:** Group input with prefix or suffix elements
- **Description:** Input group with addons
- **Pain:** Input addons break alignment and border radius consistency
- **Promise:** Grouped inputs maintain consistent structure and visual merging
- **Why:** InputGroupPrimitive encodes merge-radius behavior via ActivityState. This ensures inputs and addons render as a unified control. It prevents layout inconsistencies across grouped inputs.
- **Rules:** CR-001, CR-004
- **Use cases:** email fields, currency inputs
- **Related:** form, input, input_otp, textarea, field, label, checkbox, form_error_summary
- **States:** focus-within
- **Boundary type:** init
- **Import:** `use canonrs::ui::input_group::InputGroup;`

### Migration

```rust
// ❌ Typical
view! {
  <div class="input-group">
    <span>@</span>
    <input />
  </div>
}
```

```rust
// ✅ CanonRS
view! {
  <InputGroup merge_radius=true>
    <span data-rs-input-group-addon="">"@"</span>
  </InputGroup>
}
```


---

## `input-otp`

- **Label:** OTP Input
- **Category:** Form
- **Intent:** Capture one-time password codes
- **Description:** One-time password input
- **Pain:** OTP inputs require complex state sync across multiple fields
- **Promise:** OTP slots and active state managed automatically
- **Why:** InputOtp distributes value across slots using ActivityState. Each slot reflects position and focus without manual logic. This guarantees synchronized UI and input state.
- **Rules:** CR-001, CR-004
- **Use cases:** 2FA codes, verification inputs
- **Related:** form, input, input_group, textarea, field, label, checkbox, form_error_summary
- **States:** active
- **Boundary type:** init
- **Import:** `use canonrs::ui::input_otp::InputOtp;`

### Migration

```rust
// ❌ Typical
// multiple inputs with manual focus handling
```

```rust
// ✅ CanonRS
view! {
  <InputOtp length=6 />
}
```


---

## `kbd`

- **Label:** Kbd
- **Category:** Display
- **Intent:** Display keyboard shortcut
- **Description:** Keyboard shortcut display
- **Pain:** Keyboard shortcuts displayed inconsistently with ad-hoc styling
- **Promise:** Shortcut representation standardized via size and variant enums
- **Why:** KbdPrimitive encodes size and variant into data attributes. Group and separator primitives enforce consistent composition. This guarantees uniform shortcut display.
- **Rules:** CR-001, CR-004
- **Use cases:** shortcuts, docs
- **Related:** avatar, icon, logo, code_block, markdown, chart, stat, inline_meta, badge, carousel
- **Boundary type:** passthrough
- **Import:** `use canonrs::ui::kbd::Kbd;`

### Migration

```rust
// ❌ Typical
view! {
  <span class="kbd">Ctrl + K</span>
}
```

```rust
// ✅ CanonRS
view! {
  <KbdGroup>
    <Kbd>"Ctrl"</Kbd>
    <KbdSeparator />
    <Kbd>"K"</Kbd>
  </KbdGroup>
}
```


---

## `label`

- **Label:** Label
- **Category:** Form
- **Intent:** Associate a label with a form control
- **Description:** Form label component
- **Pain:** Labels not correctly associated with inputs, breaking accessibility
- **Promise:** Label-to-input association enforced via explicit html_for contract
- **Why:** LabelPrimitive ensures proper for/id mapping and required state. ARIA attributes are derived automatically. This guarantees accessible labeling without manual wiring.
- **Rules:** CR-001, CR-004
- **Use cases:** forms, inputs
- **Related:** form, input, input_group, input_otp, textarea, field, checkbox, form_error_summary
- **Boundary type:** passthrough
- **Import:** `use canonrs::ui::label::Label;`

### Migration

```rust
// ❌ Typical
view! {
  <label>"Email"</label>
  <input />
}
```

```rust
// ✅ CanonRS
view! {
  <Label for_id="email">"Email"</Label>
}
```


---

## `link`

- **Label:** Link
- **Category:** Navigation
- **Intent:** Navigate to a URL or trigger action
- **Description:** Hyperlink
- **Pain:** Links misuse target, rel and disabled states inconsistently
- **Promise:** Navigation semantics and external behavior enforced structurally
- **Why:** LinkPrimitive controls variant, disabled and external behavior. It automatically sets target and rel attributes. This guarantees safe navigation and consistent semantics.
- **Rules:** CR-001, CR-004
- **Use cases:** navigation, external links
- **Related:** button, button_group, icon_button, copy_button
- **Capabilities:** Disabled
- **States:** active, hover, disabled
- **Boundary type:** passthrough
- **Import:** `use canonrs::ui::link::Link;`

### Migration

```rust
// ❌ Typical
view! {
  <a href="/" target="_blank">"Link"</a>
}
```

```rust
// ✅ CanonRS
view! {
  <Link href="/" external=true>"Link"</Link>
}
```


---

## `link-group`

- **Label:** Link Group
- **Category:** Navigation
- **Intent:** Semantic group of navigation links with optional label
- **Description:** Wrapper that organizes multiple NavItems into a labeled navigation group
- **Pain:** Navigation links lack grouping semantics and structural consistency
- **Promise:** Grouped navigation structured with direction and labeling contract
- **Why:** LinkGroup uses NavigationGroup primitives to enforce grouping and labeling. Direction is encoded via enum. This guarantees consistent navigation structure across layouts.
- **Rules:** CR-001, CR-004
- **Use cases:** sidebars, footers
- **Related:** navigation_menu, sidebar, nav_item, breadcrumb, pagination
- **Capabilities:** Orientation, Disabled
- **States:** active
- **Boundary type:** interaction
- **Import:** `use canonrs::ui::link_group::LinkGroup;`

### Migration

```rust
// ❌ Typical
view! {
  <div>
    <a>"A"</a>
    <a>"B"</a>
  </div>
}
```

```rust
// ✅ CanonRS
view! {
  <LinkGroup>
    <NavItem label="A" href="/a" />
  </LinkGroup>
}
```


---

## `list-item`

- **Label:** List Item
- **Category:** Display
- **Intent:** Display a single item in a list
- **Description:** Single list item with title and description
- **Pain:** Lists lack consistent selection, disabled and accessibility states
- **Promise:** Selection and interaction states encoded via structured attributes
- **Why:** ListItem encodes selectable, selected and disabled states into data attributes. ARIA attributes are derived automatically. This guarantees consistent list interaction behavior.
- **Rules:** CR-001, CR-004
- **Use cases:** menus, lists
- **Related:** table, data_table, virtual_list, empty_table, tree
- **Capabilities:** Selected, Disabled
- **Optional parts:** ListItemTitle, ListItemDescription
- **States:** selected
- **Boundary type:** interaction
- **Import:** `use canonrs::ui::list_item::ListItem;`

### Migration

```rust
// ❌ Typical
view! {
  <li class="active">"Item"</li>
}
```

```rust
// ✅ CanonRS
view! {
  <List>
    <ListItem selectable=true selected=true>"Item"</ListItem>
  </List>
}
```


---

## `loading-overlay`

- **Label:** Loading Overlay
- **Category:** Display
- **Intent:** Block UI during async operations
- **Description:** Full loading overlay
- **Pain:** Loading states require manual visibility and aria synchronization
- **Promise:** Loading visibility and aria-busy managed automatically
- **Why:** LoadingOverlayPrimitive maps LoadingState to visibility and ARIA attributes. Overlay visibility is derived automatically. This guarantees consistent loading feedback without manual logic.
- **Rules:** CR-001, CR-004
- **Use cases:** async operations, page blocking
- **Related:** progress, spinner, skeleton, pulse, doc_progress
- **Capabilities:** OpenClose
- **States:** loading
- **Boundary type:** init
- **Import:** `use canonrs::ui::loading_overlay::LoadingOverlay;`

### Migration

```rust
// ❌ Typical
view! {
  {if loading { view! { <div class="overlay">"Loading"</div> } }}
}
```

```rust
// ✅ CanonRS
view! {
  <LoadingOverlay state=LoadingState::Loading>
    "Content"
  </LoadingOverlay>
}
```


---

## `logo`

- **Label:** Site Logo
- **Category:** Brand
- **Intent:** Brand identity link pointing to home
- **Description:** CanonRS logo combining SVG icon, wordmark and optional tagline
- **Pain:** Brand logos implemented inconsistently across layouts and break navigation semantics
- **Promise:** Brand identity structure and navigation behavior enforced in a single contract
- **Why:** LogoPrimitive enforces anchor semantics, size and variant at the root element. Icon, wordmark and tagline are explicitly structured parts. This guarantees consistent brand rendering and correct navigation behavior.
- **Rules:** CR-001, CR-004
- **Use cases:** site header, app navigation
- **Related:** avatar, icon, code_block, markdown, chart, stat, inline_meta, kbd, badge, carousel
- **Boundary type:** passthrough
- **Import:** `use canonrs::ui::logo::Logo;`

### Migration

```rust
// ❌ Typical
view! {
  <a href="/" class="logo">
    <img src="/logo.svg" />
    <span>"Brand"</span>
  </a>
}
```

```rust
// ✅ CanonRS
view! {
  <Logo>
    <LogoWordmarkPrimitive>"Brand"</LogoWordmarkPrimitive>
  </Logo>
}
```


---

## `markdown`

- **Label:** Markdown
- **Category:** Display
- **Intent:** Render markdown content as HTML
- **Description:** Rendered markdown content
- **Pain:** Client-side markdown rendering causes hydration mismatch and inconsistent DOM
- **Promise:** SSR-safe markdown rendering with deterministic DOM output
- **Why:** MarkdownPrimitive injects HTML only during SSR and avoids client mutation. Content and TOC are generated as stable HTML structure. This guarantees hydration-safe rendering with no runtime divergence.
- **Rules:** CR-001, CR-004
- **Use cases:** docs, blog content
- **Related:** avatar, icon, logo, code_block, chart, stat, inline_meta, kbd, badge, carousel
- **Boundary type:** interaction
- **Import:** `use canonrs::ui::markdown::Markdown;`

### Migration

```rust
// ❌ Typical
view! {
  <div inner_html={render_markdown(md)} />
}
```

```rust
// ✅ CanonRS
view! {
  <MarkdownSurface rendered=rendered />
}
```


---

## `menu`

- **Label:** Menu
- **Category:** Navigation
- **Intent:** Static vertical menu list
- **Description:** Menu component
- **Pain:** Menus lack consistent selection, disabled and focus behavior
- **Promise:** Menu interaction fully governed via structured ARIA and state attributes
- **Why:** MenuItemPrimitive encodes selection, disabled and activity states into data-rs and ARIA attributes. Navigation semantics are enforced at the container level. This guarantees predictable keyboard and accessibility behavior.
- **Rules:** CR-001, CR-004
- **Use cases:** dropdown lists, action menus
- **Related:** dropdown_menu, context_menu, menubar, command
- **States:** open, closed
- **Boundary type:** init
- **Import:** `use canonrs::ui::menu::Menu;`

### Migration

```rust
// ❌ Typical
view! {
  <div class="menu">
    <button class="active">"Item"</button>
  </div>
}
```

```rust
// ✅ CanonRS
view! {
  <Menu>
    <MenuItem selected=true>"Item"</MenuItem>
  </Menu>
}
```


---

## `menubar`

- **Label:** Menubar
- **Category:** Navigation
- **Intent:** Horizontal application menu bar
- **Description:** Menu bar navigation
- **Pain:** Horizontal menus break ARIA roles and keyboard navigation
- **Promise:** Menubar semantics and structure enforced via primitives
- **Why:** MenubarPrimitive enforces role="menubar" and structured menu composition. Trigger and content follow strict ARIA relationships. This guarantees accessible and predictable navigation behavior.
- **Rules:** CR-001, CR-004
- **Use cases:** desktop apps, top navigation
- **Related:** dropdown_menu, context_menu, menu, command
- **Capabilities:** OpenClose
- **Required parts:** MenubarMenu, MenubarTrigger
- **Optional parts:** MenubarContent, MenubarItem, MenubarSeparator
- **States:** active
- **Boundary type:** interaction
- **Import:** `use canonrs::ui::menubar::Menubar;`

### Migration

```rust
// ❌ Typical
view! {
  <div class="menubar">
    <button>"File"</button>
  </div>
}
```

```rust
// ✅ CanonRS
view! {
  <Menubar>
    <MenubarMenu>
      <MenubarTrigger>"File"</MenubarTrigger>
    </MenubarMenu>
  </Menubar>
}
```


---

## `modal`

- **Label:** Modal
- **Category:** Overlay
- **Intent:** Generic modal container
- **Description:** Modal window component
- **Pain:** Modals desync visibility, aria-hidden and focus management
- **Promise:** Modal visibility and accessibility fully synchronized via state
- **Why:** ModalPrimitive maps VisibilityState to aria-hidden and hidden attributes. Trigger and content share the same state contract. This guarantees consistent open/close behavior and accessibility.
- **Rules:** CR-001, CR-004
- **Use cases:** dialogs, overlays
- **Related:** dialog, alert_dialog, drawer, sheet, confirm_dialog, tooltip, hover_card, popover
- **Capabilities:** OpenClose, FocusTrap, AriaModal
- **Optional parts:** ModalOverlay, ModalContent, ModalClose
- **States:** open, closed
- **Boundary type:** interaction
- **Import:** `use canonrs::ui::modal::Modal;`

### Migration

```rust
// ❌ Typical
view! {
  {if open { view! { <div class="modal">"Content"</div> } }}
}
```

```rust
// ✅ CanonRS
view! {
  <Modal state=VisibilityState::Open>
    <ModalContent>"Content"</ModalContent>
  </Modal>
}
```


---

## `nav-item`

- **Label:** Nav Item
- **Category:** Navigation
- **Intent:** Single navigation link item
- **Description:** Single navigation item
- **Pain:** Navigation links lack active state and accessibility consistency
- **Promise:** Active and disabled navigation states enforced structurally
- **Why:** NavItemPrimitive encodes ActivityState and DisabledState into data attributes and ARIA. aria-current is derived automatically. This guarantees consistent navigation behavior and accessibility.
- **Rules:** CR-001, CR-004
- **Use cases:** sidebars, menus
- **Related:** navigation_menu, sidebar, breadcrumb, pagination, link_group
- **Capabilities:** Active, Disabled
- **States:** active, disabled
- **Boundary type:** init
- **Import:** `use canonrs::ui::nav_item::NavItem;`

### Migration

```rust
// ❌ Typical
view! {
  <a class="active" href="/">"Home"</a>
}
```

```rust
// ✅ CanonRS
view! {
  <NavItem label="Home" active=true />
}
```


---

## `navigation-menu`

- **Label:** Navigation Menu
- **Category:** Navigation
- **Intent:** Primary site navigation with submenus
- **Description:** Navigation menu
- **Pain:** Nested navigation menus require id wiring and break interaction consistency
- **Promise:** Trigger-content relationship enforced without id wiring
- **Why:** NavigationMenu uses DOM structure and data-rs-state instead of ids. Trigger and content are linked via closest/sibling logic. This guarantees stable interaction without manual wiring.
- **Rules:** CR-001, CR-004
- **Use cases:** site navigation, dropdown navigation
- **Related:** sidebar, nav_item, breadcrumb, pagination, link_group
- **Capabilities:** OpenClose
- **Required parts:** NavigationMenuList, NavigationMenuItem
- **Optional parts:** NavigationMenuTrigger, NavigationMenuContent, NavigationMenuLink
- **States:** open, closed, active
- **Boundary type:** init
- **Import:** `use canonrs::ui::navigation_menu::NavigationMenu;`

### Migration

```rust
// ❌ Typical
view! {
  <button id="trigger">"Menu"</button>
  <div id="content">"Items"</div>
}
```

```rust
// ✅ CanonRS
view! {
  <NavigationMenu>
    <NavigationMenuItem>
      <NavigationMenuTrigger>"Menu"</NavigationMenuTrigger>
      <NavigationMenuContent>"Items"</NavigationMenuContent>
    </NavigationMenuItem>
  </NavigationMenu>
}
```


---

## `page-header`

- **Label:** Page Header
- **Category:** Display
- **Intent:** Page title and actions bar
- **Description:** Page header with title and actions
- **Pain:** Page headers mix title, actions and breadcrumbs inconsistently
- **Promise:** Header structure enforced with explicit semantic regions
- **Why:** PageHeaderPrimitive defines structured parts like title, description and actions. Each region is explicitly declared. This guarantees consistent layout composition across pages.
- **Rules:** CR-001, CR-004
- **Use cases:** dashboards, admin pages
- **Related:** card, resizable, scroll_area, aspect_ratio, toolbar, separator
- **Boundary type:** passthrough
- **Import:** `use canonrs::ui::page_header::PageHeader;`

### Migration

```rust
// ❌ Typical
view! {
  <div class="header">
    <h1>"Title"</h1>
  </div>
}
```

```rust
// ✅ CanonRS
view! {
  <PageHeader>
    <PageHeaderContent>
      <PageHeaderTitle>"Title"</PageHeaderTitle>
    </PageHeaderContent>
  </PageHeader>
}
```


---

## `pagination`

- **Label:** Pagination
- **Category:** Navigation
- **Intent:** Navigate between pages of content
- **Description:** Page navigation control
- **Pain:** Pagination links lack active state and disabled navigation semantics
- **Promise:** Navigation state and accessibility enforced via structured primitives
- **Why:** PaginationPrimitive encodes navigation semantics with aria-current and disabled states. Previous and next controls enforce accessibility automatically. This guarantees consistent pagination behavior.
- **Rules:** CR-001, CR-004
- **Use cases:** tables, lists
- **Related:** navigation_menu, sidebar, nav_item, breadcrumb, link_group
- **Capabilities:** Active, Disabled
- **Optional parts:** PaginationContent, PaginationItem, PaginationLink
- **States:** active, disabled
- **Boundary type:** interaction
- **Import:** `use canonrs::ui::pagination::Pagination;`

### Migration

```rust
// ❌ Typical
view! {
  <a class="active">"1"</a>
}
```

```rust
// ✅ CanonRS
view! {
  <Pagination>
    <PaginationContent>
      <PaginationLink state=ActivityState::Active href="#">"1"</PaginationLink>
    </PaginationContent>
  </Pagination>
}
```


---

## `popover`

- **Label:** Popover
- **Category:** Overlay
- **Intent:** Show contextual floating content
- **Description:** Floating popover component
- **Pain:** Popovers lose sync between trigger state and content visibility
- **Promise:** Trigger and content visibility governed by shared state contract
- **Why:** PopoverPrimitive uses VisibilityState across trigger and content. ARIA attributes and positioning are derived automatically. This guarantees consistent overlay behavior without manual sync.
- **Rules:** CR-001, CR-004
- **Use cases:** context actions, tooltips with actions
- **Related:** dialog, alert_dialog, drawer, sheet, modal, confirm_dialog, tooltip, hover_card
- **Capabilities:** OpenClose, FocusTrap
- **Required parts:** PopoverTrigger, PopoverContent
- **States:** open, closed
- **Boundary type:** interaction
- **Import:** `use canonrs::ui::popover::Popover;`

### Migration

```rust
// ❌ Typical
view! {
  <button on:click=toggle>"Open"</button>
  {if open { view! { <div>"Popover"</div> } }}
}
```

```rust
// ✅ CanonRS
view! {
  <Popover>
    <button data-rs-popover-trigger="">"Open"</button>
    <PopoverContent>"Popover"</PopoverContent>
  </Popover>
}
```


---

## `progress`

- **Label:** Progress
- **Category:** Feedback
- **Intent:** Show completion of a task
- **Description:** Progress bar indicator
- **Pain:** Progress bars overflow or misreport values beyond valid range
- **Promise:** Progress value always clamped and ARIA-compliant
- **Why:** ProgressPrimitive clamps value between 0–100 and maps it to aria-valuenow. Indicator movement is derived from this value. This guarantees consistent visual and accessibility behavior.
- **Rules:** CR-001, CR-004
- **Use cases:** file upload, task completion
- **Related:** spinner, skeleton, pulse, loading_overlay, doc_progress
- **Capabilities:** Value
- **Required parts:** ProgressIndicator
- **States:** loading
- **Boundary type:** init
- **Import:** `use canonrs::ui::progress::Progress;`

### Migration

```rust
// ❌ Typical
view! {
  <div class="progress" style="width:150%"></div>
}
```

```rust
// ✅ CanonRS
view! {
  <Progress value=75.0 />
}
```


---

## `pulse`

- **Label:** Pulse
- **Category:** Feedback
- **Intent:** Animated attention indicator
- **Description:** Pulse animation wrapper
- **Pain:** Animations applied inconsistently with arbitrary classes and speeds
- **Promise:** Animation variant, size and speed strictly typed
- **Why:** PulsePrimitive encodes variant, size and speed as enums mapped to data attributes. This removes class-based inconsistencies. It guarantees predictable animation behavior.
- **Rules:** CR-001, CR-004
- **Use cases:** status indicators, attention highlights
- **Related:** progress, spinner, skeleton, loading_overlay, doc_progress
- **States:** active
- **Boundary type:** passthrough
- **Import:** `use canonrs::ui::pulse::Pulse;`

### Migration

```rust
// ❌ Typical
view! {
  <span class="pulse-fast big"></span>
}
```

```rust
// ✅ CanonRS
view! {
  <Pulse variant=PulseVariant::Emphasized speed=PulseSpeed::Fast />
}
```


---

## `radio`

- **Label:** Radio
- **Category:** Form
- **Intent:** Select one option from a group
- **Description:** Radio button input
- **Pain:** Radio inputs desync checked state and accessibility attributes
- **Promise:** Selection state mapped directly to DOM and ARIA
- **Why:** RadioPrimitive maps SelectionState to checked and aria attributes. Disabled state is also enforced structurally. This guarantees consistent selection behavior.
- **Rules:** CR-001, CR-004
- **Use cases:** forms, single choice inputs
- **Related:** select, combobox, radio_group, color_picker, slider
- **Capabilities:** Disabled
- **Required parts:** RadioGroup
- **States:** selected, focus, disabled
- **Boundary type:** init
- **Import:** `use canonrs::ui::radio::Radio;`

### Migration

```rust
// ❌ Typical
view! {
  <input type="radio" checked />
}
```

```rust
// ✅ CanonRS
view! {
  <Radio value="a" name="group" checked=true>"Option"</Radio>
}
```


---

## `radio-group`

- **Label:** Radio Group
- **Category:** Form
- **Intent:** Group radio buttons for single selection
- **Description:** Group of radio buttons
- **Pain:** Radio groups lose exclusivity and accessibility grouping
- **Promise:** Group semantics and exclusivity enforced at container level
- **Why:** RadioGroupPrimitive enforces role="radiogroup" and shared disabled state. Items derive selection state consistently. This guarantees exclusive selection behavior.
- **Rules:** CR-001, CR-004
- **Use cases:** settings, forms
- **Related:** select, combobox, radio, color_picker, slider
- **Capabilities:** Disabled
- **States:** selected, focus, disabled
- **Boundary type:** interaction
- **Import:** `use canonrs::ui::radio_group::RadioGroup;`

### Migration

```rust
// ❌ Typical
view! {
  <div>
    <input type="radio" name="a" />
    <input type="radio" name="a" />
  </div>
}
```

```rust
// ✅ CanonRS
view! {
  <RadioGroup>
    <RadioGroupItem name="a" value="1">"One"</RadioGroupItem>
  </RadioGroup>
}
```


---

## `resizable`

- **Label:** Resizable
- **Category:** Layout
- **Intent:** Split panels with draggable divider
- **Description:** Resizable panel component
- **Pain:** Resizable panels break layout constraints and overflow limits
- **Promise:** Panel sizes constrained and behavior encoded structurally
- **Why:** ResizablePrimitive defines orientation and size limits via attributes. Panels and handles follow strict composition. This guarantees controlled resizing behavior.
- **Rules:** CR-001, CR-004
- **Use cases:** editors, dashboards
- **Related:** card, scroll_area, aspect_ratio, page_header, toolbar, separator
- **Capabilities:** Orientation, Resize
- **Required parts:** ResizablePanel, ResizableHandle
- **States:** active, inactive
- **Boundary type:** interaction
- **Import:** `use canonrs::ui::resizable::Resizable;`

### Migration

```rust
// ❌ Typical
view! {
  <div class="split">
    <div></div>
    <div></div>
  </div>
}
```

```rust
// ✅ CanonRS
view! {
  <Resizable>
    <ResizablePanel />
    <ResizableHandle />
  </Resizable>
}
```


---

## `scroll-area`

- **Label:** Scroll Area
- **Category:** Layout
- **Intent:** Scrollable container with custom scrollbar
- **Description:** Scrollable area container
- **Pain:** Custom scroll containers break keyboard navigation and accessibility
- **Promise:** Scroll behavior and accessibility enforced via structured container
- **Why:** ScrollAreaPrimitive defines viewport, scrollbars and orientation explicitly. ARIA roles and keyboard access are enforced. This guarantees accessible scrolling behavior.
- **Rules:** CR-001, CR-004
- **Use cases:** long lists, logs
- **Related:** card, resizable, aspect_ratio, page_header, toolbar, separator
- **Capabilities:** Overflow
- **States:** active
- **Boundary type:** interaction
- **Import:** `use canonrs::ui::scroll_area::ScrollArea;`

### Migration

```rust
// ❌ Typical
view! {
  <div style="overflow:auto;height:200px"></div>
}
```

```rust
// ✅ CanonRS
view! {
  <ScrollArea>
    <div>"Content"</div>
  </ScrollArea>
}
```


---

## `section-ui`

- **Label:** Section UI
- **Category:** Display
- **Intent:** Semantic UI elements inside Section layout
- **Description:** Section header typography components
- **Import:** `use canonrs::ui::section_ui::SectionUi;`

---

## `select`

- **Label:** Select
- **Category:** Form
- **Intent:** Choose one option from a list
- **Description:** Dropdown select input
- **Pain:** Select components require manual state sync between trigger and options
- **Promise:** Selection, visibility and interaction fully governed by structure
- **Why:** SelectPrimitive encodes open state, selection and disabled behavior via data attributes. Trigger and content are synchronized without id wiring. This guarantees consistent dropdown behavior.
- **Rules:** CR-001, CR-004
- **Use cases:** forms, filters
- **Related:** combobox, radio, radio_group, color_picker, slider
- **Capabilities:** OpenClose, Disabled
- **Required parts:** SelectTrigger, SelectContent, SelectItem
- **Optional parts:** SelectValue, SelectSeparator
- **States:** open, closed, selected, disabled
- **Boundary type:** interaction
- **Import:** `use canonrs::ui::select::Select;`

### Migration

```rust
// ❌ Typical
view! {
  <select>
    <option>"A"</option>
  </select>
}
```

```rust
// ✅ CanonRS
view! {
  <Select>
    <SelectTrigger>
      <SelectValue placeholder="Select..." />
    </SelectTrigger>
    <SelectContent>
      <SelectItem value="a">"A"</SelectItem>
    </SelectContent>
  </Select>
}
```


---

## `separator`

- **Label:** Separator
- **Category:** Layout
- **Intent:** Visually divide content sections
- **Description:** Visual divider line
- **Pain:** Dividers lack semantic meaning and accessibility roles
- **Promise:** Separator semantics enforced via orientation and role contract
- **Why:** SeparatorPrimitive encodes orientation and decorative behavior. ARIA roles are derived automatically. This guarantees semantic and accessible separators.
- **Rules:** CR-001, CR-004
- **Use cases:** layout separation, menus
- **Related:** card, resizable, scroll_area, aspect_ratio, page_header, toolbar
- **Capabilities:** Orientation
- **Boundary type:** passthrough
- **Import:** `use canonrs::ui::separator::Separator;`

### Migration

```rust
// ❌ Typical
view! {
  <hr />
}
```

```rust
// ✅ CanonRS
view! {
  <Separator />
}
```


---

## `sheet`

- **Label:** Sheet
- **Category:** Overlay
- **Intent:** Side panel for forms or navigation
- **Description:** Sheet panel overlay
- **Pain:** Side panels desync visibility, overlay and focus behavior
- **Promise:** Sheet visibility and overlay fully governed via shared state
- **Why:** SheetPrimitive maps VisibilityState to both panel and overlay. Side positioning is encoded structurally. This guarantees consistent slide-in behavior and accessibility.
- **Rules:** CR-001, CR-004
- **Use cases:** mobile navigation, side panels
- **Related:** dialog, alert_dialog, drawer, modal, confirm_dialog, tooltip, hover_card, popover
- **Capabilities:** OpenClose, FocusTrap, KeyboardEsc
- **Required parts:** SheetContent
- **Optional parts:** SheetOverlay
- **States:** open, closed
- **Boundary type:** interaction
- **Import:** `use canonrs::ui::sheet::Sheet;`

### Migration

```rust
// ❌ Typical
view! {
  {if open { view! { <div class="drawer">"Content"</div> } }}
}
```

```rust
// ✅ CanonRS
view! {
  <Sheet>
    <SheetContent aria_labelledby="title">"Content"</SheetContent>
  </Sheet>
}
```


---

## `sidebar`

- **Label:** Sidebar
- **Category:** Navigation
- **Intent:** Vertical navigation panel
- **Description:** Sidebar navigation component
- **Pain:** Sidebars lose active state, disabled links and structural consistency
- **Promise:** Navigation state and structure enforced at component level
- **Why:** SidebarPrimitive encodes visibility, variant and navigation semantics. Menu items derive active and disabled states automatically. This guarantees consistent navigation behavior.
- **Rules:** CR-001, CR-004
- **Use cases:** app navigation, admin panels
- **Related:** navigation_menu, nav_item, breadcrumb, pagination, link_group
- **Capabilities:** OpenClose
- **Required parts:** SidebarContent
- **Optional parts:** SidebarHeader, SidebarFooter, SidebarMenu, SidebarMenuItem
- **States:** open, closed, active, disabled
- **Boundary type:** interaction
- **Import:** `use canonrs::ui::sidebar::Sidebar;`

### Migration

```rust
// ❌ Typical
view! {
  <aside>
    <a class="active">"Home"</a>
  </aside>
}
```

```rust
// ✅ CanonRS
view! {
  <Sidebar>
    <SidebarMenu>
      <SidebarMenuItem active=ActivityState::Active href="/">"Home"</SidebarMenuItem>
    </SidebarMenu>
  </Sidebar>
}
```


---

## `skeleton`

- **Label:** Skeleton
- **Category:** Feedback
- **Intent:** Show placeholder while content loads
- **Description:** Loading skeleton placeholder
- **Pain:** Loading placeholders inconsistent and lack accessibility semantics
- **Promise:** Skeleton state and variant standardized via structure
- **Why:** SkeletonPrimitive encodes variant and loading state with aria-busy. This guarantees consistent placeholder rendering and accessibility feedback.
- **Rules:** CR-001, CR-004
- **Use cases:** loading states, content placeholders
- **Related:** progress, spinner, pulse, loading_overlay, doc_progress
- **States:** loading
- **Boundary type:** passthrough
- **Import:** `use canonrs::ui::skeleton::Skeleton;`

### Migration

```rust
// ❌ Typical
view! {
  <div class="skeleton"></div>
}
```

```rust
// ✅ CanonRS
view! {
  <Skeleton variant=SkeletonVariant::Text />
}
```


---

## `slider`

- **Label:** Slider
- **Category:** Form
- **Intent:** Select a value within a range
- **Description:** Range slider input
- **Pain:** Sliders allow invalid values and break accessibility attributes
- **Promise:** Value clamped and ARIA attributes enforced automatically
- **Why:** SliderPrimitive clamps value within min/max and maps percent and aria-valuenow. Track and thumb are structurally defined. This guarantees consistent interaction behavior.
- **Rules:** CR-001, CR-004
- **Use cases:** volume control, range selection
- **Related:** select, combobox, radio, radio_group, color_picker
- **Capabilities:** Value, Disabled
- **Required parts:** SliderTrack, SliderThumb
- **Optional parts:** SliderRange
- **States:** active, disabled
- **Boundary type:** interaction
- **Import:** `use canonrs::ui::slider::Slider;`

### Migration

```rust
// ❌ Typical
view! {
  <input type="range" value="200" />
}
```

```rust
// ✅ CanonRS
view! {
  <Slider min=0.0 max=100.0 value=50.0 />
}
```


---

## `spinner`

- **Label:** Spinner
- **Category:** Feedback
- **Intent:** Indicate loading or processing
- **Description:** Loading spinner
- **Pain:** Loading indicators lack consistent size and accessibility state
- **Promise:** Spinner state and size strictly controlled via enums
- **Why:** SpinnerPrimitive encodes size and LoadingState into ARIA attributes. aria-busy and role=status are enforced. This guarantees accessible loading indicators.
- **Rules:** CR-001, CR-004
- **Use cases:** loading indicators, async feedback
- **Related:** progress, skeleton, pulse, loading_overlay, doc_progress
- **States:** loading
- **Boundary type:** passthrough
- **Import:** `use canonrs::ui::spinner::Spinner;`

### Migration

```rust
// ❌ Typical
view! {
  <div class="spinner"></div>
}
```

```rust
// ✅ CanonRS
view! {
  <Spinner size=SpinnerSize::Large />
}
```


---

## `stat`

- **Label:** Stat
- **Category:** Display
- **Intent:** Display a key metric with label
- **Description:** Metric stat display
- **Pain:** Metrics displayed without consistent structure or alignment
- **Promise:** Metric layout and semantics enforced via structured primitives
- **Why:** StatPrimitive enforces composition of value, label and optional delta. Size, alignment and trend are encoded via attributes. This guarantees consistent KPI display.
- **Rules:** CR-001, CR-004
- **Use cases:** dashboards, analytics
- **Related:** avatar, icon, logo, code_block, markdown, chart, inline_meta, kbd, badge, carousel
- **Required parts:** StatValue, StatLabel
- **Optional parts:** StatDelta, StatIcon
- **Boundary type:** passthrough
- **Import:** `use canonrs::ui::stat::Stat;`

### Migration

```rust
// ❌ Typical
view! {
  <div>
    <h1>"100"</h1>
    <p>"Users"</p>
  </div>
}
```

```rust
// ✅ CanonRS
view! {
  <Stat>
    <StatValue>"100"</StatValue>
    <StatLabel>"Users"</StatLabel>
  </Stat>
}
```


---

## `status-dot`

- **Label:** Status Dot
- **Category:** Display
- **Intent:** Indicate user presence or availability
- **Description:** Status indicator dot
- **Pain:** Status indicators mix semantic feedback with presence states
- **Promise:** Presence states strictly separated from semantic feedback
- **Why:** StatusDotVariant encodes only presence states like online or busy. ARIA labels are derived automatically. This guarantees correct semantic usage.
- **Rules:** CR-001, CR-004
- **Use cases:** user presence, chat apps
- **Related:** toast, alert, banner, callout, inline_notice
- **States:** active, inactive
- **Boundary type:** init
- **Import:** `use canonrs::ui::status_dot::StatusDot;`

### Migration

```rust
// ❌ Typical
view! {
  <span class="green-dot"></span>
}
```

```rust
// ✅ CanonRS
view! {
  <StatusDot variant=StatusDotVariant::Online />
}
```


---

## `switch`

- **Label:** Switch
- **Category:** Form
- **Intent:** Toggle between on and off states
- **Description:** Toggle switch on off
- **Pain:** Toggle inputs desync visual state and checked value
- **Promise:** Toggle state mapped directly to DOM and interaction state
- **Why:** SwitchPrimitive maps SelectionState to checked and data attributes. Disabled state is enforced consistently. This guarantees reliable toggle behavior.
- **Rules:** CR-001, CR-004
- **Use cases:** settings, toggles
- **Related:** toggle, toggle_group
- **Capabilities:** Disabled
- **States:** selected, disabled, focus
- **Boundary type:** init
- **Import:** `use canonrs::ui::switch::Switch;`

### Migration

```rust
// ❌ Typical
view! {
  <input type="checkbox" checked />
}
```

```rust
// ✅ CanonRS
view! {
  <Switch checked=true>"On"</Switch>
}
```


---

## `table`

- **Label:** Table
- **Category:** Data
- **Intent:** Display tabular data
- **Description:** HTML table component
- **Pain:** Tables lack consistent state handling and accessibility attributes
- **Promise:** Table state, sorting and selection enforced structurally
- **Why:** TablePrimitive encodes state, striped and hoverable behavior. Rows and headers derive selection and sort semantics. This guarantees consistent data table behavior.
- **Rules:** CR-001, CR-004
- **Use cases:** data grids, reports
- **Related:** data_table, virtual_list, empty_table, tree, list_item
- **Capabilities:** Selected
- **Required parts:** TableHeader, TableBody, TableRow, TableHead, TableCell
- **Optional parts:** TableFooter, TableCaption, TableWrapper
- **States:** loading, error, selected
- **Boundary type:** init
- **Import:** `use canonrs::ui::table::Table;`

### Migration

```rust
// ❌ Typical
view! {
  <table>
    <tr><td>"Data"</td></tr>
  </table>
}
```

```rust
// ✅ CanonRS
view! {
  <Table>
    <TableBody>
      <TableRow>
        <TableCell>"Data"</TableCell>
      </TableRow>
    </TableBody>
  </Table>
}
```


---

## `table-of-contents`

- **Label:** Table of Contents
- **Category:** Navigation
- **Intent:** Navigate document sections via anchors
- **Description:** Document table of contents
- **Pain:** TOC structures inconsistent and hard to sync with document hierarchy
- **Promise:** TOC hierarchy and state derived from structured data model
- **Why:** TocPrimitive encodes mode and item states with hierarchical structure. SSR rendering ensures deterministic output. This guarantees consistent navigation.
- **Rules:** CR-001, CR-004
- **Use cases:** docs navigation, long pages
- **Related:** tabs
- **States:** active, open, closed
- **Boundary type:** init
- **Import:** `use canonrs::ui::table_of_contents::TableOfContents;`

### Migration

```rust
// ❌ Typical
view! {
  <ul>
    <li><a href="#a">"A"</a></li>
  </ul>
}
```

```rust
// ✅ CanonRS
view! {
  <TableOfContents items=items />
}
```


---

## `tabs`

- **Label:** Tabs
- **Category:** Navigation
- **Intent:** Switch between related content panels
- **Description:** Tabbed navigation
- **Pain:** Tabs require manual state sync between triggers and panels
- **Promise:** Active state governs trigger and content without manual wiring
- **Why:** TabsPrimitive uses ActivityState to synchronize triggers and panels. ARIA roles and visibility are derived automatically. This guarantees consistent tab behavior.
- **Rules:** CR-001, CR-004
- **Use cases:** panel navigation, settings
- **Related:** table_of_contents
- **Capabilities:** Active
- **Required parts:** TabsList, TabsTrigger, TabsContent
- **States:** active, inactive, disabled
- **Boundary type:** interaction
- **Import:** `use canonrs::ui::tabs::Tabs;`

### Migration

```rust
// ❌ Typical
view! {
  <button>"Tab"</button>
  <div>"Content"</div>
}
```

```rust
// ✅ CanonRS
view! {
  <Tabs>
    <TabsList>
      <TabsTrigger value="a" active=true>"Tab"</TabsTrigger>
    </TabsList>
    <TabsContent value="a" active=true>"Content"</TabsContent>
  </Tabs>
}
```


---

## `textarea`

- **Label:** Textarea
- **Category:** Form
- **Intent:** Capture multi-line text from user
- **Description:** Multi-line text input
- **Pain:** Textarea misses required, readonly and aria attributes consistency
- **Promise:** All form states mapped directly to DOM and ARIA
- **Why:** TextareaPrimitive encodes disabled, readonly and required into both DOM and ARIA attributes. Label and description linkage is explicit. This guarantees accessible multi-line input behavior.
- **Rules:** CR-001, CR-004
- **Use cases:** comments, descriptions
- **Related:** form, input, input_group, input_otp, field, label, checkbox, form_error_summary
- **Capabilities:** Value, Disabled
- **States:** focus, disabled
- **Boundary type:** passthrough
- **Import:** `use canonrs::ui::textarea::Textarea;`

### Migration

```rust
// ❌ Typical
view! {
  <textarea placeholder="Type..."></textarea>
}
```

```rust
// ✅ CanonRS
view! {
  <Textarea placeholder="Type..." required=true />
}
```


---

## `toast`

- **Label:** Toast
- **Category:** Feedback
- **Intent:** Show brief non-blocking notifications
- **Description:** Toast notification message
- **Pain:** Notifications misuse urgency, role and lifecycle behavior
- **Promise:** Variant enforces correct role and aria-live automatically
- **Why:** ToastVariant defines role and aria-live behavior per type. Lifecycle and visibility are encoded structurally. This guarantees correct notification semantics and timing.
- **Rules:** CR-001, CR-004
- **Use cases:** system notifications, user feedback
- **Related:** alert, banner, callout, inline_notice, status_dot
- **Capabilities:** OpenClose
- **Required parts:** ToastViewport
- **Optional parts:** ToastTitle, ToastDescription, ToastAction, ToastClose
- **States:** open, closed
- **Boundary type:** init
- **Import:** `use canonrs::ui::toast::Toast;`

### Migration

```rust
// ❌ Typical
view! {
  <div class="toast error">"Error"</div>
}
```

```rust
// ✅ CanonRS
view! {
  <Toast variant=ToastVariant::Error>
    <ToastTitle>"Error"</ToastTitle>
  </Toast>
}
```


---

## `toggle`

- **Label:** Toggle
- **Category:** Form
- **Intent:** Toggle a pressed state
- **Description:** Toggle button
- **Pain:** Toggle buttons desync pressed state and visual representation
- **Promise:** Pressed state mapped directly to DOM and interaction state
- **Why:** TogglePrimitive maps ToggleState to data-rs-state and checkbox checked state. Disabled and aria-label are enforced. This guarantees consistent toggle behavior.
- **Rules:** CR-001, CR-004
- **Use cases:** formatting tools, feature toggles
- **Related:** switch, toggle_group
- **Capabilities:** Pressed, Disabled
- **States:** on, off, disabled
- **Boundary type:** init
- **Import:** `use canonrs::ui::toggle::Toggle;`

### Migration

```rust
// ❌ Typical
view! {
  <button class="active">"On"</button>
}
```

```rust
// ✅ CanonRS
view! {
  <Toggle pressed=true>"On"</Toggle>
}
```


---

## `toggle-group`

- **Label:** Toggle Group
- **Category:** Action
- **Intent:** Group of toggle buttons with single or multiple selection
- **Description:** Group of toggle buttons
- **Pain:** Grouped toggles lack exclusivity and shared disabled state
- **Promise:** Group behavior and selection mode enforced structurally
- **Why:** ToggleGroupPrimitive encodes multiple selection and disabled state at container level. Child toggles inherit behavior. This guarantees consistent grouped interactions.
- **Rules:** CR-001, CR-004
- **Use cases:** toolbars, option groups
- **Related:** switch, toggle
- **Capabilities:** Multiple, Disabled
- **States:** on, off, disabled
- **Boundary type:** interaction
- **Import:** `use canonrs::ui::toggle_group::ToggleGroup;`

### Migration

```rust
// ❌ Typical
view! {
  <div>
    <button>"A"</button>
    <button>"B"</button>
  </div>
}
```

```rust
// ✅ CanonRS
view! {
  <ToggleGroup multiple=false>
    <Toggle>"A"</Toggle>
  </ToggleGroup>
}
```


---

## `toolbar`

- **Label:** Toolbar
- **Category:** Layout
- **Intent:** Action toolbar region
- **Description:** Action toolbar component
- **Pain:** Action groups lack orientation and accessibility semantics
- **Promise:** Toolbar role and orientation enforced via contract
- **Why:** ToolbarPrimitive encodes orientation and role="toolbar". ARIA labeling is explicit. This guarantees accessible grouping of actions.
- **Rules:** CR-001, CR-004
- **Use cases:** editors, action bars
- **Related:** card, resizable, scroll_area, aspect_ratio, page_header, separator
- **States:** active
- **Boundary type:** interaction
- **Import:** `use canonrs::ui::toolbar::Toolbar;`

### Migration

```rust
// ❌ Typical
view! {
  <div class="toolbar">
    <button>"Bold"</button>
  </div>
}
```

```rust
// ✅ CanonRS
view! {
  <Toolbar aria_label="Editor">
    <button>"Bold"</button>
  </Toolbar>
}
```


---

## `tooltip`

- **Label:** Tooltip
- **Category:** Overlay
- **Intent:** Show brief label on hover/focus
- **Description:** Hover tooltip
- **Pain:** Tooltip requires id wiring and loses sync between trigger and content
- **Promise:** Trigger-content relationship enforced without manual wiring
- **Why:** TooltipPrimitive uses visibility state and DOM structure instead of ids. aria-describedby is optional but supported. This guarantees consistent overlay behavior.
- **Rules:** CR-001, CR-004
- **Use cases:** hints, help text
- **Related:** dialog, alert_dialog, drawer, sheet, modal, confirm_dialog, hover_card, popover
- **Capabilities:** OpenClose
- **Required parts:** TooltipTrigger, TooltipContent
- **Optional parts:** TooltipProvider
- **States:** open, closed
- **Boundary type:** init
- **Import:** `use canonrs::ui::tooltip::Tooltip;`

### Migration

```rust
// ❌ Typical
view! {
  <span title="Info">"Hover"</span>
}
```

```rust
// ✅ CanonRS
view! {
  <Tooltip>
    <TooltipTrigger>"Hover"</TooltipTrigger>
    <TooltipContent>"Info"</TooltipContent>
  </Tooltip>
}
```


---

## `tree`

- **Label:** Tree
- **Category:** Display
- **Intent:** Display hierarchical data
- **Description:** Tree view component
- **Pain:** Tree structures lose selection, expansion and focus consistency
- **Promise:** Hierarchy state fully governed via structured attributes
- **Why:** TreePrimitive and TreeItemPrimitive encode selection, focus and expansion via data attributes. ARIA roles are enforced. This guarantees accessible hierarchical navigation.
- **Rules:** CR-001, CR-004
- **Use cases:** file explorers, nested navigation
- **Related:** table, data_table, virtual_list, empty_table, list_item
- **Capabilities:** Selected
- **Required parts:** TreeItem
- **Optional parts:** TreeGroup
- **States:** expanded, collapsed, selected, disabled, active
- **Boundary type:** interaction
- **Import:** `use canonrs::ui::tree::Tree;`

### Migration

```rust
// ❌ Typical
view! {
  <ul>
    <li>"Item"</li>
  </ul>
}
```

```rust
// ✅ CanonRS
view! {
  <Tree>
    <TreeItem has_children=true>"Item"</TreeItem>
  </Tree>
}
```


---

## `virtual-list`

- **Label:** Virtual List
- **Category:** Display
- **Intent:** Efficiently render large lists
- **Description:** Virtualized list for large datasets
- **Pain:** Large lists render all items causing performance degradation
- **Promise:** List virtualization enforced structurally for large datasets
- **Why:** VirtualListPrimitive encodes list structure and item indexing. Viewport and content separation enable virtualization logic. This guarantees scalable rendering.
- **Rules:** CR-001, CR-004
- **Use cases:** logs, large datasets
- **Related:** table, data_table, empty_table, tree, list_item
- **Capabilities:** VirtualScroll
- **Boundary type:** interaction
- **Import:** `use canonrs::ui::virtual_list::VirtualList;`

### Migration

```rust
// ❌ Typical
view! {
  <ul>
    {items.map(|i| view! { <li>{i}</li> })}
  </ul>
}
```

```rust
// ✅ CanonRS
view! {
  <VirtualList items_count=1000 item_height=40.0>
    "Item"
  </VirtualList>
}
```


---

# CanonRS — Blocks

> AUTO-GENERATED — do not edit manually

---

## `page-header`

- **Label:** Page Header
- **Description:** Page title and actions header block
- **Category:** page

---

## `section`

- **Label:** Section
- **Description:** Generic content section block with header, body and footer regions
- **Category:** layout

---

## `sidebar-layout`

- **Label:** Sidebar Layout
- **Description:** Block-level sidebar and main content
- **Category:** layout

---

## `hero`

- **Label:** Hero
- **Description:** Page hero block with media, content and actions regions
- **Category:** page

---

## `form-field`

- **Label:** Form Field
- **Description:** Form field block with label, input, hint and error regions
- **Category:** form

---

## `card`

- **Label:** Card
- **Description:** Card block with header, content and footer regions
- **Category:** content

---

## `stat-group`

- **Label:** Stat Group
- **Description:** Dashboard stat group block for displaying metric collections
- **Category:** dashboard

---

## `data-table`

- **Label:** Data Table
- **Description:** Data table block with toolbar, column header, rows, empty state and pagination
- **Category:** dashboard

---

# CanonRS — Layouts

> AUTO-GENERATED — do not edit manually

---

## `page-layout`

- **Label:** Page Layout
- **Description:** Flexible page layout with optional sidebar, main content and aside regions
- **Category:** page

---

## `wizard-layout`

- **Label:** Wizard Layout
- **Description:** wizard-layout layout
- **Category:** flow

---

## `three-pane-layout`

- **Label:** Three Pane Layout
- **Description:** three-pane-layout layout
- **Category:** editor

---

## `split-view-layout`

- **Label:** Split View Layout
- **Description:** split-view-layout layout
- **Category:** editor

---

## `dashboard-layout`

- **Label:** Dashboard Layout
- **Description:** dashboard-layout layout
- **Category:** dashboard

---

## `marketing-layout`

- **Label:** Marketing Layout
- **Description:** marketing-layout layout
- **Category:** marketing

---

## `fullscreen-layout`

- **Label:** Fullscreen Layout
- **Description:** fullscreen-layout layout
- **Category:** overlay

---

# CanonRS — Layout Primitives

> AUTO-GENERATED — do not edit manually

---

## `stack`

- **Component:** `StackPrimitive`
- **Label:** Stack
- **Description:** Opiniated flex container for vertical or horizontal linear layouts
- **Use when:** Vertical or horizontal linear flex layout — use for most layouts
- **Tags:** stack, flex, vertical, horizontal, layout, gap

### Decision Guide

- Default choice for linear layouts
- Vertical by default
- Use `gap` for spacing
- No justify — use Flex if needed

---

## `flex`

- **Component:** `FlexPrimitive`
- **Label:** Flex
- **Description:** Free-form flex layout with full control over direction, justify, align, gap and wrap
- **Use when:** Full-control flex — use when Stack is not enough
- **Tags:** flex, layout, justify, align, gap, wrap, direction

### Decision Guide

- Escape hatch when Stack is not enough
- Full control: direction, justify, align, gap, wrap
- Use sparingly — prefer Stack

---

## `grid`

- **Component:** `GridPrimitive`
- **Label:** Grid
- **Description:** CSS grid container with configurable columns and gap for content grids
- **Use when:** CSS grid — use for multi-column content grids
- **Tags:** grid, layout, columns, gap, css-grid

### Decision Guide

- Use for multi-column content (cards, stats, thumbnails)
- Cols: 1,2,3,4,6,12,auto
- Not for page layout — use PageLayout

---

## `container`

- **Component:** `ContainerPrimitive`
- **Label:** Container
- **Description:** Max-width centered layout container with size variants
- **Use when:** Max-width centered wrapper — use at page level
- **Tags:** container, max-width, centered, layout, wrapper

### Decision Guide

- Always wrap page content with Container
- Size lg by default
- Full for edge-to-edge

---

## `center`

- **Component:** `CenterPrimitive`
- **Label:** Center
- **Description:** Centers content horizontally, vertically or both within its container
- **Use when:** Centers content — use for single centered elements
- **Tags:** center, align, horizontal, vertical, layout

### Decision Guide

- Use for single centered element (login, empty state, modal)
- Both by default (horizontal + vertical)

---

## `spacer`

- **Component:** `SpacerPrimitive`
- **Label:** Spacer
- **Description:** Auto-expanding spacer for pushing content apart in flex layouts
- **Use when:** Flex expander — use to push content apart
- **Tags:** spacer, flex, expand, push, layout

### Decision Guide

- Only inside Flex or Stack horizontal
- Pushes siblings apart
- No children

---

