# Canon Rule #199: Server Adapters Are Integration Code

**Status:** ENFORCED  
**Severity:** HIGH  
**Scope:** architecture, ssr
**Version:** 1.0.0  
**Date:** 2026-02-03

---

## Principle

**Server adapters are integration glue, not framework architecture.**

The framework must remain server-agnostic.

---

## Problem

When adapters leak into framework code:

- Framework becomes locked to one server
- Migration cost explodes
- Runtime types propagate incorrectly
- Build graphs destabilize

---

## Forbidden Patterns

❌ Framework crates importing `axum::*`  
❌ UI or behaviors referencing server extractors  
❌ Adapter-specific types in shared APIs  

---

## Canonical Pattern

```rust
// framework
pub fn render_app() -> String {}

// integration
Router::new().route("/", get(|| render_app()));
```

Adapters connect systems — they do not define them.

---

## Enforcement

- Adapters allowed only in integration crates
- Zero adapter dependencies in design-system crates
- Review rejects adapter imports outside integration layer

---

## Exceptions

**None. Adapters are replaceable by definition.**

---
