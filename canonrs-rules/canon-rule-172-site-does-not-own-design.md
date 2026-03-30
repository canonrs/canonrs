# Canon Rule #172: The Site Does Not Own Design

**Status:** ENFORCED  
**Severity:** HIGH  
**Version:** 1.0.0  
**Date:** 2026-01-28

**Category:** governance
**Tags:** architecture, design-system, ownership, ui
**Language:** EN

---

**Intro:**
Allowing applications to define design rules leads to fragmentation and inconsistency across the system. Design must be centralized.

**Problem:**
applications define design rules causing fragmentation

**Solution:**
centralize all design definitions within the design system packages

**Signals:**
- design drift
- inconsistency
- fragmentation

**Search Intent:**
how to enforce design system ownership architecture

**Keywords:**
design system ownership pattern, ui centralization architecture, frontend design governance, component design separation

---

## Principle

**Applications consume design; they do not define it.**

---

## Problem

Allowing sites to define UI rules causes framework fragmentation.

---

## Forbidden Pattern

❌ Defining component-level design inside site projects.

---

## Canonical Pattern

✅ All design lives in rs-design and canonrs-design.

---

## Enforcement

- Repository boundaries
- Architecture review

---

## Exceptions

None.
