# Canon Rule #164: WASM and JS Assets Must Be Served by SSR

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Version:** 1.0.0  
**Date:** 2026-01-27

**Category:** build-tooling
**Tags:** ssr, wasm, assets, routing
**Language:** EN

---

**Intro:**
Missing asset routing for WASM and JS breaks hydration and prevents client initialization. Applications render HTML but fail to become interactive.

**Problem:**
wasm and js assets are not served causing hydration failure

**Solution:**
explicitly serve pkg assets from ssr server routes

**Signals:**
- 404 wasm
- no hydration
- static html only

**Search Intent:**
how to fix wasm not loading in ssr

**Keywords:**
wasm asset routing ssr, leptos pkg serve issue, hydration missing assets, ssr static asset config

---

## Principle

**All `/pkg` assets must be served explicitly by the SSR server.**

- Objective
- Verifiable
- One clear boundary

---

## Problem

Missing `/pkg` routing breaks hydration.

- 404 on `.wasm` and `.js`
- App loads HTML but never hydrates
- Silent runtime failure
- False UI rendering success

---

## Forbidden Pattern

### ❌ Forbidden

```rust
Router::new().fallback_service(ServeDir::new("public"))
```

Without `/pkg` handling.

---

## Canonical Pattern

### ✅ Canonical

```rust
.nest_service("/pkg", ServeDir::new("{site_root}/pkg"))
```

Why this complies with the rule.

---

## Rationale

WASM is not optional in CanonRS.

- Enforces runtime completeness
- Guarantees hydration integrity
- Prevents partial renders
- Deployment invariant

---

## Enforcement

- Runtime smoke test
- CI asset existence check
- Manual verification

---

## Exceptions

> **No exceptions. This rule is absolute.**

---

## Version History

- **1.0.0** — Initial version