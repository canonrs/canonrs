# Canon Rule #320: DOM Is the Single Source of Truth

**Status:** ENFORCED
**Severity:** CRITICAL
**Version:** 1.0.0
**Date:** 2026-04-02

**Category:** governance
**Tags:** dom, state, architecture
**Language:** EN

---

**Intro:**
The DOM must be the canonical source of truth for all component state and interaction.

**Problem:**
multiple sources of truth exist

**Solution:**
centralize all state in DOM

**Signals:**
- state divergence
- inconsistent UI
- integration issues

**Search Intent:**
why dom should be source of truth

**Keywords:**
dom source of truth, frontend architecture, state consistency, ui synchronization

---

## Principle

DOM is authoritative.

---

## Problem

Multiple sources:

- create inconsistency
- cause desynchronization
- break integration
- complicate debugging

---

## Patterns

### Forbidden Pattern

```
state in memory + dom mismatch
```

---

### Canonical Pattern

```
data-rs-state="active"
```

---

## Contract

### Enforcement

- DOM is primary state source
- all systems must read from DOM

---

### Exceptions

None.

---

## Version History

- 1.0.0 — Initial definition
