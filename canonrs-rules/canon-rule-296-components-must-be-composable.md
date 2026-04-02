# Canon Rule #296: Components Must Be Composable

**Status:** ENFORCED
**Severity:** HIGH
**Version:** 1.0.0
**Date:** 2026-04-02

**Category:** component-architecture
**Tags:** composition, architecture, integration
**Language:** EN

---

**Intro:**
Components must integrate seamlessly with other system parts. Isolation breaks system value.

**Problem:**
components cannot interact with others

**Solution:**
design components with standardized contracts

**Signals:**
- isolated components
- integration failure
- duplicated logic

**Search Intent:**
how to design composable components

**Keywords:**
component composition frontend, reusable ui architecture, composable design system, integration patterns

---

## Principle

Components are building blocks.

---

## Problem

Non-composable components:

- cannot scale
- increase complexity
- reduce reuse

---

## Contract

- must follow canonical contracts
- must emit events and expose state

---

## Version History

- 1.0.0 — Initial definition
