# Canon Rule #282: No Raw Text Nodes in SSR Boundaries

**Status:** ENFORCED
**Severity:** CRITICAL
**Version:** 1.0.0
**Date:** 2026-04-02

**Category:** core-runtime
**Tags:** hydration, ssr, dom, leptos
**Language:** EN

---

**Intro:**
Raw text nodes rendered directly in SSR cause hydration mismatches when WASM expects structured elements.

**Problem:**
text nodes rendered in SSR differ from expected element nodes in hydration phase

**Solution:**
always wrap text content in explicit HTML elements

**Signals:**
- hydration panic
- expected text node mismatch
- tachys failed_to_cast_text_node

**Search Intent:**
how to fix leptos hydration text node mismatch

**Keywords:**
leptos hydration error, ssr text node mismatch, wasm dom hydration, tachys hydration panic

---

## Principle

Text must never exist as a root render node.

All text must be encapsulated in a stable element.

---

## Problem

Raw text:

- produces TEXT node in SSR
- may produce ELEMENT in WASM
- breaks hydration contract

---

## Patterns

### Forbidden Pattern

```
"Hello world"
```

---

### Canonical Pattern

```
<span>"Hello world"</span>
```

---

## Contract

### Enforcement

- no raw text nodes in primitives or UI
- all text must be wrapped

---

### Exceptions

None.

---

## Version History

- 1.0.0 — Initial definition
