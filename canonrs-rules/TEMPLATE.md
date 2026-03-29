path to save rules: /opt/docker/monorepo/libs/canonRS-rules
file name: canon-rule-XX-<title>.md

# Canon Rule #NNN: <Clear, Declarative Title>

**Status:** ENFORCED | PROPOSED | DEPRECATED  
**Severity:** CRITICAL | HIGH | MEDIUM | LOW  
**Scope:** <domains>  
**Version:** 1.0.0  
**Date:** YYYY-MM-DD

---

## Principle

**<One imperative, testable sentence defining the rule.>**

- Objective
- Verifiable
- One clear boundary

---

## Problem

What breaks **without** this rule.

- Observable symptoms
- Recurrent bugs or failures
- Architectural impact (SSR, hydration, coupling, maintenance)

---

## Forbidden Pattern

### ❌ Forbidden

```rust
// Incorrect usage
```

Why this violates the rule.

---

## Canonical Pattern

### ✅ Canonical

```rust
// Correct usage
```

Why this complies with the rule.

---

## Rationale

Why this rule exists **architecturally**.

- Invariants it protects
- Contracts it enforces
- Bugs it prevents
- Why it is not opinion

---

## Enforcement

How this rule is validated.

- Linter / CI / Static analysis
- Build-time failure
- Review checklist

---

## Exceptions

If exceptions exist, list **all conditions**.

Otherwise state explicitly:

> **No exceptions. This rule is absolute.**

---

## Version History

- **1.0.0** — Initial version
