# Canon Rule #213: Streaming SSR Requires Explicit Executor Initialization

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Version:** 1.0.0  
**Date:** 2026-02-03

**Category:** core-runtime
**Tags:** ssr, async, executor, streaming
**Language:** EN

---

**Intro:**
Streaming SSR fails silently or crashes when no async executor is initialized. Deterministic scheduling requires explicit executor setup.

**Problem:**
streaming ssr runs without executor causing runtime failures

**Solution:**
initialize async executor explicitly before rendering

**Signals:**
- runtime panic
- stream hang
- async failure

**Search Intent:**
how to fix streaming ssr executor init

**Keywords:**
streaming ssr executor rust, async executor initialization, leptos streaming issue, tokio executor setup

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