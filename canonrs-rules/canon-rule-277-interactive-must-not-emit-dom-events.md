# Canon Rule #277: Interactive Must Not Emit DOM Events

**Status:** ENFORCED  
**Severity:** HIGH  
**Version:** 1.0.0  
**Date:** 2026-02-13

**Category:** behavior
**Tags:** interactive, events, runtime, architecture
**Language:** EN

---

**Intro:**
Emitting DOM events from the interactive layer breaks separation between runtime behavior and state orchestration. This introduces hidden coupling and unpredictable execution flows.

**Problem:**
interactive layer dispatches dom events directly breaking separation of concerns

**Solution:**
restrict dom event dispatching exclusively to behavior layer

**Signals:**
- custom event dispatch
- event bubbling misuse
- runtime coupling

**Search Intent:**
why interactive layer should not dispatch dom events

**Keywords:**
dom event dispatch architecture, interactive vs behavior separation, frontend event layer design, runtime event coupling issues

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