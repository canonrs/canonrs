# Canon Rule #307: No Inline Style for Logic

**Status:** ENFORCED
**Severity:** HIGH
**Version:** 1.0.0
**Date:** 2026-04-02

**Category:** styling-css
**Tags:** css, inline-style
**Language:** EN

---

**Intro:**
Inline styles must not encode logic or dynamic state.

**Problem:**
logic embedded in style attributes

**Solution:**
move logic to state attributes and CSS

**Signals:**
- style conditions
- inconsistent rendering
- duplication

**Search Intent:**
how to avoid logic in css

**Keywords:**
css architecture, inline style logic, state driven css, frontend patterns

---

## Principle

Logic must not live in style.

---

## Problem

Inline logic:

- breaks separation of concerns
- duplicates logic
- reduces maintainability
- complicates theming

---

## Patterns

### Forbidden Pattern

```
style={if active { "color:red" } else { "" }}
```

---

### Canonical Pattern

```
data-rs-state="active"
```

---

## Contract

### Enforcement

- no logic in style
- styling must derive from state

---

### Exceptions

None.

---

## Version History

- 1.0.0 — Initial definition
