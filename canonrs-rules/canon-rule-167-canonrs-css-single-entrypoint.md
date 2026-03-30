# Canon Rule #167: CanonRS CSS Has a Single Entrypoint

**Status:** ENFORCED  
**Severity:** HIGH  
**Version:** 1.0.0  
**Date:** 2026-01-28

**Category:** styling-css
**Tags:** css, entrypoint, imports, architecture
**Language:** EN

---

**Intro:**
Multiple CSS entrypoints cause inconsistent imports, ordering issues, and missing tokens. This leads to unpredictable styling behavior across applications.

**Problem:**
multiple css entrypoints cause ordering bugs and missing styles

**Solution:**
expose a single canonical css entrypoint for all applications

**Signals:**
- missing tokens
- import drift
- css inconsistency

**Search Intent:**
how to enforce single css entrypoint design system

**Keywords:**
css single entrypoint pattern, design system css import order, frontend css architecture, canonical css bundle

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
