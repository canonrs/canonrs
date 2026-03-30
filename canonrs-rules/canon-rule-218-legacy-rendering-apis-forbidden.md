# Canon Rule #218: Legacy Rendering APIs Are Forbidden

**Status:** ENFORCED  
**Severity:** HIGH  
**Scope:** ssr
**Version:** 1.0.0  
**Date:** 2026-02-03

---

## Principle

**Deprecated or legacy rendering APIs MUST NOT be used in new or migrated systems.**

- Objective
- Verifiable
- One clear boundary

---

## Problem

Using legacy APIs:

- Bypasses canonical shell handling
- Breaks meta integration
- Causes hydration mismatches
- Locks architecture to obsolete patterns

---

## Forbidden Pattern

### Forbidden

```rust
render_app_to_stream(|| view! { <App/> })
```

Manual streaming without shell integration.

---

## Canonical Pattern

### Canonical

```rust
Router::new()
    .leptos_routes(&opts, routes, || shell(opts.clone()))
```

Canonical integration path.

---

## Rationale

Framework evolution invalidates old entrypoints.

- CanonRS enforces forward-only architecture
- Legacy APIs create hidden technical debt
- Migration must be explicit and complete

---

## Enforcement

- Static analysis for forbidden symbols
- CI block on legacy imports
- Mandatory migration checklist

---

## Exceptions

> **No exceptions. This rule is absolute.**

---

## Version History

- **1.0.0** — Initial version
