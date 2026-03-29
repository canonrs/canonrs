# Canon Rule #254: CanonRS Must Build Independently of Products

**Status:** ENFORCED
**Severity:** CRITICAL
**Scope:** build, design-system
**Version:** 1.0.0
**Date:** 2026-02-13

---

## Principle

CanonRS (tokens-engine + UI + CSS) MUST compile and generate artifacts without any product crate present.

---

## Problem

If CanonRS depends on product context:

- Circular dependency appears
- Artifact governance breaks
- CLI orchestration becomes mandatory
- Design system cannot be versioned independently

---

## Canonical Condition

From:

packages-rust/rs-canonrs/

The following must work:

cargo build
cargo run --bin tokens-engine

Without any product.

---

## Forbidden

- Importing product paths
- Accessing product config
- Reading product Leptos.toml

---

## Rationale

Design system is infrastructure.
Infrastructure must be product-agnostic.

---

## Exceptions

None.
