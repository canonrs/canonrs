# Canon Rule #125: UI Component Contracts

**Status:** ENFORCED
**Severity:** CRITICAL
**Version:** 1.0.0
**Date:** 2025-01-22

**Category:** component-architecture
**Tags:** ui, contracts, callbacks, state
**Language:** EN

---

**Intro:**
UI components without strict contracts leak primitives, mix domain logic, and create type ambiguity. This results in poor ergonomics and inconsistent APIs across the design system.

**Problem:**
ui components mix responsibilities and use ambiguous prop patterns

**Solution:**
enforce adapter controlled and composite ui types with strict callback and prop rules

**Signals:**
- prop optional into
- mouseevent in api
- primitive leakage

**Search Intent:**
how to design ui component contracts

**Keywords:**
ui component patterns adapter controlled, leptos callback design, ui abstraction layer design, component api ergonomics

---

## Principle

**UI components bridge Primitives and Application code by normalizing props, managing visual state, and emitting semantic callbacks. Every UI component MUST be classified as Adapter, Controlled, or Composite.**

---

## The Problem

Without formal UI layer types, developers create components that:

1. Use Primitives directly in domain code (skipping UI layer)
2. Mix visual state with domain state
3. Define CSS tokens instead of using design system
4. Emit DOM events (`MouseEvent`) instead of semantic callbacks
5. Use `#[prop(optional, into)]` causing type ambiguity

**Real symptoms:**
- E0308 errors cascading across call sites
- FnOnce errors from improper `Children` usage
- Primitive props leaking into application code
- CSS defined in component instead of tokens

**Without this rule:**
- UI layer becomes ad-hoc collection of wrappers
- No clear boundary between UI and domain
- Code reviews debate "is this UI or Component?"

---

## UI Component Types

### Type 1: AdapterUI

**Definition:** Wraps exactly ONE Primitive, adds ergonomics, normalizes types.

**Use when:** Making Primitive API more convenient without adding logic.

**Examples:** `Button`, `Input`, `Label`, `Checkbox`

#### CAN:
- Wrap exactly one Primitive component
- Accept `Option<T>` props WITHOUT `#[prop(into)]` (Rule #119)
- Call `.unwrap_or_default()` on props before passing to Primitive
- Add variant/size enums (`ButtonVariant::Solid`, `InputSize::Md`)
- Set data attributes based on variants (`data-variant="outline"`)
- Use `Children` (invoke before `view!`) or `ChildrenFn` (with `StoredValue`)
- Provide sensible defaults for optional props

#### CANNOT:
- Call multiple Primitives (that's CompositeUI)
- Have visual state (`RwSignal` for loading, expanded, etc.)
- Emit DOM events to users (`on:click` in public API)
- Define CSS tokens or inline styles
- Call `use_context()` for domain state
- Use `#[prop(optional, into)]` (forbidden by Rule #119)

#### Canonical Example:
```rust
// ✅ CORRECT: AdapterUI
#[component]
pub fn Button(
    children: Children,
    #[prop(default = ButtonVariant::Solid)] variant: ButtonVariant,
    #[prop(default = ButtonSize::Md)] size: ButtonSize,
    #[prop(optional)] class: Option<String>,  // ✅ NO into
    #[prop(optional)] disabled: Option<bool>,
) -> impl IntoView {
    let class_val = format!(
        "{} {}",
        class.unwrap_or_default(),
        variant.as_class()
    );
    
    view! {
        <ButtonPrimitive
            class=Some(class_val)
            disabled=disabled
        >
            {children}
        </ButtonPrimitive>
    }
}
```

#### Forbidden Example:
```rust
// ❌ WRONG: Multiple Primitives in AdapterUI
#[component]
pub fn Button(children: Children) -> impl IntoView {
    view! {
        <ButtonPrimitive>  // ❌ Calling 2 primitives
            <SpanPrimitive>{children}</SpanPrimitive>
        </ButtonPrimitive>
    }
}
```

---

### Type 2: ControlledUI

**Definition:** Manages visual state and emits semantic callbacks.

**Use when:** Component needs internal state for UI concerns (tabs, accordions, dropdowns).

**Examples:** `Tabs`, `Accordion`, `Select`, `Combobox`

#### CAN:
- Have `RwSignal` for **visual state only** (active tab, expanded, open)
- Emit semantic callbacks: `on_change`, `on_select`, `on_toggle`
  - Type: `Callback<T>` where `T` is domain-specific (`String`, `usize`, `MyEnum`)
  - NEVER `Callback<MouseEvent>` or other DOM types
- Manage keyboard navigation (arrow keys, enter, escape)
- Sync state with props (controlled/uncontrolled pattern)
- Accept `Option<T>` props WITHOUT `#[prop(into)]`
- Use `ChildrenFn` with `StoredValue` pattern (Rule #121)

#### CANNOT:
- Have domain state (user data, API responses, cart contents)
- Fetch data or call APIs
- Use `use_context()` for application state
- Define page layout (flex, grid positioning)
- Use CSS classes for state (use data attributes)
- Use `#[prop(optional, into)]`
- Emit DOM events in public API

#### Canonical Example:
```rust
// ✅ CORRECT: ControlledUI
#[component]
pub fn Tabs(
    children: ChildrenFn,
    #[prop(default = 0)] default_tab: usize,
    on_change: Callback<usize>,  // ✅ Semantic callback
) -> impl IntoView {
    let (active_tab, set_active_tab) = signal(default_tab);  // ✅ Visual state
    
    Effect::new(move |_| {
        on_change.run(active_tab.get());  // ✅ Emit semantic event
    });
    
    view! {
        <div data-tabs="" data-active={active_tab.get()}>
            {children()}
        </div>
    }
}
```

#### Forbidden Example:
```rust
// ❌ WRONG: Domain state in ControlledUI
#[component]
pub fn UserTabs() -> impl IntoView {
    let user = use_context::<User>().unwrap();  // ❌ Domain state
    let (active, set_active) = signal(0);
    
    view! {
        <div data-tabs="">
            <button on:click=move |_| set_active.set(0)>
                {user.name}  // ❌ Using domain data
            </button>
        </div>
    }
}
```

---

### Type 3: CompositeUI

**Definition:** Composes 2+ Primitives or UI components into cohesive unit.

**Use when:** Multiple primitives form a single logical component (cards, forms, modals).

**Examples:** `Card`, `Dialog`, `Popover`, `Form`, `Alert`

#### CAN:
- Compose 2+ Primitives OR UI components
- Provide slot-based APIs (header, body, footer)
- Accept multiple `ChildrenFn` for different slots
- Coordinate child component state (which slot is active)
- Accept `Option<T>` props WITHOUT `#[prop(into)]`
- Use `ChildrenFn` for all slots

#### CANNOT:
- Have business logic (validation, API calls, domain rules)
- Know about domain models (`User`, `Product`, `Order`)
- Manage routing or navigation
- Fetch data
- Use `#[prop(optional, into)]`

#### Canonical Example:
```rust
// ✅ CORRECT: CompositeUI
#[component]
pub fn Card(
    header: Option<ChildrenFn>,
    children: ChildrenFn,
    footer: Option<ChildrenFn>,
    #[prop(optional)] class: Option<String>,  // ✅ NO into
) -> impl IntoView {
    view! {
        <CardPrimitive class=class>
            {header.map(|h| view! {
                <div data-card-header="">{h()}</div>
            })}
            <div data-card-body="">{children()}</div>
            {footer.map(|f| view! {
                <div data-card-footer="">{f()}</div>
            })}
        </CardPrimitive>
    }
}
```

#### Forbidden Example:
```rust
// ❌ WRONG: Domain logic in CompositeUI
#[component]
pub fn UserCard(user_id: String) -> impl IntoView {
    let user = fetch_user(user_id);  // ❌ API call in UI layer
    
    view! {
        <CardPrimitive>
            <div>{user.name}</div>  // ❌ Domain data
        </CardPrimitive>
    }
}
```

---

## Universal UI Contracts

**ALL UI components MUST follow these contracts:**

### 1. No `#[prop(optional, into)]`
**NEVER combine `optional` and `into` (Rule #119).**
```rust
// ✅ CORRECT
#[prop(optional)] class: Option<String>

// ❌ WRONG
#[prop(optional, into)] class: Option<String>
```

### 2. Semantic Callbacks Only
**Emit `Callback<T>` where `T` is domain type — NEVER `MouseEvent`.**
```rust
// ✅ CORRECT
on_click: Callback<()>
on_select: Callback<String>
on_change: Callback<usize>

// ❌ WRONG
on_click: Callback<MouseEvent>  // DOM event in UI layer
```

### 3. Children Handling
**Use `Children` (invoke before `view!`) or `ChildrenFn` (with `StoredValue`).**
```rust
// ✅ Option 1: Children (simple)
#[component]
pub fn Button(children: Children) -> impl IntoView {
    view! {
        <ButtonPrimitive>{children}</ButtonPrimitive>
    }
}

// ✅ Option 2: ChildrenFn (reactive)
#[component]
pub fn Tabs(children: ChildrenFn) -> impl IntoView {
    let children = StoredValue::new(children);
    view! {
        <div>{move || children.with_value(|c| c())}</div>
    }
}
```

### 4. Visual State Only
**`RwSignal` for UI concerns only — never domain state.**
```rust
// ✅ CORRECT: Visual state
let (active_tab, set_active_tab) = signal(0);
let (loading, set_loading) = signal(false);

// ❌ WRONG: Domain state
let (user, set_user) = signal(User::default());
let (cart_items, set_cart_items) = signal(vec![]);
```

### 5. No Direct Primitive Usage in Application
**Application code uses UI layer — never Primitives directly.**
```rust
// ✅ CORRECT: Application uses UI component
view! {
    <Button on_click=save>"Save"</Button>
}

// ❌ WRONG: Application uses Primitive
view! {
    <ButtonPrimitive on:click=save>"Save"</ButtonPrimitive>
}
```

---

## Rationale

### Why UI Layer Exists

1. **Abstraction boundary** — hides Primitive implementation details
2. **Ergonomics** — provides defaults, variants, convenience
3. **Type safety** — normalizes props, eliminates `.into()` ambiguity
4. **Semantic events** — converts DOM → business intent
5. **Visual state management** — keeps UI concerns separate from domain

### What This Rule Protects

- **Call-site ergonomics** — users write natural code
- **Type clarity** — no ambiguous conversions
- **Separation of concerns** — UI ≠ domain ≠ primitives
- **Reactivity correctness** — proper Children types prevent errors

---

## Enforcement

### Static Analysis
```rust
// Check 1: No #[prop(optional, into)]
for prop in ui_component.props {
    if prop.has_attributes(&["optional", "into"]) {
        emit_error!("Canon Rule #125: UI cannot use #[prop(optional, into)]");
    }
}

// Check 2: No DOM events in callbacks
for prop in ui_component.props {
    if prop.type.contains("Callback<MouseEvent>") {
        emit_error!("Canon Rule #125: Use Callback<()>, not Callback<MouseEvent>");
    }
}
```

### CI Checks
```bash
# No #[prop(optional, into)] in UI layer
rg '#\[prop\(optional.*into\)' packages-rust/rs-design/src/ui/ && exit 1

# No domain state patterns
rg 'use_context::<User>|fetch_|api::' packages-rust/rs-design/src/ui/ && exit 1
```

---

## Exceptions

**No exceptions. This rule is absolute.**

If a UI component needs domain logic or API calls, it's misclassified — move to Component layer (Rule #126).

---

## Quick Reference

| Type | Wraps | State | Callbacks | Example |
|------|-------|-------|-----------|---------|
| **Adapter** | 1 Primitive | ❌ None | ❌ None | `Button`, `Input` |
| **Controlled** | 1+ Primitives | ✅ Visual only | ✅ Semantic | `Tabs`, `Select` |
| **Composite** | 2+ Primitives/UI | ❌ None | ❌ None | `Card`, `Dialog` |

---

## Related Rules

- **Canon Rule #119:** No `#[prop(optional, into)]` in UI layer
- **Canon Rule #120:** DOM Events vs Semantic Callbacks
- **Canon Rule #121:** StoredValue for Non-Copy in view!
- **Canon Rule #124:** Primitive Contract Types (previous rule)
- **Canon Rule #126:** Component Domain Contracts (next rule)

---

## Version History

- **1.0.0** — Initial canonical version (2025-01-22)