# Canon Rule #123: Component Architecture Taxonomy and Contracts

**Status:** ENFORCED
**Severity:** CRITICAL
**Version:** 1.0.0
**Date:** 2025-01-22

**Category:** component-architecture
**Tags:** architecture, layers, contracts, design-system
**Language:** EN

---

**Intro:**
Lack of strict component classification leads to boundary leakage, inconsistent responsibilities, and architectural decay. Without taxonomy, primitives, UI, and domain logic mix unpredictably.

**Problem:**
components are not classified leading to boundary violations across layers

**Solution:**
enforce strict taxonomy with five layers and explicit contracts per component type

**Signals:**
- primitive with state
- ui with css tokens
- component confusion

**Search Intent:**
how to structure component architecture layers

**Keywords:**
component architecture layers ui, design system taxonomy, primitive ui component separation, frontend architecture contracts

---

## Principle

**Every component MUST be classified into exactly one architectural type, and MUST respect the contracts of its layer.**

---

## The Problem

Without explicit architectural boundaries, codebases degrade into:

1. **Classification chaos** — "Is this a primitive or UI component?"
2. **Contract violations** — Primitives with business logic, UI with CSS tokens
3. **Boundary leakage** — DOM events in application code, styling in primitives
4. **Onboarding friction** — New developers don't know where to put code
5. **Review ambiguity** — PRs lack objective validation criteria

**Real symptoms from production:**

- Primitives calling `use_context()` for state
- UI components defining CSS variables
- Application code using `on:click` instead of semantic callbacks
- Blocks managing global z-index
- Layouts with business logic

**Without this rule:**
- "Where does this go?" becomes a recurring question
- Architecture erodes through small violations
- Technical debt compounds invisibly
- Code reviews become subjective debates

---

## Architectural Layers

CanonRS defines **5 architectural layers**, each with distinct types and contracts:
```
┌─────────────────────────────────────────┐
│          5. Layout Layer                │  ← Shell, Zones
├─────────────────────────────────────────┤
│          4. Block Layer                 │  ← Semantic, Interactive
├─────────────────────────────────────────┤
│          3. Component Layer (Domain)    │  ← Stateful, Orchestrator
├─────────────────────────────────────────┤
│          2. UI Layer                    │  ← Adapter, Controlled, Composite
├─────────────────────────────────────────┤
│          1. Primitive Layer             │  ← Pure, Interactive, Container
└─────────────────────────────────────────┘
```

Each layer has:
- **Allowed responsibilities** (CAN)
- **Forbidden responsibilities** (CANNOT)
- **Input/output contracts**
- **Concrete types**

---

## Layer 1: Primitive Contracts

**Purpose:** Render semantic HTML with zero logic, zero styling, zero state.

### Critical Children Contract

**ALL Primitives MUST use `ChildrenFn` if children are rendered inside `view!`.**

This is non-negotiable and prevents 90% of `FnOnce` errors.
```rust
// ✅ CORRECT
#[component]
pub fn Primitive(children: ChildrenFn) -> impl IntoView {
    view! {
        <div>{children()}</div>
    }
}

// ❌ WRONG
#[component]
pub fn Primitive(children: Children) -> impl IntoView {
    view! {
        <div>{children()}</div>  // FnOnce error
    }
}
```

### Types

#### 1.1 PurePrimitive
**Definition:** Stateless HTML element with data attributes only.

**Examples:** `ButtonPrimitive`, `InputPrimitive`, `LabelPrimitive`

**CAN:**
- Render semantic HTML (`<button>`, `<input>`, `<label>`)
- Accept `ChildrenFn` (MANDATORY if rendering children)
- Accept text content props (String)
- Expose data attributes (`data-button`, `data-state`)
- Accept `Option<String>` for optional props (NO `into`)
- Pass through DOM events implicitly (Leptos handles this)

**CANNOT:**
- Have internal state (`RwSignal`, `StoredValue`)
- Call `use_context()` or access global state
- Define CSS (no classes, no inline styles, no tokens)
- Convert types with `#[prop(into)]` — FORBIDDEN
- Call `.unwrap_or_default()` — conversion is UI layer responsibility
- Emit semantic callbacks (`on_click`, `on_select`)
- Use `Children` type (MUST use `ChildrenFn`)
```rust
// ✅ CORRECT: PurePrimitive
#[component]
pub fn ButtonPrimitive(
    children: ChildrenFn,  // ✅ ChildrenFn, not Children
    #[prop(optional)] class: Option<String>,  // ✅ NO into
    #[prop(optional)] disabled: Option<bool>,  // ✅ NO into
) -> impl IntoView {
    view! {
        <button 
            data-button="" 
            class={class}  // ✅ Pass Option directly, no unwrap
            disabled={disabled}
        >
            {children()}
        </button>
    }
}
```
```rust
// ❌ WRONG: Type conversion in Primitive
#[component]
pub fn ButtonPrimitive(
    children: ChildrenFn,
    #[prop(optional, into)] class: Option<String>,  // ❌ NEVER use `into`
) -> impl IntoView {
    view! {
        <button class={class.unwrap_or_default()}>  // ❌ NEVER unwrap
            {children()}
        </button>
    }
}
```

#### 1.2 InteractivePrimitive
**Definition:** HTML form element that emits DOM events.

**Examples:** `CheckboxPrimitive`, `SelectPrimitive`, `RadioPrimitive`

**CAN:**
- Everything from PurePrimitive
- Accept `checked`, `value`, `disabled` props as `Option<T>` (NO `into`)
- Emit DOM events: `on:change`, `on:input`, `on:focus`

**CANNOT:**
- Transform event data (e.g., `event.target.checked → bool`)
- Manage validation state
- Have semantic callbacks
- Use `#[prop(into)]` for any prop
```rust
// ✅ CORRECT: InteractivePrimitive
#[component]
pub fn CheckboxPrimitive(
    #[prop(optional)] checked: Option<bool>,  // ✅ NO into
    #[prop(optional)] class: Option<String>,  // ✅ NO into
) -> impl IntoView {
    view! {
        <input 
            type="checkbox" 
            checked={checked}
            data-checkbox=""
            class={class}
        />
    }
}
```

#### 1.3 ContainerPrimitive
**Definition:** Structural wrapper with no visual semantics.

**Examples:** `DialogPrimitive`, `PopoverPrimitive`, `CardPrimitive`

**CAN:**
- Everything from PurePrimitive
- Accept multiple `ChildrenFn` slots (header, body, footer)
- Provide structural data attributes (`data-dialog-content`, `data-popover-trigger`)

**CANNOT:**
- Manage open/close state
- Handle keyboard navigation logic
- Position itself (no `position: fixed` logic)
- Use `#[prop(into)]`
```rust
// ✅ CORRECT: ContainerPrimitive
#[component]
pub fn DialogPrimitive(
    children: ChildrenFn,
    #[prop(optional)] open: Option<bool>,  // ✅ NO into
) -> impl IntoView {
    view! {
        <div data-dialog="" data-state={if open.unwrap_or(false) { "open" } else { "closed" }}>
            {children()}
        </div>
    }
}
```

---

## Layer 2: UI Component Contracts

**Purpose:** Provide ergonomic APIs, normalize props, bridge Primitive → Application.

### Critical Children Contract

**UI Components rendering children inside `view!` MUST use `ChildrenFn` wrapped in `StoredValue`.**
```rust
// ✅ CORRECT
#[component]
pub fn UIComponent(children: ChildrenFn) -> impl IntoView {
    let children = StoredValue::new(children);
    view! {
        <div>{move || children.with_value(|c| c())}</div>
    }
}
```

See **Canon Rule #121** for complete `StoredValue` guidance.

### Types

#### 2.1 AdapterUI
**Definition:** Wraps a Primitive, adds ergonomics, normalizes types.

**Examples:** `Button`, `Input`, `Label`

**CAN:**
- Call exactly one Primitive
- Accept `Option<T>` WITHOUT `#[prop(into)]` (Rule #119)
- Call `.unwrap_or_default()` on props before passing to Primitive
- Add variant/size props (`ButtonVariant`, `ButtonSize`)
- Set data attributes based on props (`data-variant="outline"`)
- Provide default values
- Use `Children` or `ChildrenFn` (invoke before `view!` if using `Children`)

**CANNOT:**
- Define CSS tokens or variables
- Call multiple Primitives (that's CompositeUI)
- Have business logic state
- Call `use_context()` for domain state
- Use `#[prop(optional, into)]` (forbidden by Rule #119)
```rust
// ✅ CORRECT: AdapterUI
#[component]
pub fn Button(
    children: Children,
    #[prop(default = ButtonVariant::Solid)] variant: ButtonVariant,
    #[prop(optional)] class: Option<String>,  // ✅ NO into
) -> impl IntoView {
    view! {
        <ButtonPrimitive 
            class=Some(format!("{} {}", variant.as_class(), class.unwrap_or_default()))
        >
            {children}  // ✅ Children invoked implicitly by Leptos
        </ButtonPrimitive>
    }
}
```

#### 2.2 ControlledUI
**Definition:** Manages visual state (loading, disabled, expanded).

**Examples:** `Accordion`, `Tabs`, `Select`

**CAN:**
- Have `RwSignal` for **visual state only** (not domain state)
- Emit semantic callbacks: `on_change`, `on_select`, `on_toggle`
- Type: `Callback<T>` where `T` is domain-specific (not `MouseEvent`)
- Manage keyboard navigation
- Sync state with props (controlled/uncontrolled pattern)
- Accept `Option<T>` WITHOUT `#[prop(into)]`

**CANNOT:**
- Fetch data or call APIs
- Access global application state
- Define layout (flex, grid, positioning)
- Use CSS classes for state (use data attributes)
- Use `#[prop(optional, into)]`
```rust
// ✅ CORRECT: ControlledUI
#[component]
pub fn Tabs(
    children: ChildrenFn,
    #[prop(default = 0)] default_tab: usize,
    on_change: Callback<usize>,
) -> impl IntoView {
    let (active_tab, set_active_tab) = signal(default_tab);  // ✅ Visual state
    
    Effect::new(move |_| {
        on_change.run(active_tab.get());  // ✅ Semantic callback
    });
    
    view! {
        <div data-tabs="" data-active={active_tab.get()}>
            {children()}
        </div>
    }
}
```

#### 2.3 CompositeUI
**Definition:** Composes multiple Primitives/UIs into a single component.

**Examples:** `Card`, `Dialog`, `Popover`, `Form`

**CAN:**
- Compose 2+ Primitives or UI components
- Provide slot-based APIs (header, body, footer)
- Coordinate child component state
- Accept `ChildrenFn` for flexible composition
- Accept `Option<T>` WITHOUT `#[prop(into)]`

**CANNOT:**
- Have business logic (validation, API calls)
- Know about domain models (`User`, `Order`, `Product`)
- Define page-level layout
- Use `#[prop(optional, into)]`
```rust
// ✅ CORRECT: CompositeUI
#[component]
pub fn Card(
    header: Option<ChildrenFn>,
    children: ChildrenFn,
    footer: Option<ChildrenFn>,
) -> impl IntoView {
    view! {
        <CardPrimitive>
            {header.map(|h| view! { <div data-card-header="">{h()}</div> })}
            <div data-card-body="">{children()}</div>
            {footer.map(|f| view! { <div data-card-footer="">{f()}</div> })}
        </CardPrimitive>
    }
}
```

---

## Layer 3: Component Domain Contracts

**Purpose:** Implement business logic, manage domain state, orchestrate workflows.

### Types

#### 3.1 StatefulComponent
**Definition:** Has business logic and domain state.

**Examples:** `LoginForm`, `ShoppingCart`, `UserProfile`

**CAN:**
- Use `RwSignal` for domain state
- Call APIs, fetch data
- Use `use_context()` for global state
- Validate data
- Emit domain events: `on_login`, `on_checkout`, `on_save`
- Use UI components from Layer 2
- Accept `Option<T>` props naturally

**CANNOT:**
- Use Primitives directly (must go through UI layer)
- Define CSS or data attributes
- Know about routing (use callbacks instead)
```rust
// ✅ CORRECT: StatefulComponent
#[component]
pub fn LoginForm(on_login: Callback<User>) -> impl IntoView {
    let (email, set_email) = signal(String::new());
    let (password, set_password) = signal(String::new());
    let (loading, set_loading) = signal(false);
    
    let handle_submit = move |_| {
        set_loading.set(true);
        // API call, validation, etc.
        on_login.run(user);  // ✅ Domain callback
    };
    
    view! {
        <form on:submit=handle_submit>
            <Input value=email on_input=move |v| set_email.set(v) />
            <Button on_click=handle_submit loading=loading.get()>
                "Login"
            </Button>
        </form>
    }
}
```

#### 3.2 OrchestratorComponent
**Definition:** Coordinates multiple StatefulComponents or complex workflows.

**Examples:** `CheckoutWizard`, `MultiStepForm`, `Dashboard`

**CAN:**
- Manage workflow state (step 1, 2, 3)
- Coordinate child components
- Handle cross-component communication
- Use `provide_context()` for child components

**CANNOT:**
- Render Primitives directly
- Define global layout (that's Layer 5)
```rust
// ✅ CORRECT: OrchestratorComponent
#[component]
pub fn CheckoutWizard() -> impl IntoView {
    let (step, set_step) = signal(1);
    let cart = use_context::<CartContext>().unwrap();
    
    view! {
        <div data-wizard="">
            <Show when=move || step.get() == 1>
                <CartReview on_next=move |_| set_step.set(2) />
            </Show>
            <Show when=move || step.get() == 2>
                <ShippingForm on_next=move |_| set_step.set(3) />
            </Show>
            <Show when=move || step.get() == 3>
                <PaymentForm on_submit=move |_| complete_order() />
            </Show>
        </div>
    }
}
```

---

## Layer 4: Block Contracts

**Purpose:** Define semantic page sections, compose Components into features.

### Types

#### 4.1 SemanticBlock
**Definition:** Structural page section with semantic meaning.

**Examples:** `HeroBlock`, `FooterBlock`, `SidebarBlock`, `FeatureGridBlock`

**CAN:**
- Use `<section>`, `<header>`, `<footer>`, `<aside>`, `<nav>`
- Compose UI components and domain components
- Define block-level structure (not page-level)
- Accept content via props

**CANNOT:**
- Manage application routing
- Have global state (cart, user session)
- Define z-index or position (that's Layout layer)
```rust
// ✅ CORRECT: SemanticBlock
#[component]
pub fn HeroBlock(
    title: String,
    subtitle: String,
    cta: ChildrenFn,
) -> impl IntoView {
    view! {
        <section data-hero="">
            <h1>{title}</h1>
            <p>{subtitle}</p>
            <div data-hero-cta="">
                {cta()}
            </div>
        </section>
    }
}
```

#### 4.2 InteractiveBlock
**Definition:** Feature-complete interactive section.

**Examples:** `CommentSectionBlock`, `SearchBlock`, `FilterBarBlock`

**CAN:**
- Combine multiple domain components
- Manage block-level interactions
- Emit block-level events: `on_filter_change`, `on_comment_submit`

**CANNOT:**
- Know about page structure (header position, sidebar width)
- Define page transitions
```rust
// ✅ CORRECT: InteractiveBlock
#[component]
pub fn CommentSectionBlock(
    post_id: String,
    on_comment: Callback<Comment>,
) -> impl IntoView {
    let (comments, set_comments) = signal(vec![]);
    
    view! {
        <section data-comments="">
            <h3>"Comments"</h3>
            <CommentList comments=comments />
            <CommentForm on_submit=move |c| {
                set_comments.update(|cs| cs.push(c.clone()));
                on_comment.run(c);
            } />
        </section>
    }
}
```

---

## Layer 5: Layout Contracts

**Purpose:** Define page structure, manage stacking context, control global positioning.

### Types

#### 5.1 ShellLayout
**Definition:** Application-level layout (header, sidebar, main, footer).

**Examples:** `AppShell`, `DashboardShell`, `MarketingShell`

**CAN:**
- Define top-level structure (`<header>`, `<aside>`, `<main>`, `<footer>`)
- Manage global z-index layers
- Control responsive breakpoints for layout
- Provide context for child routes

**CANNOT:**
- Have business logic
- Fetch data or call APIs
- Render domain components directly (use `<Outlet />` or slots)
```rust
// ✅ CORRECT: ShellLayout
#[component]
pub fn AppShell() -> impl IntoView {
    view! {
        <div data-shell="">
            <header data-shell-header="">
                <NavBar />
            </header>
            <aside data-shell-sidebar="">
                <Sidebar />
            </aside>
            <main data-shell-main="">
                <Outlet />  // ✅ Router content
            </main>
        </div>
    }
}
```

#### 5.2 ZoneLayout
**Definition:** Page-level layout regions (grid, flexbox zones).

**Examples:** `TwoColumnLayout`, `DashboardGridLayout`, `SplitLayout`

**CAN:**
- Define CSS Grid or Flexbox structure
- Accept `ChildrenFn` for zones
- Manage responsive layout changes

**CANNOT:**
- Know what components are in zones
- Have state management
```rust
// ✅ CORRECT: ZoneLayout
#[component]
pub fn TwoColumnLayout(
    left: ChildrenFn,
    right: ChildrenFn,
) -> impl IntoView {
    view! {
        <div data-layout="two-column">
            <div data-zone="left">{left()}</div>
            <div data-zone="right">{right()}</div>
        </div>
    }
}
```

---

## Forbidden Cross-Layer Violations

### ❌ Primitive calling UI Component
```rust
// WRONG
#[component]
pub fn ButtonPrimitive() -> impl IntoView {
    view! {
        <button>
            <Icon name="check" />  // ❌ UI component in Primitive
        </button>
    }
}
```

### ❌ Primitive with type conversion
```rust
// WRONG
#[component]
pub fn ButtonPrimitive(
    #[prop(optional, into)] class: Option<String>,  // ❌ NEVER use `into`
) -> impl IntoView {
    view! {
        <button class={class.unwrap_or_default()}>  // ❌ NEVER unwrap
            "Button"
        </button>
    }
}
```

### ❌ UI Component with CSS tokens
```rust
// WRONG
#[component]
pub fn Button() -> impl IntoView {
    view! {
        <button style="background: var(--color-primary)">  // ❌ CSS in UI
            "Click"
        </button>
    }
}
```

### ❌ Component using Primitive directly
```rust
// WRONG
#[component]
pub fn LoginForm() -> impl IntoView {
    view! {
        <form>
            <ButtonPrimitive>  // ❌ Should use Button (UI layer)
                "Submit"
            </ButtonPrimitive>
        </form>
    }
}
```

### ❌ Block managing routing
```rust
// WRONG
#[component]
pub fn HeroBlock() -> impl IntoView {
    let navigate = use_navigate();  // ❌ Routing in Block
    
    view! {
        <section>
            <button on:click=move |_| navigate("/signup")>"Sign Up"</button>
        </section>
    }
}
```

---

## Rationale

### Why This Rule Exists

1. **Objective classification:**
   - "Where does this go?" has a definitive answer
   - Code review becomes rule validation, not opinion

2. **Boundary enforcement:**
   - Prevents responsibility leakage
   - Maintains separation of concerns
   - Enables independent evolution of layers

3. **Onboarding velocity:**
   - New developers learn taxonomy once
   - Code location becomes predictable
   - PR structure follows architecture

4. **Long-term maintainability:**
   - Violations are detectable
   - Refactoring has clear targets
   - Technical debt is visible

### What This Rule Protects

- **Architectural integrity** — layers stay separated
- **Code discoverability** — predictable file structure
- **Refactoring safety** — clear contracts to maintain
- **Team scalability** — shared mental model
- **Type safety** — proper prop handling prevents E0308 cascades
- **Reactivity correctness** — proper Children types prevent FnOnce errors

---

## Enforcement

### Static Analysis
```rust
// Validator pseudocode
for component in all_components {
    let layer = classify_by_path(component);
    let violations = check_contracts(component, layer);
    
    if !violations.is_empty() {
        emit_error!(
            "Canon Rule #123: Component '{}' in {} layer violates contracts: {:?}",
            component.name,
            layer,
            violations
        );
    }
}
```

### File Structure Convention
```
packages-rust/rs-design/src/
├── primitives/          # Layer 1
│   ├── button.rs        # PurePrimitive
│   ├── checkbox.rs      # InteractivePrimitive
│   └── dialog.rs        # ContainerPrimitive
├── ui/                  # Layer 2
│   ├── button.rs        # AdapterUI
│   ├── tabs.rs          # ControlledUI
│   └── card.rs          # CompositeUI
├── components/          # Layer 3 (domain)
│   ├── login_form.rs    # StatefulComponent
│   └── checkout.rs      # OrchestratorComponent
├── blocks/              # Layer 4
│   ├── hero.rs          # SemanticBlock
│   └── comments.rs      # InteractiveBlock
└── layouts/             # Layer 5
    ├── app_shell.rs     # ShellLayout
    └── two_column.rs    # ZoneLayout
```

### CI Checks
```bash
# Check 1: Primitives have no state
rg 'RwSignal|use_context' packages-rust/rs-design/src/primitives/ && exit 1

# Check 2: Primitives never use #[prop(into)]
rg '#\[prop\(.*into.*\)' packages-rust/rs-design/src/primitives/ && exit 1

# Check 3: UI layer never uses #[prop(optional, into)]
rg '#\[prop\(optional.*into\)' packages-rust/rs-design/src/ui/ && exit 1

# Check 4: Primitives use ChildrenFn, not Children
rg 'children: Children[^F]' packages-rust/rs-design/src/primitives/ && exit 1

# Check 5: Components don't use Primitives directly
rg 'ButtonPrimitive|InputPrimitive' products/*/src/components/ && exit 1

# Check 6: Blocks don't have routing
rg 'use_navigate|use_location' packages-rust/rs-design/src/blocks/ && exit 1
```

---

## Exceptions

**No exceptions. This rule is absolute.**

Every component must fit into exactly one type. If a component seems to span multiple types, it's misclassified and should be split.

---

## Quick Reference Table

| Layer | Types | Children Type | `#[prop(into)]` | CAN | CANNOT |
|-------|-------|---------------|-----------------|-----|--------|
| **1. Primitive** | Pure, Interactive, Container | `ChildrenFn` ONLY | ❌ NEVER | Render HTML, data attrs | State, CSS, unwrap, conversion |
| **2. UI** | Adapter, Controlled, Composite | `Children` or `ChildrenFn` | ❌ NEVER | Normalize props, unwrap, semantic events | CSS tokens, domain logic, `#[prop(into)]` |
| **3. Component** | Stateful, Orchestrator | Any | ✅ Allowed | Domain state, API calls | Use Primitives directly |
| **4. Block** | Semantic, Interactive | Any | ✅ Allowed | Compose components, section structure | Global state, routing |
| **5. Layout** | Shell, Zone | Any | ✅ Allowed | Define structure, z-index | Business logic, data fetching |

---

## Related Rules

- **Canon Rule #75:** Primitives have zero styling
- **Canon Rule #89:** Primitives are SSR-safe
- **Canon Rule #119:** No `#[prop(optional, into)]` in UI layer
- **Canon Rule #120:** DOM Events vs Semantic Callbacks
- **Canon Rule #121:** StoredValue for Non-Copy in view!
- **Canon Rule #122:** No Conditional Rendering with .then()

---

## Version History

- **1.0.0** — Initial canonical version (2025-01-22)
  - Added explicit `ChildrenFn` requirement for Primitives
  - Prohibited `#[prop(into)]` in Primitives completely
  - Clarified type conversion responsibilities per layer