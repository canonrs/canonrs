# Canon Rule #250: Design System Must Not Depend on Products

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Scope:** workspace, architecture
**Version:** 1.0.0  
**Date:** 2026-02-13  

---

## Principle

Crates classified as `design-system`, `ui`, `tokens-engine`, or `shared`
MUST NEVER depend on `product` crates.

Dependency flow is strictly:

Core → Products  
Never the reverse.

---

## Problem

Reverse dependency creates:

- Cycles in graph
- Hidden feature leakage
- Build instability
- Impossible artifact isolation

---

## Forbidden Pattern

```toml
[dependencies]
canonrs-site = { path = "../../products/canonrs-site" }
```

---

## Canonical Flow

Allowed:

```
canonrs (design-system)
canonrs-ui
canonrs-interactive
canonrs-providers
canonrs-site (product)
```

Disallowed:

```
canonrs → canonrs-site
```

---

## Rationale

Products consume platform.
Platform never consumes product.

Architectural gravity must be downward.

---

## Enforcement

CI must:

- Detect dependencies from non-product crate to product crate
- Fail build if detected

---

## Exceptions

None.
