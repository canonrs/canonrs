# Canon Rule #310: Components Must Be SSR Safe

**Status:** ENFORCED
**Severity:** CRITICAL
**Version:** 1.0.0
**Date:** 2026-04-02

**Category:** core-runtime
**Tags:** ssr, hydration
**Language:** EN

---

**Intro:**
All components must render identically in SSR and client environments.

**Problem:**
component output differs between environments

**Solution:**
ensure deterministic and environment-independent rendering

**Signals:**
- hydration mismatch
- runtime panic
- inconsistent UI

**Search Intent:**
how to build ssr safe components

**Keywords:**
ssr safe components, hydration consistency, frontend rendering safety, leptos ssr

---

## Principle

SSR and client must match.

---

## Problem

Environment divergence:

- creates different DOM
- breaks hydration
- causes runtime failure
- invalidates SSR guarantees

---

## Patterns

### Forbidden Pattern

```
if is_client() { render_a } else { render_b }
```

---

### Canonical Pattern

```
render_same_structure()
```

---

## Contract

### Enforcement

- identical DOM in SSR and client
- no environment-based divergence

---

### Exceptions

Client-only islands.

---

## Version History

- 1.0.0 — Initial definition
