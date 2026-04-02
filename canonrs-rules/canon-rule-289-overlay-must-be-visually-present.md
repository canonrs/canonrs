# Canon Rule #289: Overlay Must Be Visually Present

**Status:** ENFORCED
**Severity:** HIGH
**Version:** 1.0.0
**Date:** 2026-04-02

**Category:** design-system
**Tags:** overlay, ui, feedback, visual
**Language:** EN

---

**Intro:**
An overlay must provide visible feedback. Invisible overlays break user perception and interaction clarity.

**Problem:**
overlay exists structurally but has no visual representation

**Solution:**
ensure overlay has visible feedback such as spinner, backdrop, or indicator

**Signals:**
- invisible overlay
- user confusion
- no loading feedback

**Search Intent:**
how to create visible loading overlay

**Keywords:**
loading overlay ui, spinner overlay design, visual feedback loading, frontend ux overlay

---

## Principle

Overlay must communicate state.

---

## Problem

Invisible overlay:

- provides no feedback
- confuses users
- breaks UX expectations

---

## Patterns

### Forbidden Pattern

```
<div data-rs-loading-overlay></div>
```

---

### Canonical Pattern

```
<div data-rs-loading-overlay>
  <Spinner/>
</div>
```

---

## Contract

### Enforcement

- overlay must render visible content
- feedback must be perceivable

---

### Exceptions

None.

---

## Version History

- 1.0.0 — Initial definition
