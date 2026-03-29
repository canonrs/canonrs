# Canon Rule #184: Floating Overlays Expose Position Only via CSS Variables

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Scope:** css, primitives, overlays
**Version:** 1.1.0  
**Date:** 2026-02-03  

---

## Principle

**Floating overlay primitives MUST expose computed positioning exclusively via CSS custom properties as a public contract.**

This rule defines the **external interface** between positioning logic and rendering.

---

## Contract Definition

- Position values are exposed as:
  - `--floating-x`
  - `--floating-y`
- No other positioning channel is allowed
- Consumers (CSS, themes, animations) rely on this contract

---

## Forbidden Pattern

❌ Inline positioning  
❌ DOM mutation of `top`, `left`, `bottom`, `right`

---

## Canonical Pattern

```css
[data-overlay] {
  transform: translate(var(--floating-x), var(--floating-y));
}
```

```rust
style.set_property("--floating-x", "128px").ok();
style.set_property("--floating-y", "42px").ok();
```

---

## Scope Clarification

- This rule defines **WHAT is exposed**
- Enforcement details live in **Canon Rule #187**

---

## Exceptions

**None. This contract is absolute.**

---
