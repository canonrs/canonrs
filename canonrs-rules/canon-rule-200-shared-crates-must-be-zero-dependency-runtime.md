# Canon Rule #200: Shared Crates Must Be Zero-Dependency Runtime

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Version:** 1.0.0  
**Date:** 2026-02-03

**Category:** governance
**Tags:** shared, runtime, contracts, workspace
**Language:** EN

---

**Intro:**
Adding runtime dependencies to shared crates breaks portability and contaminates architecture. Shared must remain pure contract layer.

**Problem:**
shared crates include runtime dependencies causing architecture leakage

**Solution:**
restrict shared crates to pure types with no runtime dependencies

**Signals:**
- runtime import
- dependency leak
- build failure

**Search Intent:**
how to keep shared crates runtime agnostic

**Keywords:**
shared crate architecture rust, zero dependency runtime pattern, contract layer design, workspace isolation rules

---

## Principle

**Shared crates contain only pure, runtime-agnostic contracts.**

Shared is not a common runtime.  
Shared is a **contract layer**.

---

## Allowed Contents

✅ structs  
✅ enums  
✅ traits  
✅ constants  
✅ serde-compatible models  

---

## Forbidden Contents

❌ DOM access  
❌ Server adapters  
❌ Async runtimes  
❌ Effects, hooks, behaviors  
❌ Runtime-bound dependencies  

---

## Canonical Pattern

```rust
// shared
pub enum ThemeMode {
    Light,
    Dark,
}
```

```rust
// ssr
apply_theme(ThemeMode::Dark);

// csr
hydrate_theme(ThemeMode::Dark);
```

---

## Enforcement

- Shared crates compile for all targets
- Dependency audit shows zero runtime crates
- CI fails on any runtime import

---

## Exceptions

**None. Shared must remain pure forever.**

---