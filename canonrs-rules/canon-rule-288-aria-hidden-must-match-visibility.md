# Canon Rule #288: aria-hidden Must Match Visibility

**Status:** ENFORCED
**Severity:** HIGH
**Version:** 1.0.0
**Date:** 2026-04-02

**Category:** accessibility
**Tags:** aria, accessibility, visibility, dom
**Language:** EN

---

**Intro:**
ARIA visibility must reflect actual DOM visibility. Mismatches cause incorrect screen reader behavior.

**Problem:**
aria-hidden does not match real visibility state

**Solution:**
synchronize aria-hidden with actual DOM visibility

**Signals:**
- screen reader reads hidden content
- accessibility audit failures
- inconsistent aria state

**Search Intent:**
how to fix aria-hidden mismatch

**Keywords:**
aria hidden accessibility, dom visibility aria, screen reader mismatch, wcag aria rules

---

## Principle

ARIA must reflect reality.

---

## Problem

Mismatch:

- breaks accessibility
- confuses assistive tech
- invalidates semantic contract

---

## Patterns

### Forbidden Pattern

```
aria-hidden="true"
<!-- element visible -->
```

---

### Canonical Pattern

```
aria-hidden={state == "idle"}
```

---

## Contract

### Enforcement

- aria-hidden must reflect visibility
- no divergence allowed

---

### Exceptions

None.

---

## Version History

- 1.0.0 — Initial definition
