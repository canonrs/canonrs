# Canon Rule #130: Controlled UI Contract

**Status:** ENFORCED
**Severity:** HIGH
**Version:** 1.0.0
**Date:** 2025-01-22

**Category:** state-reactivity
**Tags:** signals, state, reactivity, ui
**Language:** EN

---

**Intro:**
Passing plain values instead of signals to UI components breaks reactive updates and causes stale UI state. Components lose synchronization with application state.

**Problem:**
ui components receive plain values instead of signals breaking reactivity

**Solution:**
require ui components to accept signal or rwsignal for reactive state props

**Signals:**
- ui not updating
- stale state
- manual rerender

**Search Intent:**
how to fix reactivity issues with

**Keywords:**
leptos signal ui pattern, reactive state ui components, signal vs value leptos, frontend reactivity architecture

---

## Principle

**UI components that control reactive state MUST accept `Signal<T>` or `RwSignal<T>`, never plain `T` for stateful props.**

---

## The Problem

When UI components accept plain types for reactive state:

1. The component cannot react to state changes
2. Parent components must call `.get()` to "adapt" the signal
3. This breaks reactivity—updates don't propagate
4. State becomes stale and UI diverges from reality

**Observable symptoms:**
- UI doesn't update when state changes
- Dialog doesn't open/close when signal changes
- Tabs don't switch when active tab signal changes
- Components require manual re-rendering

---

## Forbidden Patterns

### ❌ Forbidden
```rust
// UI accepts bool instead of Signal<bool>
#[component]
pub fn Dialog(
    children: ChildrenFn,
    open: bool,  // ❌ Wrong!
) -> impl IntoView {
    view! {
        <DialogPrimitive open={open}>
            {children()}
        </DialogPrimitive>
    }
}

// Usage forces .get() and breaks reactivity
<Dialog open=is_open.get()>  // ❌ Only reads once!
    "Content"
</Dialog>
```

### ❌ Forbidden
```rust
// Tabs accepts String instead of Signal<String>
#[component]
pub fn Tabs(
    active: String,  // ❌ Wrong!
) -> impl IntoView {
    // Cannot react to changes
}
```

---

## Canonical Pattern

### ✅ Canonical — Signal-Based UI
```rust
// UI accepts Signal<bool>
#[component]
pub fn Dialog(
    children: ChildrenFn,
    open: Signal<bool>,  // ✅ Correct!
) -> impl IntoView {
    view! {
        <DialogPrimitive open=move || open.get()>
            {children()}
        </DialogPrimitive>
    }
}

// Usage preserves reactivity
let (is_open, set_is_open) = signal(false);

view! {
    <Dialog open=is_open.into()>  // ✅ Reactive!
        "Content"
    </Dialog>
}
```

### ✅ Canonical — Contract Flow
```
Block/Page
    ↓ Signal<T>
UI Component
    ↓ T (via .get())
Primitive
```

**Key insight:** The UI layer is responsible for calling `.get()` to snapshot the signal for the Primitive. The Block never calls `.get()`.

---

## Rationale

**Separation of concerns:**
- **Blocks/Pages**: Manage state (`Signal<T>`)
- **UI Components**: Handle reactivity (`.get()` at render time)
- **Primitives**: Render snapshots (plain `T`)

**Why this matters:**
1. **Reactivity**: UI components must respond to state changes
2. **Single Responsibility**: Each layer has one job
3. **Predictability**: State flows one direction
4. **SSR Safety**: Signals work in both SSR and CSR

**Anti-pattern consequence:**
When UI accepts plain `T`, parent components call `.get()` too early. The value is "frozen" and updates are lost. This creates bugs that only appear at runtime.

---

## Enforcement

**Type System:**
- UI component signatures must use `Signal<T>` for reactive state
- Compiler enforces this automatically

**Code Review:**
- Check UI component props
- Verify no `.get()` calls in parent components passing to UI

**Linter:**
- Detect `#[component]` with state-like props (e.g., `open`, `active`, `selected`) that aren't `Signal<T>`

---

## Exceptions

**Exception 1: Static Props**
Non-reactive props like `variant`, `size`, `class` can be plain types:
```rust
#[component]
pub fn Button(
    variant: ButtonVariant,  // ✅ OK - not reactive state
    children: Children,
) -> impl IntoView { ... }
```

**Exception 2: Primitives**
Primitives always accept plain types (they render snapshots):
```rust
#[component]
pub fn DialogPrimitive(
    open: bool,  // ✅ OK - Primitive layer
) -> impl IntoView { ... }
```

---

## Version History

- **1.0.0** — Initial canonical version (2025-01-22)
