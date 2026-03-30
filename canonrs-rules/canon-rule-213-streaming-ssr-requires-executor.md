# Canon Rule #213: Streaming SSR Requires Explicit Executor Initialization

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Scope:** ssr, state
**Version:** 1.0.0  
**Date:** 2026-02-03

---

## Principle

**Streaming SSR MUST explicitly initialize an async executor before rendering.**

- Objective
- Verifiable
- One clear boundary

---

## Problem

Without executor initialization:

- Runtime panics occur
- Errors are silent or misleading
- Streaming hangs or crashes
- Debugging becomes impractical

---

## Forbidden Pattern

### Forbidden

```rust
#[tokio::main]
async fn main() {
    render_app();
}
```

No executor initialization.

---

## Canonical Pattern

### Canonical

```rust
#[tokio::main]
async fn main() {
    any_spawner::Executor::init_tokio();
    render_app();
}
```

Executor is explicitly defined.

---

## Rationale

Streaming SSR depends on async scheduling.

- Executors are not implicit
- Tooling does not auto-initialize
- Explicit init guarantees determinism

---

## Enforcement

- Build-time lint for missing initialization
- Runtime panic treated as rule violation
- Review checklist requirement

---

## Exceptions

> **No exceptions. This rule is absolute.**

---

## Version History

- **1.0.0** — Initial version
