# Canon Rule #259: Products Must Not Define Build Topology

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Version:** 1.0.0  
**Date:** 2026-02-13  

**Category:** build-tooling
**Tags:** architecture, build, workspace, cli
**Language:** EN

---

**Intro:**
Allowing products to control build topology creates inconsistency and breaks framework governance. Build structure must remain centralized.

**Problem:**
products define build topology causing inconsistency and architectural drift

**Solution:**
delegate all build topology control exclusively to framework cli

**Signals:**
- config drift
- workspace mismatch
- build inconsistency

**Search Intent:**
why products should not define build topology

**Keywords:**
build topology control cli, workspace governance rust, frontend architecture separation, leptos metadata ownership

---

## Principle

**Products define business logic only. Build topology is governed exclusively by CanonRS CLI.**

Products must not control:

- Workspace layout
- Leptos metadata
- Build targets
- Profile configuration

---

## Architectural Separation

Product responsibilities:
- Domain logic
- UI composition
- Routes
- Providers

Framework responsibilities:
- CSS cascade
- Profile injection
- Workspace structure
- Compilation strategy

---

## Forbidden

❌ Defining `[workspace]` inside product  
❌ Overriding leptos metadata  
❌ Custom target management  

---

This protects framework integrity and ensures multi-product consistency.
