# Canon Rule #331: Island SSR State Must Be Fully Materialized Without Signals

**Status:** ENFORCED
**Severity:** HIGH
**Version:** 1.0.0
**Date:** 2026-04-04

**Category:** islands-architecture
**Tags:** islands, ssr, signals, hydration, state
**Language:** EN

---

**Intro:**
Every initial state value rendered in SSR must be pre-computed as a static string. Reactive signals are not available during SSR and produce empty or incorrect output when used as the sole source of truth for `data-rs-state`.

**Problem:**
Signal values like `is_hover.get()` return their initial value during SSR but the rendered attribute may be empty if the closure does not account for static initial state separately.

**Solution:**
Pre-compute an `initial_state` string from all static booleans and use it as a fallback in the reactive closure.

**Signals:**
- `data-rs-state=""` in SSR output even when initial state was passed
- state appears only after user interaction, not on page load
- CSS that depends on initial state does not apply on first render

**Search Intent:**
leptos island ssr state empty, signal not rendered ssr, data-rs-state blank on load

**Keywords:**
leptos ssr signal, island initial state, data-rs-state ssr, hydration state mismatch

---

## Principle

Leptos signals are reactive primitives for the client. During SSR, signal closures execute once with their initial value. If the closure returns an empty string when signals are false, the SSR output will be empty regardless of static props passed in.

---

## Problem
```rust
// ❌ broken — if initial_hover is true, signal starts true,
// but the closure may return "" on SSR if not handled
let state = move || {
    let mut s = vec![];
    if is_hover.get() { s.push("hover"); }
    s.join(" ")
};
```

---

## Patterns

### Forbidden Pattern

Using only signal getters in the `data-rs-state` closure without a static fallback.

### Canonical Pattern
```rust
// Pre-compute static initial state for SSR
let initial_state = {
    let mut s = vec![];
    if disabled      { s.push("disabled"); }
    if is_first      { s.push("first");    }
    if is_last       { s.push("last");     }
    if initial_hover { s.push("hover");    }
    if initial_focus { s.push("focus");    }
    s.join(" ")
};

// Reactive closure with SSR fallback
let state = move || {
    let mut s = vec![];
    if disabled        { s.push("disabled"); }
    if is_first        { s.push("first");    }
    if is_last         { s.push("last");     }
    if is_hover.get()  { s.push("hover");    }
    if is_active.get() { s.push("active");   }
    if is_focus.get()  { s.push("focus");    }
    s.join(" ")
};

// In view: use initial_state as SSR fallback
data-rs-state=move || {
    let s = state();
    if s.is_empty() { initial_state.clone() } else { s }
}
```

---

## Contract

### Enforcement

- every island with initial state props must pre-compute `initial_state` as a static string
- the `data-rs-state` attribute must use `initial_state` as fallback when the reactive closure returns empty
- static positional states (first, last, disabled) must appear in both the static and reactive paths

### Exceptions

None.

---

## Version History

- 1.0.0 - Initial definition
