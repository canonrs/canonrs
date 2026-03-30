# Canon Rule #86: Children vs ChildrenFn Contract

**Status:** ENFORCED
**Severity:** CRITICAL
**Scope:** leptos, state
**Version:** 1.0.0
**Date:** 2026-01-14

---

## Principle

`Children` and `ChildrenFn` are **NOT interchangeable**.

> **`Children` = `FnOnce` (call once)**
> **`ChildrenFn` = `Fn` (call multiple times)**

Using the wrong type causes `FnOnce` errors, `Sync` violations, and broken reactivity.

---

## The Problem

Developers encounter this error repeatedly:
```
error[E0525]: expected a closure that implements the `Fn` trait, 
but this closure only implements `FnOnce`
```

**Root causes:**
1. Using `Children` in wrapper components
2. Trying to call `children()` multiple times
3. Attempting `StoredValue<Children>` (not `Sync`)
4. Writing `{children}` instead of `{children()}`

---

## Always Use ChildrenFn When

### 1. Wrapper Components
Components that wrap other content and may re-render:
```rust
#[component]
pub fn Dialog(
    children: ChildrenFn, // ✅ Can re-render
    #[prop(into)] open: RwSignal<bool>,
) -> impl IntoView {
    view! {
        <DialogPrimitive open=open>
            {children()} // ✅ Always call with ()
        </DialogPrimitive>
    }
}
```

### 2. Components Used in `Show`
Any component that appears inside reactive boundaries:
```rust
view! {
    <Show when=move || open.get()>
        <Dialog> // ✅ Dialog uses ChildrenFn
            {children()} // ✅ Can re-render
        </Dialog>
    </Show>
}
```

### 3. Components with Reactive Visibility
Any component that conditionally renders:
```rust
#[component]
pub fn Collapsible(
    children: ChildrenFn, // ✅ May show/hide
    #[prop(into)] expanded: RwSignal<bool>,
) -> impl IntoView {
    view! {
        <Show when=move || expanded.get()>
            {children()} // ✅ Reactive
        </Show>
    }
}
```

---

## Always Use Children When

### 1. Static Layout Slots
Components that render once and never change:
```rust
#[component]
pub fn DialogHeader(
    children: Children, // ✅ Static slot
) -> impl IntoView {
    view! {
        <div data-dialog-header="">
            {children()} // ✅ Called once
        </div>
    }
}
```

### 2. Simple Wrappers
Divs, sections, or containers without logic:
```rust
#[component]
pub fn Card(
    children: Children, // ✅ No reactivity
) -> impl IntoView {
    view! {
        <div class="card">
            {children()}
        </div>
    }
}
```

---

## Never Do These

### ❌ Anti-Pattern 1: `{children}` Without `()`
```rust
view! {
    <div>{children}</div> // ❌ WRONG
}
```

**Why it fails:**
`ChildrenFn` is `Arc<dyn Fn() -> AnyView>` which does NOT implement `RenderHtml`.

**Correct:**
```rust
view! {
    <div>{children()}</div> // ✅ Always call
}
```

---

### ❌ Anti-Pattern 2: `StoredValue<Children>`
```rust
let children_stored = StoredValue::new(children); // ❌ WRONG
```

**Error:**
```
the trait `Sync` is not implemented for `dyn FnOnce() -> AnyView + Send`
```

**Why it fails:**
`Children` is `Box<dyn FnOnce()>` which is NOT `Sync`.

**Correct:**
Use `ChildrenFn` instead, or don't store at all.

---

### ❌ Anti-Pattern 3: Cloning Children
```rust
let children_clone = children.clone(); // ❌ Does not exist
```

**Why it fails:**
Neither `Children` nor `ChildrenFn` implement `Clone`.

**Correct:**
Use `ChildrenFn` which can be called multiple times without cloning.

---

### ❌ Anti-Pattern 4: Children in Wrapper Components
```rust
#[component]
pub fn Dialog(
    children: Children, // ❌ WRONG - will break in Show
    #[prop(into)] open: RwSignal<bool>,
) -> impl IntoView {
    view! {
        <DialogPrimitive open=open>
            {children()} // ❌ FnOnce error
        </DialogPrimitive>
    }
}
```

**Why it fails:**
`DialogPrimitive` expects `Fn + Send + Sync`, but `Children` is `FnOnce`.

**Correct:**
```rust
#[component]
pub fn Dialog(
    children: ChildrenFn, // ✅ Fn trait
    #[prop(into)] open: RwSignal<bool>,
) -> impl IntoView {
    view! {
        <DialogPrimitive open=open>
            {children()} // ✅ Works
        </DialogPrimitive>
    }
}
```

---

## Decision Matrix

| Use Case | Type | Reason |
|----------|------|--------|
| Wrapper (Dialog, Sheet, Button) | `ChildrenFn` | May re-render |
| Inside `Show`, `Suspense` | `ChildrenFn` | Reactive boundary |
| Static slot (Header, Footer) | `Children` | Renders once |
| Simple div wrapper | `Children` | No reactivity |
| Conditional visibility | `ChildrenFn` | Show/hide logic |

---

## Debugging Protocol

When you see:
```
error[E0525]: expected a closure that implements the `Fn` trait, 
but this closure only implements `FnOnce`
```

**Check:**
1. Is the component used inside `Show`? → Use `ChildrenFn`
2. Is it a wrapper component? → Use `ChildrenFn`
3. Does it have reactive state? → Use `ChildrenFn`
4. Is it just a static slot? → Use `Children`

---

## Canonical Examples

### ✅ Correct: Dialog (Wrapper)
```rust
#[component]
pub fn Dialog(
    children: ChildrenFn, // ✅
    #[prop(into)] open: RwSignal<bool>,
) -> impl IntoView {
    view! {
        <DialogPrimitive open=open>
            {children()} // ✅
        </DialogPrimitive>
    }
}
```

### ✅ Correct: DialogHeader (Static Slot)
```rust
#[component]
pub fn DialogHeader(
    children: Children, // ✅
) -> impl IntoView {
    view! {
        <div data-dialog-header="">
            {children()} // ✅
        </div>
    }
}
```

### ✅ Correct: Usage with Show
```rust
view! {
    <Dialog open=modal_open> // ChildrenFn internally
        <Show when=move || modal_open.get()>
            <DialogContent> // ChildrenFn internally
                {content()} // ✅
            </DialogContent>
        </Show>
    </Dialog>
}
```

---

## Common Mistakes And Fixes

### Mistake 1: Wrong Type in Wrapper
```rust
// ❌ WRONG
#[component]
pub fn Sheet(children: Children, ...) -> impl IntoView { ... }

// ✅ CORRECT
#[component]
pub fn Sheet(children: ChildrenFn, ...) -> impl IntoView { ... }
```

### Mistake 2: Forgot `()`
```rust
// ❌ WRONG
view! { <div>{children}</div> }

// ✅ CORRECT
view! { <div>{children()}</div> }
```

### Mistake 3: Trying to Store
```rust
// ❌ WRONG
let stored = StoredValue::new(children);

// ✅ CORRECT
// Don't store - just call children() when needed
```

---

## Canonical Justification

> **Leptos enforces ownership rules strictly.**
> `FnOnce` can only be called once.
> `Fn` can be called multiple times.
> Wrapper components need `Fn` because they may re-render.

This rule enforces:
- **Correct ownership** — No `FnOnce` violations
- **Proper reactivity** — Components re-render when needed
- **Type safety** — Compiler catches errors early
- **SSR compatibility** — `Fn + Send + Sync` works everywhere

---

## Canon References

- Canon Rule #2 — Ownership Rules (StoredValue, reactive access)
- Canon Rule #50 — Provider Singleton Pattern (context + children)
- Canon Rule #4 — Hydration Safety (SSR + reactivity)

---

## Mental Model
```
Component Type → Children Type → Call Pattern

Wrapper         → ChildrenFn   → {children()}
Static Slot     → Children     → {children()}
Reactive        → ChildrenFn   → {children()}
Simple Div      → Children     → {children()}
```

**Rule of thumb:**
> When in doubt, use `ChildrenFn`.
> It works everywhere `Children` works, plus reactivity.

---

## Related Errors

If you see these errors, you have a Children/ChildrenFn problem:

1. `expected Fn, found FnOnce`
2. `Sync is not implemented for dyn FnOnce`
3. `method 'clone' not found`
4. `ToChildren` trait bound not satisfied

**Solution:** Check if you're using the right type for your use case.
