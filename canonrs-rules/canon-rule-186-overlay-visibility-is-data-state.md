# Canon Rule #190: Overlay Visibility Is Controlled Only via data-state

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Scope:** hydration, css
**Version:** 1.1.0  
**Date:** 2026-02-03

---

## Principle

**Overlay visibility MUST be controlled exclusively via the `data-state` attribute.**

No conditional DOM.  
No inline styles.  
No runtime structure changes.

---

## Forbidden Patterns

❌ `display: none` set in Rust  
❌ Conditional rendering  
❌ Inline visibility styles  

```rust
overlay.style().set_property("display", "none"); // ❌
```

---

## Canonical Pattern

```html
<div data-overlay data-state="closed"></div>
```

```css
[data-overlay][data-state="closed"] {
  visibility: hidden;
  pointer-events: none;
}

[data-overlay][data-state="open"] {
  visibility: visible;
}
```

---

## Architectural Invariants

- SSR and CSR DOM are identical
- Hydration is deterministic
- State is inspectable and debuggable

---

## Enforcement

- CI greps for display mutation
- Overlays must always exist in DOM
- CSS maps `data-state` → visibility

---

## Exceptions

**None. This rule is absolute.**

---
