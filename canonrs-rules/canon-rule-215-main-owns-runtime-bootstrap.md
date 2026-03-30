# Canon Rule #215: main.rs Owns Runtime and Server Bootstrap Exclusively

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Scope:** ssr, architecture
**Version:** 1.0.0  
**Date:** 2026-02-03

---

## Principle

**`main.rs` MUST contain only runtime initialization and server bootstrap logic.**

- Objective
- Verifiable
- One clear boundary

---

## Problem

When `main.rs` contains application logic:

- Build features become entangled
- SSR/CSR branches become unsafe
- UI logic is duplicated or conditionally compiled
- Bootstrap order becomes fragile

This breaks determinism and makes the application untestable in isolation.

---

## Forbidden Pattern

### Forbidden

```rust
#[tokio::main]
async fn main() {
    view! { <App/> }
}
```

Rendering logic in the bootstrap layer.

---

## Canonical Pattern

### Canonical

```rust
#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    any_spawner::Executor::init_tokio();
    start_server();
}
```

Bootstrap only. No UI semantics.

---

## Rationale

Bootstrap is infrastructural.

- It wires runtimes, servers, and ports
- It must be replaceable without touching UI
- It must never affect application semantics

This separation is mandatory for enterprise-grade systems.

---

## Enforcement

- CI: fail if `view!` macro appears in `main.rs`
- Code review checklist
- Static analysis

---

## Exceptions

> **No exceptions. This rule is absolute.**

---

## Version History

- **1.0.0** — Initial version
