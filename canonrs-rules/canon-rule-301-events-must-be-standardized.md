# Canon Rule #301: Events Must Be Standardized

**Status:** ENFORCED
**Severity:** HIGH
**Version:** 1.0.0
**Date:** 2026-04-02

**Category:** behavior
**Tags:** events, standardization, architecture
**Language:** EN

---

**Intro:**
Event naming and structure must be standardized to ensure interoperability.

**Problem:**
custom or inconsistent event naming breaks integration

**Solution:**
use canonical event names such as rs-change

**Signals:**
- inconsistent event names
- integration issues
- duplicated logic

**Search Intent:**
how to standardize events in ui systems

**Keywords:**
event naming frontend, standardized events ui, rs-change pattern, component event architecture

---

## Principle

Events must be predictable.

---

## Problem

Custom events:

- break integration
- increase complexity

---

## Contract

- use canonical event names
- follow system-wide standards

---

## Version History

- 1.0.0 — Initial definition
