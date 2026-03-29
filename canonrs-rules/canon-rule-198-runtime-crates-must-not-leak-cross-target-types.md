# Canon Rule #198: Runtime Crates Must Not Leak Cross-Target Types

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Scope:** architecture, workspace
**Version:** 1.0.0  
**Date:** 2026-02-03

---

## Principle

**Runtime crates must never exchange concrete runtime types across targets.**

SSR and CSR are different runtimes.  
Their types must never cross boundaries.

---

## Problem

When runtime types leak:

- Multiple versions of the same crate are linked
- Type identity breaks (`axum::Body ≠ leptos::Body`)
- Errors surface deep in dependency graphs
- Integration becomes fragile

---

## Forbidden Patterns

❌ Shared crates depending on `axum`, `leptos`, `web-sys`  
❌ Returning runtime-specific types from framework APIs  
❌ Passing runtime objects across crate boundaries  

---

## Canonical Pattern

```rust
// shared
pub struct RouteInfo {
    pub path: String,
}

// ssr
handle_route(RouteInfo);

// csr
hydrate_route(RouteInfo);
```

Shared defines **contracts**, runtimes execute.

---

## Enforcement

- `cargo tree --features hydrate` must exclude SSR runtimes
- Shared crates compile without feature flags
- CI fails on duplicated runtime crates

---

## Exceptions

**None. Runtime isolation is mandatory.**

---
