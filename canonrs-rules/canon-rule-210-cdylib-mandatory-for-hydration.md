# Canon Rule #210: cdylib Is Mandatory for Hydration

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Scope:** build, wasm, hydration
**Version:** 1.0.0  
**Date:** 2026-02-03

---

## Principle

**Any Leptos application that supports hydration MUST declare `cdylib` as a crate type.**

- Objective
- Verifiable
- One clear boundary

---

## Problem

Without `cdylib`:

- WASM artifacts are not generated
- `wasm-bindgen` fails late
- Hydration silently breaks
- Build errors appear far from the root cause

This creates false-positive “successful” builds.

---

## Forbidden Pattern

### ❌ Forbidden

```toml
[package]
name = "app"
```

No explicit library crate type for WASM output.

---

## Canonical Pattern

### ✅ Canonical

```toml
[lib]
crate-type = ["cdylib", "rlib"]
```

This guarantees correct WASM emission.

---

## Rationale

Hydration is a cross-compilation contract.

- WASM output is mandatory
- Tooling assumes `cdylib`
- Implicit defaults are unsafe
- This prevents late-stage failures

---

## Enforcement

- CI validates presence of `cdylib`
- Missing crate-type is a build error
- Review checklist item

---

## Exceptions

> **No exceptions. This rule is absolute.**

---

## Version History

- **1.0.0** — Initial version
