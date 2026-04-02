# Canon Rule #304: No Optional DOM Nodes

**Status:** ENFORCED
**Severity:** CRITICAL
**Version:** 1.0.0
**Date:** 2026-04-02

**Category:** core-runtime
**Tags:** dom, hydration, structure
**Language:** EN

---

**Intro:**
Conditional DOM nodes break structural determinism required for hydration.

**Problem:**
DOM structure differs between SSR and client

**Solution:**
always render nodes and control visibility via attributes

**Signals:**
- hydration panic
- node mismatch
- inconsistent structure

**Search Intent:**
how to maintain stable dom structure in ssr

**Keywords:**
stable dom structure, ssr mismatch, hydration bug, frontend rendering rules

---

## Principle

DOM structure must be stable.

---

## Problem

Conditional rendering:

- changes node count
- shifts DOM tree
- breaks hydration expectations
- creates runtime instability

---

## Patterns

### Forbidden Pattern

```
{show && view! { <div/> }}
```

---

### Canonical Pattern

```
<div hidden={!show}></div>
```

---

## Contract

### Enforcement

- no conditional DOM nodes
- use hidden / aria-hidden

---

### Exceptions

None.

---

## Version History

- 1.0.0 — Initial definition
