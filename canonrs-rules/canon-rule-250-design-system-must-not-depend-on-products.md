# Canon Rule #250: Design System Must Not Depend on Products

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Version:** 1.0.0  
**Date:** 2026-02-13  

**Category:** component-architecture
**Tags:** workspace, dependencies, architecture, graph
**Language:** EN

---

**Intro:**
Reverse dependencies from design system to products create cycles and break architectural layering. Dependency direction must be strictly enforced.

**Problem:**
design system depends on product crates causing cycles and instability

**Solution:**
enforce one directional dependency flow from core to products only

**Signals:**
- dependency cycle
- build instability
- feature leakage

**Search Intent:**
how to prevent dependency cycles design system

**Keywords:**
design system dependency rules, rust workspace dependency direction, avoid circular dependencies frontend, architecture layering enforcement

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