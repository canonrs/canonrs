# Canon Rule #254: CanonRS Must Build Independently of Products

**Status:** ENFORCED
**Severity:** CRITICAL
**Version:** 1.0.0
**Date:** 2026-02-13

**Category:** build-tooling
**Tags:** build, design-system, independence, architecture
**Language:** EN

---

**Intro:**
Design system depending on products creates coupling and blocks independent versioning. Build must remain isolated.

**Problem:**
design system depends on products causing coupling and circular dependencies

**Solution:**
ensure design system builds independently without product context

**Signals:**
- build failure
- dependency cycle
- artifact coupling

**Search Intent:**
how to decouple design system from products

**Keywords:**
design system independent build, frontend architecture decoupling, rust workspace isolation, design system versioning independence

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