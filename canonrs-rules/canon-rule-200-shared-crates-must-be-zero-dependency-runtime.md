# Canon Rule #200: Shared Crates Must Be Zero-Dependency Runtime

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Scope:** architecture, workspace
**Version:** 1.0.0  
**Date:** 2026-02-03

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
