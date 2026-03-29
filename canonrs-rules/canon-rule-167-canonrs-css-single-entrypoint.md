# Canon Rule #167: CanonRS CSS Has a Single Entrypoint

**Status:** ENFORCED  
**Severity:** HIGH  
**Scope:** css, architecture
**Version:** 1.0.0  
**Date:** 2026-01-28

---

## Principle

**CanonRS must expose exactly one canonical CSS entrypoint.**

---

## Problem

Multiple CSS entrypoints cause import drift, ordering bugs, and missing tokens.

---

## Forbidden Pattern

❌ Importing individual UI CSS files in applications.

---

## Canonical Pattern

✅ Import only canonrs.css, which defines the full order.

---

## Enforcement

- Static analysis
- CSS import validation

---

## Exceptions

None.

