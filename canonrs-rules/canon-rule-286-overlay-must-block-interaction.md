# Canon Rule #286: Overlay Must Block Interaction

**Status:** ENFORCED
**Severity:** HIGH
**Version:** 1.0.0
**Date:** 2026-04-02

**Category:** behavior
**Tags:** overlay, interaction, ux, pointer-events
**Language:** EN

---

**Intro:**
Overlays must prevent user interaction with underlying content during blocking states.

**Problem:**
overlay does not block pointer events allowing unintended interaction

**Solution:**
enforce pointer-event blocking and layering via z-index

**Signals:**
- click through overlay
- focus leakage
- interaction during loading

**Search Intent:**
how to block interaction with overlay

**Keywords:**
overlay pointer events css, blocking ui interaction, z-index overlay behavior, frontend loading overlay

---

## Principle

Overlay must intercept interaction.

---

## Problem

Without blocking:

- users interact with hidden content
- state becomes inconsistent
- UX breaks

---

## Patterns

### Forbidden Pattern

```
.overlay {
  pointer-events: none;
}
```

---

### Canonical Pattern

```
.overlay {
  pointer-events: all;
  position: absolute;
  inset: 0;
  z-index: 10;
}
```

---

## Contract

### Enforcement

- overlay must block pointer interaction
- overlay must visually cover content

---

### Exceptions

None.

---

## Version History

- 1.0.0 — Initial definition
