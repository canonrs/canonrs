# Canon Rule #263: Workspace Crate Boundaries Must Be Explicitly Declared

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Scope:** workspace, architecture
**Version:** 1.0.0  
**Date:** 2026-02-13

---

## Principle

Every crate inside the CanonRS workspace MUST declare its architectural role explicitly in its `Cargo.toml`.

Crates MUST identify themselves as one of:

- `design-system`
- `ui`
- `interactive`
- `tokens-engine`
- `server`
- `shared`
- `providers`
- `cli`
- `product`
- `tooling`

---

## Problem

Without declared architectural roles:

- Crates drift in responsibility
- Dependency layering becomes unclear
- Cyclic coupling risk increases
- Refactors break invisible contracts

---

## Canonical Pattern

```toml
[package.metadata.canonrs]
role = "design-system"
layer = "core"
```

---

## Rationale

Architecture must not be implicit.
Cargo knows crates.
CanonRS must know _what they are_.

This rule enforces semantic clarity at crate boundary level.

---

## Enforcement

- CI must validate presence of `[package.metadata.canonrs]`
- Role must be one of the allowed values
- Layer hierarchy must not be violated

---

## Exceptions

No exceptions.

All CanonRS crates must self-declare role.
