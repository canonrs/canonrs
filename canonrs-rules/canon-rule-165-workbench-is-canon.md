# Canon Rule #165: CanonRS Workbench Is a Canonical Reference

**Status:** ENFORCED  
**Severity:** HIGH  
**Version:** 1.0.0  
**Date:** 2026-01-27

**Category:** governance
**Tags:** architecture, reference, canon, design
**Language:** EN

---

**Intro:**
Ignoring working reference implementations leads to architectural drift and inconsistent decisions. Proven patterns must guide design decisions.

**Problem:**
architecture ignores working reference leading to inconsistency

**Solution:**
treat workbench implementation as canonical source of truth

**Signals:**
- reinventing patterns
- inconsistency
- architecture drift

**Search Intent:**
how to use reference implementation as architecture guide

**Keywords:**
reference implementation architecture, canonical design patterns, frontend architecture governance, workbench as source of truth

---

## Principle

**A working CanonRS Workbench implementation defines canonical behavior.**

- Objective
- Verifiable
- One clear boundary

---

## Problem

Ignoring the workbench leads to theoretical architecture drift.

- Duplicate decisions
- Conflicting patterns
- Reinvented solutions
- Fragmented framework behavior

---

## Forbidden Pattern

### ❌ Forbidden

```text
“This is better than the workbench, let’s change it.”
```

Without evidence.

---

## Canonical Pattern

### ✅ Canonical

```text
“If the workbench does it and works, it is canon.”
```

Why this complies with the rule.

---

## Rationale

CanonRS canon is empirical, not speculative.

- Protects proven architecture
- Reduces decision entropy
- Anchors rules in reality
- Enterprise-grade stability

---

## Enforcement

- Architecture review
- Reference comparison
- Documentation alignment

---

## Exceptions

> **No exceptions. This rule is absolute.**

---

## Version History

- **1.0.0** — Initial version