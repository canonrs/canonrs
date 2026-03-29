# Canon Rule #277: Interactive Must Not Emit DOM Events

**Status:** ENFORCED  
**Severity:** HIGH  
**Scope:** interactive
**Version:** 1.0.0  
**Date:** 2026-02-13

---

## Principle

**Interactive layer must never dispatch DOM events directly.**

---

## Problem

Violates separation between runtime and state orchestration.

---

## Enforcement

DOM dispatch limited to Behavior only.

---

## Exceptions

None.

---

## Version History

- 1.0.0 — Initial version
