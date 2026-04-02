# Canon Rule #305: All Text Must Be Wrapped

**Status:** ENFORCED
**Severity:** CRITICAL
**Version:** 1.0.0
**Date:** 2026-04-02

**Category:** core-runtime
**Tags:** text, hydration, dom
**Language:** EN

---

**Intro:**
Raw text nodes introduce implicit DOM structures that break hydration expectations.

**Problem:**
text nodes mismatch between SSR and client

**Solution:**
wrap all text inside explicit elements

**Signals:**
- expected text node error
- hydration panic
- string mismatch

**Search Intent:**
how to fix hydration errors caused by text nodes

**Keywords:**
text node hydration, leptos mismatch, wrap text dom, ssr bug

---

## Principle

Text must be explicit.

---

## Problem

Raw text nodes:

- create implicit DOM
- mismatch expected structure
- break hydration casting
- cause runtime panic

---

## Patterns

### Forbidden Pattern

```
<div>"Hello"</div>
```

---

### Canonical Pattern

```
<div><span>"Hello"</span></div>
```

---

## Contract

### Enforcement

- no raw text nodes
- all text must be wrapped

---

### Exceptions

None.

---

## Version History

- 1.0.0 — Initial definition
