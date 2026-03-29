# Canon Rule #209: Axum Version Must Match Adapter Version

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Scope:** ssr, build
**Version:** 1.0.0  
**Date:** 2026-02-03

---

## Principle

**An application MUST use the exact Axum version required by its server adapter.**

- Objective
- Verifiable
- One clear boundary

---

## Problem

When Axum versions diverge between the application and the adapter layer:

- Type mismatches between identical-looking types
- `Body` incompatibilities at compile time
- Multiple `axum_core` versions in the dependency graph
- Non-obvious, hard-to-debug build failures

This breaks SSR routing and streaming deterministically.

---

## Forbidden Pattern

### ❌ Forbidden

```toml
leptos_axum = "0.8"
axum = "0.7"
```

The application is overriding a framework dependency owned by the adapter.

---

## Canonical Pattern

### ✅ Canonical

```toml
leptos_axum = "0.8"
axum = "0.8"
```

The adapter defines the framework version contract.

---

## Rationale

Adapters are integration boundaries.

- They define concrete framework versions
- They encapsulate compatibility guarantees
- Allowing apps to override breaks type identity
- This is a contract, not a preference

---

## Enforcement

- `cargo tree | grep axum` must show a single major version
- CI must fail on multiple `axum_core` versions
- Code review checklist item

---

## Exceptions

> **No exceptions. This rule is absolute.**

---

## Version History

- **1.0.0** — Initial version
