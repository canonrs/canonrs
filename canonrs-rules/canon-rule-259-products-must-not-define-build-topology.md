# Canon Rule #259: Products Must Not Define Build Topology

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Scope:** architecture, build
**Version:** 1.0.0  
**Date:** 2026-02-13  

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

