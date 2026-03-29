# Canon Rule #217: Entry Points Must Be Explicit, Public, and Isolated

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Scope:** ssr, csr, build
**Version:** 1.0.0  
**Date:** 2026-02-03

---

## Principle

**All SSR and CSR entrypoints MUST be explicit, public, and feature-isolated.**

- Objective
- Verifiable
- One clear boundary

---

## Problem

Implicit or private entrypoints cause:

- Unresolvable imports
- Feature flag leakage
- Build-time ambiguity
- Tooling incompatibility

---

## Forbidden Pattern

### ❌ Forbidden

```rust
#[cfg(feature = "hydrate")]
fn hydrate() { ... }
```

Entrypoint not exported.

---

## Canonical Pattern

### ✅ Canonical

```rust
#[cfg(feature = "hydrate")]
pub fn hydrate() { ... }

#[cfg(feature = "ssr")]
pub fn shell(opts: LeptosOptions) -> impl IntoView { ... }
```

Entrypoints are explicit contracts.

---

## Rationale

Entrypoints are integration surfaces.

- Tooling depends on visibility
- Features must not overlap
- Isolation guarantees determinism

---

## Enforcement

- Build fails on unresolved imports
- CI validates `pub` visibility
- Feature-gated symbol checks

---

## Exceptions

> **No exceptions. This rule is absolute.**

---

## Version History

- **1.0.0** — Initial version
