# Canon Rule #176: Governance Is the Single Source of Truth

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Scope:** governance, architecture, design-system
**Version:** 1.0.0  
**Date:** 2026-01-29

---

## Principle

**All authoritative decisions in CanonRS flow through governance.**

Governance is the **only layer** allowed to:
- interpret contracts
- resolve components
- validate mappings
- enforce invariants

No other layer may override or bypass it.

---

## Definition

The **governance layer** is defined as:

```
packages-rust/rs-design/src/design/governance/
```

This layer represents the **canonical brain** of the design system.

---

## Responsibilities

Governance MUST:

- Resolve which tokens a component may consume
- Enforce family boundaries
- Validate contract compliance
- Detect inventory drift
- Fail fast on architectural violations
- Produce deterministic outcomes

Governance is **authoritative**, not advisory.

---

## Forbidden Patterns

### Forbidden

- UI resolving its own tokens
- Blocks inferring behavior dynamically
- Layouts bypassing governance
- “Helper logic” that duplicates governance rules
- Runtime decision-making that changes design contracts

```rust
// ❌ WRONG
if component == "alert" {
    use_token("alert-bg-error");
}
```

```rust
// ❌ WRONG
component.resolve_tokens_locally();
```

---

## Canonical Patterns

### Canonical

- Governance resolves → others consume
- Governance validates → build enforces
- Governance is queried, never copied

```rust
// ✅ CORRECT
let resolved = governance::resolve(component_id)?;
```

---

## Change Policy

Any change to governance logic is **BREAKING**.

Required steps:

1. Explicit version bump
2. Canon Rule update (if behavior changes)
3. Governance test update
4. Full CI validation

Silent logic changes are forbidden.

---

## Enforcement

### Build Time

- Governance is compiled as a mandatory dependency
- Token resolution flows through governance only
- Missing or bypassed governance causes build failure

### CI

Build MUST FAIL if:

- A component resolves tokens without governance
- Governance rules are duplicated elsewhere
- Inventory or mapping differs from governance output

---

## Rationale

Without a single source of truth:

- Rules fork silently
- Fixes diverge
- Components behave inconsistently
- Architecture becomes opinion-based

With governance:

- Behavior is deterministic
- Drift is detectable
- Refactors are safe
- CanonRS behaves like a real framework

---

## Relationship to Other Rules

- Rule #165 — CanonRS Workbench Is a Canonical Reference
- Rule #170 — HTML and CSS Must Share the Same Contract
- Rule #174 — Tokens Are Compile-Time Contracts
- Rule #175 — UI Inventory Is Fixed and Canonical
- Rule #178 — Contracts Are Read-Only APIs

---

## Canon Outcome

- Governance decides
- Others comply
- Architecture is enforced
- Drift is impossible

---

**If it is not governed, it is not canon.**

