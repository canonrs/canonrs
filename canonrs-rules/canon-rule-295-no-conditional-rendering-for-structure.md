# Canon Rule #295: No Conditional Rendering for Structure

**Status:** ENFORCED
**Severity:** CRITICAL
**Version:** 1.0.0
**Date:** 2026-04-02

**Category:** core-runtime
**Tags:** hydration, structure, conditional, ssr
**Language:** EN

---

**Intro:**
Conditional rendering that changes DOM structure breaks hydration guarantees.

**Problem:**
if/else modifies node structure between SSR and client

**Solution:**
use stable structure and conditionally change content or visibility

**Signals:**
- hydration mismatch
- missing nodes
- runtime panic

**Search Intent:**
how to avoid conditional dom mismatch in ssr

**Keywords:**
conditional rendering ssr, hydration mismatch leptos, stable dom structure, ssr best practices

---

## Principle

Structure must be constant.

---

## Problem

Conditional DOM:

- alters hierarchy
- breaks hydration
- causes runtime errors

---

## Contract

- no structural branching
- only content variation allowed

---

## Version History

- 1.0.0 — Initial definition
