# Canon Rule #285: Overlay Must Use Container Pattern

**Status:** ENFORCED
**Severity:** CRITICAL
**Version:** 1.0.0
**Date:** 2026-04-02

**Category:** component-architecture
**Tags:** overlay, layout, composition, ui
**Language:** EN

---

**Intro:**
Overlay components require structural separation from content to ensure proper layering and interaction control.

**Problem:**
overlay implemented as standalone content breaks layering and interaction model

**Solution:**
use container pattern separating content and overlay layers

**Signals:**
- overlay not blocking interaction
- overlay rendered as content
- layering issues

**Search Intent:**
how to implement overlay pattern correctly

**Keywords:**
overlay container pattern, ui layering architecture, blocking overlay design, frontend overlay structure

---

## Principle

Overlay is a layer, not content.

---

## Problem

Standalone overlay:

- cannot cover content
- breaks composition
- fails interaction control

---

## Patterns

### Forbidden Pattern

```
<Overlay>content</Overlay>
```

---

### Canonical Pattern

```
<OverlayContainer>
  <Content/>
  <Overlay/>
</OverlayContainer>
```

---

## Contract

### Enforcement

- overlay must be separate from content
- container defines layering context

---

### Exceptions

None.

---

## Version History

- 1.0.0 — Initial definition
