# Canon Rule #312: Behaviors Must Be Idempotent

**Status:** ENFORCED
**Severity:** CRITICAL
**Version:** 1.0.0
**Date:** 2026-04-02

**Category:** behavior
**Tags:** behavior, idempotency, runtime
**Language:** EN

---

**Intro:**
Behaviors may attach multiple times due to hydration, re-rendering, or hot-reload. They must not duplicate effects or side effects.

**Problem:**
multiple attachments create duplicated listeners and inconsistent state

**Solution:**
ensure behavior attachment is idempotent using guards

**Signals:**
- duplicated events
- multiple listeners firing
- unstable toggle behavior

**Search Intent:**
how to make frontend behaviors idempotent

**Keywords:**
idempotent behavior dom, attach guard frontend, duplicate event listeners, wasm hydration behavior

---

## Principle

Behavior must be safe to attach multiple times.

---

## Problem

When behaviors attach more than once:

- duplicate listeners accumulate
- state toggles inconsistently
- performance degrades
- side effects multiply

Hydration and hot-reload make this scenario inevitable.

---

## Patterns

### Forbidden Pattern

```
element.addEventListener("click", handler);
```

(no guard)

---

### Canonical Pattern

```
if (!element.hasAttribute("data-rs-attached")) {
  element.setAttribute("data-rs-attached", "1");
  element.addEventListener("click", handler);
}
```

---

## Contract

### Enforcement

- all behaviors must use attach guards
- use data-rs-*-attached pattern
- listeners must be registered once

---

### Exceptions

None.

---

## Version History

- 1.0.0 — Initial definition
