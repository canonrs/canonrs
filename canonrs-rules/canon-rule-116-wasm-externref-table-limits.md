# Canon Rule #116: WASM Externref Table Limits

**Status:** ENFORCED
**Severity:** CRITICAL
**Version:** 1.0.0
**Date:** 2026-01-17

**Category:** core-runtime
**Tags:** wasm, externref, callbacks, runtime
**Language:** EN

---

**Intro:**
Excessive JS↔WASM references exhaust browser externref tables, causing runtime crashes. Each callback or handler adds entries, leading to resource exhaustion at scale.

**Problem:**
unbounded creation of externrefs causes wasm runtime crashes

**Solution:**
avoid per-component callbacks and delegate behavior to global shell handlers

**Signals:**
- table grow error
- runtime crash
- externref overflow

**Search Intent:**
how to fix wasm externref error

**Keywords:**
wasm externref table limit, leptos callback memory issue, js wasm reference overflow, externref table grow error

---

## Principle

**Components must not create unbounded JS↔WASM references.**

The externref table is a finite browser resource.
Design systems that violate this will crash at runtime.

---

## The Problem

When using WASM with JS interop:
- Each `#[wasm_bindgen]` export creates table entries
- Each `Callback`, `on:*` handler, `web_sys` API call consumes refs
- Providers that register global handlers multiply this
- Browser refuses to grow table beyond limits

### Real Error
```
RangeError: WebAssembly.Table.grow(): failed to grow table by 4
at __wbindgen_init_externref_table
```

This is NOT:
- A build error
- A memory limit
- A WASM size issue
- Fixable with compiler flags

This IS:
- Runtime resource exhaustion
- Architectural boundary violation
- Direct consequence of too many JS bindings

---

## Anti-Pattern (FORBIDDEN)
```rust
// ❌ FORBIDDEN - creates externref per component
#[component]
pub fn DraggableItem() -> impl IntoView {
    let on_drag = move |ev: web_sys::DragEvent| { ... };
    view! { <div on:dragstart=on_drag /> }
}

// ❌ FORBIDDEN - providers with closures
#[component]
pub fn DragDropProvider(children: Children) -> impl IntoView {
    let ctx = DragContext::new();
    ctx.set_handler(move |event| { ... }); // externref!
    children()
}
```

Each instance = new externref.
100 items = 100 refs.
Browser says NO.

---

## Canonical Pattern (REQUIRED)
```rust
// ✅ REQUIRED - zero externrefs
#[component]
pub fn DraggableItem(id: String) -> impl IntoView {
    view! { <div data-drag-id=id draggable="true" /> }
}
```
```javascript
// ✅ Shell runtime (ONE handler globally)
document.addEventListener("dragstart", e => {
  const el = e.target.closest("[data-drag-id]");
  if (!el) return;
  // handle via data attributes
});
```

---

## Enforcement

UI Components and Blocks MUST:
- Render data-* attributes only
- Use Signal/RwSignal for state
- Delegate runtime behavior to shell

UI Components MUST NOT:
- Use `on:*` handlers with closures
- Call `web_sys` APIs directly
- Register callbacks per instance
- Create `Callback::new` in render

---

## Scope

This applies to:
- UI components (Button, Input, etc.)
- Blocks (DataTable, Tree, Drag&Drop)
- Any repeating element

This does NOT apply to:
- Shell/App boundary (one-time setup OK)
- Primitives (limited, controlled use)
- Server-side only code

---

## Canonical Justification

> Externrefs are a finite resource. Components that consume them unbounded will always fail at scale.

---

## Related Canon Rules

- Canon Rule #102 — Runtime JS Is Shell Infrastructure
- Canon Rule #103 — Critical Runtime JS Must Be Inline
- Canon Rule #93 — WASM Dev Builds Must Use Release Mode

---