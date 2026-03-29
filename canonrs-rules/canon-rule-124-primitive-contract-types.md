# Canon Rule #124: Primitive Contract Types

**Status:** ENFORCED
**Severity:** CRITICAL
**Scope:** primitives
**Version:** 1.0.0
**Date:** 2025-01-22

---

## Principle

**Primitives render semantic HTML with zero logic, zero state, zero styling. Every Primitive MUST be classified as Pure, Interactive, or Container.**

---

## The Problem

Without formal primitive types, developers create "hybrid primitives" that:

1. Mix DOM rendering with state management
2. Apply CSS directly instead of data attributes
3. Convert types (`.into()`, `unwrap_or_default()`) instead of passing through
4. Emit semantic callbacks instead of raw DOM events

**Real symptoms:**
- Primitives calling `use_context()`
- Primitives with `RwSignal`
- Primitives using `#[prop(into)]` causing type cascades
- Primitives with business logic

**Without this rule:**
- Primitive layer becomes polluted with UI concerns
- SSR safety breaks (state, browser APIs)
- Hydration mismatches emerge
- Testing becomes dependent on environment

---

## Primitive Types

### Type 1: PurePrimitive

**Definition:** Stateless HTML element with pass-through props only.

**Use when:** Rendering semantic HTML without interaction or state.

**Examples:** `ButtonPrimitive`, `LabelPrimitive`, `DivPrimitive`, `SpanPrimitive`

#### CAN:
- Render one semantic HTML element (`<button>`, `<label>`, `<div>`)
- Accept `ChildrenFn` (MANDATORY if children rendered)
- Accept `Option<String>` for optional props (class, id, aria-*)
- Expose data attributes (`data-button=""`, `data-state="loading"`)
- Pass props directly to HTML without transformation

#### CANNOT:
- Have internal state (`RwSignal`, `create_signal`, `StoredValue`)
- Call `use_context()` or any context provider
- Use `#[prop(into)]` on ANY prop
- Call `.unwrap_or_default()` or `.unwrap()`
- Define CSS (inline styles, classes, tokens)
- Emit semantic callbacks (`on_click: Callback<()>`)
- Use `Children` type (MUST use `ChildrenFn`)

#### Canonical Example:
```rust
// ✅ CORRECT: PurePrimitive
#[component]
pub fn ButtonPrimitive(
    children: ChildrenFn,  // ✅ ChildrenFn mandatory
    #[prop(optional)] class: Option<String>,  // ✅ NO into
    #[prop(optional)] disabled: Option<bool>,  // ✅ Pass through
    #[prop(optional)] r#type: Option<String>,
) -> impl IntoView {
    view! {
        <button
            data-button=""
            class={class}  // ✅ Pass Option directly
            disabled={disabled}
            type={r#type}
        >
            {children()}
        </button>
    }
}
```

#### Forbidden Example:
```rust
// ❌ WRONG: Type conversion in Primitive
#[component]
pub fn ButtonPrimitive(
    children: ChildrenFn,
    #[prop(optional, into)] class: Option<String>,  // ❌ NEVER use into
) -> impl IntoView {
    view! {
        <button class={class.unwrap_or_default()}>  // ❌ NEVER unwrap
            {children()}
        </button>
    }
}
```

---

### Type 2: InteractivePrimitive

**Definition:** HTML form element that emits raw DOM events.

**Use when:** User input is captured (forms, selections, toggles).

**Examples:** `InputPrimitive`, `CheckboxPrimitive`, `SelectPrimitive`, `TextareaPrimitive`

#### CAN:
- Everything from PurePrimitive
- Render form elements (`<input>`, `<select>`, `<textarea>`)
- Accept `value`, `checked`, `selected` as `Option<T>`
- Emit raw DOM events implicitly (`on:change`, `on:input`, `on:focus`)
- Accept `name`, `placeholder`, `autocomplete` props

#### CANNOT:
- Transform event values (`event.target.value → String`)
- Manage validation state
- Emit semantic callbacks (`on_change: Callback<String>`)
- Call `.unwrap()` or `.into()` on any prop
- Have focus management logic

#### Canonical Example:
```rust
// ✅ CORRECT: InteractivePrimitive
#[component]
pub fn InputPrimitive(
    #[prop(optional)] value: Option<String>,  // ✅ NO into
    #[prop(optional)] placeholder: Option<String>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] r#type: Option<String>,
) -> impl IntoView {
    view! {
        <input
            data-input=""
            value={value}
            placeholder={placeholder}
            disabled={disabled}
            type={r#type}
        />
    }
}
```

#### Forbidden Example:
```rust
// ❌ WRONG: Semantic callback in Primitive
#[component]
pub fn InputPrimitive(
    on_change: Callback<String>,  // ❌ Semantic callback forbidden
) -> impl IntoView {
    view! {
        <input on:input=move |ev| {
            on_change.run(event_target_value(&ev));  // ❌ Transformation
        } />
    }
}
```

---

### Type 3: ContainerPrimitive

**Definition:** Structural wrapper providing slots without visual semantics.

**Use when:** Grouping content with semantic structure (dialogs, cards, popovers).

**Examples:** `DialogPrimitive`, `PopoverPrimitive`, `CardPrimitive`, `AccordionItemPrimitive`

#### CAN:
- Everything from PurePrimitive
- Accept multiple `ChildrenFn` slots (header, body, footer)
- Render container elements (`<div>`, `<section>`, `<article>`)
- Provide structural data attributes (`data-dialog-content`, `data-card-header`)
- Accept `open`, `expanded`, `collapsed` as `Option<bool>`

#### CANNOT:
- Manage open/close state internally
- Handle keyboard navigation (that's UI layer)
- Position itself (`position: fixed` logic)
- Trap focus or manage z-index
- Emit semantic events (`on_open`, `on_close`)

#### Canonical Example:
```rust
// ✅ CORRECT: ContainerPrimitive
#[component]
pub fn DialogPrimitive(
    children: ChildrenFn,
    #[prop(optional)] open: Option<bool>,
) -> impl IntoView {
    view! {
        <div
            data-dialog=""
            data-state={open.and_then(|v| if v { Some("open") } else { Some("closed") })}
            role="dialog"
        >
            {children()}
        </div>
    }
}
```

#### Forbidden Example:
```rust
// ❌ WRONG: State management in Primitive
#[component]
pub fn DialogPrimitive(
    children: ChildrenFn,
) -> impl IntoView {
    let (open, set_open) = signal(false);  // ❌ State in Primitive
    
    view! {
        <div data-dialog="" data-state={if open.get() { "open" } else { "closed" }}>
            {children()}
        </div>
    }
}
```

---

## Universal Primitive Contracts

**ALL Primitives MUST follow these contracts regardless of type:**

### 1. Children Contract
**If children are rendered inside `view!`, MUST use `ChildrenFn`.**
```rust
// ✅ CORRECT
children: ChildrenFn

// ❌ WRONG
children: Children  // Causes FnOnce errors
```

### 2. No Type Conversion
**NEVER use `#[prop(into)]` on any prop.**
```rust
// ✅ CORRECT
#[prop(optional)] class: Option<String>

// ❌ WRONG
#[prop(optional, into)] class: Option<String>
```

### 3. No Unwrapping
**Pass `Option<T>` directly to HTML — no `.unwrap()` or `.unwrap_or_default()`.**
```rust
// ✅ CORRECT
<button class={class}>

// ❌ WRONG
<button class={class.unwrap_or_default()}>
```

### 4. No State
**Zero internal state — no `RwSignal`, `StoredValue`, `use_context()`.**

### 5. No Styling
**Zero CSS — no inline styles, no CSS classes (only pass-through), no tokens.**

### 6. Data Attributes Only
**Expose state via data attributes, not CSS classes.**
```rust
// ✅ CORRECT
data-state={disabled.and_then(|v| if v { Some("disabled") } else { Some("enabled") })}

// ❌ WRONG
class={disabled.and_then(|v| if v { Some("btn-disabled") } else { Some("btn-enabled") })}
```

---

## Rationale

### Why Primitives Exist

1. **SSR Safety:** No browser APIs, no client-only state
2. **Hydration Predictability:** No conditional logic affecting structure
3. **Zero Coupling:** Can be used in any context without dependencies
4. **Testing Simplicity:** Pure functions, no mocking needed
5. **Type Clarity:** No `.into()` ambiguity

### What This Rule Protects

- **SSR/Hydration contracts** — primitives render identically server/client
- **Type inference** — no `.into()` cascades
- **Architectural boundaries** — primitives never know about UI/domain
- **Reactivity correctness** — `ChildrenFn` prevents `FnOnce` errors

---

## Enforcement

### Static Analysis
```rust
// Check 1: No #[prop(into)]
for prop in primitive.props {
    if prop.has_attribute("into") {
        emit_error!("Canon Rule #124: Primitives cannot use #[prop(into)]");
    }
}

// Check 2: ChildrenFn required
for prop in primitive.props {
    if prop.name == "children" && prop.type != "ChildrenFn" {
        emit_error!("Canon Rule #124: Primitives must use ChildrenFn, not Children");
    }
}

// Check 3: No state
if primitive.contains("RwSignal") || primitive.contains("use_context") {
    emit_error!("Canon Rule #124: Primitives cannot have state");
}
```

### CI Checks
```bash
# No #[prop(into)] in primitives
rg '#\[prop\(.*into.*\)' packages-rust/rs-design/src/primitives/ && exit 1

# No state in primitives
rg 'RwSignal|create_signal|use_context' packages-rust/rs-design/src/primitives/ && exit 1

# ChildrenFn only (not Children)
rg 'children: Children[^F]' packages-rust/rs-design/src/primitives/ && exit 1
```

---

## Exceptions

**No exceptions. This rule is absolute.**

If a primitive needs state, type conversion, or callbacks, it's misclassified — move it to UI layer.

---

## Quick Reference

| Aspect | Pure | Interactive | Container |
|--------|------|-------------|-----------|
| HTML Element | `<button>`, `<div>` | `<input>`, `<select>` | `<div>` with slots |
| Children | `ChildrenFn` | N/A | `ChildrenFn` (multiple) |
| DOM Events | Implicit | `on:change`, `on:input` | Implicit |
| State Props | ❌ None | `value`, `checked` | `open`, `expanded` |
| Semantic Callbacks | ❌ Never | ❌ Never | ❌ Never |

---

## Related Rules

- **Canon Rule #75:** Primitives have zero styling
- **Canon Rule #89:** Primitives are SSR-safe
- **Canon Rule #119:** No `#[prop(optional, into)]` in UI layer
- **Canon Rule #121:** StoredValue for Non-Copy in view!
- **Canon Rule #125:** UI Component Contracts (next rule)

---

## Version History

- **1.0.0** — Initial canonical version (2025-01-22)
