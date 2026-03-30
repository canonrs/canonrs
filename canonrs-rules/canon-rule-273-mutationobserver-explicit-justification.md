# Canon Rule #273: MutationObserver Requires Explicit Architectural Justification

**Status:** ENFORCED  
**Severity:** HIGH  
**Version:** 1.0.0  
**Date:** 2026-02-13

**Category:** behavior
**Tags:** mutationobserver, performance, dom, behavior
**Language:** EN

---

**Intro:**
Uncontrolled MutationObserver usage degrades performance and causes layout thrashing. Usage must be justified.

**Problem:**
mutationobserver is used without control causing performance degradation

**Solution:**
require explicit architectural approval before using mutationobserver

**Signals:**
- performance drop
- layout thrash
- observer spam

**Search Intent:**
when to use mutationobserver safely

**Keywords:**
mutationobserver performance issues, dom observer control pattern, frontend performance dom watch, behavior dom monitoring rules

---

## Principle

**MutationObserver must never be used implicitly.**

---

## Problem

Uncontrolled observers cause:

- Performance degradation
- Layout thrashing

---

## Enforcement

Mandatory architecture review approval.

---

## Exceptions

Documented reactive DOM sync only.

---

## Version History

- 1.0.0 — Initial version