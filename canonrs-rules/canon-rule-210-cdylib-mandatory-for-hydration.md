# Canon Rule #210: cdylib Is Mandatory for Hydration

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Version:** 1.0.0  
**Date:** 2026-02-03

**Category:** build-tooling
**Tags:** wasm, hydration, build, rust
**Language:** EN

---

**Intro:**
Missing crate type prevents wasm generation and causes silent hydration failures. Explicit configuration is required.

**Problem:**
cdylib crate type is missing causing wasm generation failure

**Solution:**
declare cdylib crate type to ensure proper wasm output

**Signals:**
- no wasm
- hydration fail
- build error

**Search Intent:**
how to fix missing cdylib wasm hydration

**Keywords:**
cdylib wasm rust, leptos hydration build error, wasm crate type config, frontend rust wasm setup

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

### Forbidden

```toml
[package]
name = "app"
```

No explicit library crate type for WASM output.

---

## Canonical Pattern

### Canonical

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