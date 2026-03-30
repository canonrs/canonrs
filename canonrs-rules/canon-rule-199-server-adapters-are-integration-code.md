# Canon Rule #199: Server Adapters Are Integration Code

**Status:** ENFORCED  
**Severity:** HIGH  
**Version:** 1.0.0  
**Date:** 2026-02-03

**Category:** governance
**Tags:** server, adapters, integration, ssr
**Language:** EN

---

**Intro:**
Embedding server-specific logic into framework code couples architecture to a specific runtime and breaks portability. Adapters must remain isolated.

**Problem:**
server adapter logic leaks into framework causing coupling and instability

**Solution:**
isolate server adapters as integration layer separate from framework

**Signals:**
- framework coupling
- runtime leak
- migration issue

**Search Intent:**
how to separate server adapters from framework

**Keywords:**
server adapter architecture, integration layer pattern, framework agnostic design, ssr adapter separation

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