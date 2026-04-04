# Canon Rule #327: Signals Are the SSOT for Island State

**Status:** ENFORCED
**Severity:** CRITICAL
**Version:** 1.0.0
**Date:** 2026-04-04

**Category:** islands-architecture
**Tags:** islands, signals, state, leptos
**Language:** EN

---

**Intro:**
In Leptos islands, signals are the single source of truth. DOM attributes are outputs, never inputs.

**Problem:**
DOM mutation and attribute reading as state source breaks the reactive model

**Solution:**
derive all state from signals initialized from props

**Signals:**
- reading data-rs-* attributes as state
- set_attribute called directly
- DOM scanning inside island

**Search Intent:**
leptos island signal state management

**Keywords:**
leptos signal SSOT, island state management, reactive state leptos, DOM attribute output

---

## Principle

State lives in signals. DOM reflects signals. Never the reverse.

---

## Problem

The behavior registry pattern reads DOM attributes as state and mutates them directly. This is architecturally incompatible with islands. Islands have a reactive owner and signals as their state model.

---

## Patterns

### Forbidden Pattern

Reading get_attribute to get state, calling set_attribute to update state, or scanning the DOM to find and update elements.

### Canonical Pattern

Signal initialized from prop. Reactive attributes in view! derive their values from the signal. No DOM reads or writes outside the reactive system.

---

## Contract

### Enforcement

- island state must live in signal()
- data-rs-* attributes must be reactive outputs of signals
- no set_attribute, get_attribute, or DOM scanning inside islands
- behavior registry pattern is forbidden inside islands

### Exceptions

None.

---

## Version History

- 1.0.0 - Initial definition
