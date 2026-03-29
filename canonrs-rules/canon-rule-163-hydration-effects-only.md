# Canon Rule #163: DOM Effects Are Hydration-Only

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Scope:** ssr, hydration
**Version:** 1.0.0  
**Date:** 2026-01-27

---

## Principle

**DOM mutations must only run under the `hydrate` feature.**

- Objective
- Verifiable
- One clear boundary

---

## Problem

Running DOM effects during SSR causes hydration mismatch.

- Silent crashes
- Inconsistent HTML
- Impossible-to-debug runtime errors
- Broken SSR guarantees

---

## Forbidden Pattern

### ❌ Forbidden

```rust
Effect::new(|| {
    document().body().unwrap();
});
```

Without hydration guard.

---

## Canonical Pattern

### ✅ Canonical

```rust
#[cfg(feature = "hydrate")]
Effect::new(|| {
    document().document_element();
});
```

Why this complies with the rule.

---

## Rationale

SSR and hydration are separate execution domains.

- Protects SSR determinism
- Enforces runtime boundaries
- Prevents DOM walking during SSR
- Architectural invariant

---

## Enforcement

- Feature-gated compilation
- CI SSR build checks
- Manual audit

---

## Exceptions

> **No exceptions. This rule is absolute.**

---

## Version History

- **1.0.0** — Initial version
