# Canon Rule #283: Stable DOM Structure Required for Hydration

**Status:** ENFORCED
**Severity:** CRITICAL
**Version:** 1.0.0
**Date:** 2026-04-02

**Category:** core-runtime
**Tags:** hydration, ssr, dom, structure
**Language:** EN

---

**Intro:**
Hydration requires identical DOM structure between server and client. Any structural divergence causes runtime failure.

**Problem:**
ssr and wasm generate different dom trees causing hydration mismatch

**Solution:**
ensure identical node hierarchy regardless of state

**Signals:**
- hydration panic
- dom mismatch warning
- inconsistent node tree

**Search Intent:**
how to fix ssr wasm dom mismatch leptos

**Keywords:**
dom structure hydration, ssr mismatch leptos, wasm hydration failure, deterministic dom tree

---

## Principle

Structure must be deterministic.

State may change content, never structure.

---

## Problem

Conditional rendering:

- changes node hierarchy
- breaks hydration
- causes unrecoverable panic

---

## Patterns

### Forbidden Pattern

```
if condition {
  <div/>
} else {
  <span/>
}
```

---

### Canonical Pattern

```
<div>
  {if condition { "A" } else { "B" }}
</div>
```

---

## Contract

### Enforcement

- node tree must be identical in SSR and WASM
- only content may vary

---

### Exceptions

None.

---

## Version History

- 1.0.0 — Initial definition
