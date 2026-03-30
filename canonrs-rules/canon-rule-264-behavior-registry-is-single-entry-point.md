# Canon Rule #264: Behavior Registry Is the Single Runtime Entry Point

**Status:** ENFORCED
**Severity:** CRITICAL
**Version:** 1.0.0
**Date:** 2026-02-13

**Category:** behavior
**Tags:** behavior, runtime, wasm, registry
**Language:** EN

---

**Intro:**
Multiple behavior initialization paths cause duplication and nondeterministic runtime behavior. A single entry point is required.

**Problem:**
behaviors are initialized in multiple places causing duplication and instability

**Solution:**
centralize all behavior initialization through a single registry bootstrap

**Signals:**
- duplicate listeners
- init conflict
- runtime anomaly

**Search Intent:**
how to centralize behavior initialization

**Keywords:**
behavior registry pattern, single runtime entrypoint wasm, frontend behavior initialization, leptos behavior bootstrap

---

## Principle

**All behaviors MUST be registered exclusively through the Behavior Registry and initialized via a single runtime bootstrap call.**

- Single entry point
- No parallel init paths
- Deterministic runtime wiring

---

## Problem

Without this rule:

- Multiple init systems compete
- Behaviors attach twice
- Order-dependent bugs appear
- MutationObserver conflicts
- Inconsistent CSR activation

Observable symptoms:

- `init_x()` called manually per component
- Different apps registering behaviors differently
- Duplicate listeners
- Random runtime anomalies

Architectural impact:

- Breaks deterministic hydration
- Breaks runtime uniformity across apps
- Destroys plug-and-play architecture

---

## Forbidden Pattern

### Forbidden

```rust
// Manual init inside component
resize_handle_behavior::register();
setup_resize(container);

// Direct init without registry
init_sidebar();
```

Why this violates the rule:

It bypasses centralized discovery and lifecycle control.

---

## Canonical Pattern

### Canonical

```rust
#[cfg(feature = "hydrate")]
pub fn hydrate() {
    leptos::mount::hydrate_body(App);
    canonrs::behaviors::init_canonrs_behaviors();
}
```

Behavior internally:

```rust
register_behavior("data-resize-container", Box::new(...));
```

Why this complies:

- Single bootstrap
- Central registry control
- MutationObserver unified
- Zero per-component setup

---

## Rationale

This protects:

- Runtime determinism
- Zero-config consumption
- Portable app integration
- CanonRS behavior auto-discovery model

Registry is architectural infrastructure.
It is not optional plumbing.

---

## Enforcement

- CI blocks any `init_*` functions outside registry
- Code review rejects manual event listener attachment without registry
- All behaviors must expose `register()`
- Bootstrap must contain exactly one `init_canonrs_behaviors()`

---

## Exceptions

> **No exceptions. This rule is absolute.**

---

## Version History

- **1.0.0** — Initial version



# ===========================================================


