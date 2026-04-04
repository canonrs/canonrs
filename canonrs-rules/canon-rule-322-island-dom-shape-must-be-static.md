# Canon Rule #322: Island DOM Shape Must Be Static

**Status:** ENFORCED
**Severity:** CRITICAL
**Version:** 1.0.0
**Date:** 2026-04-04

**Category:** islands-architecture
**Tags:** islands, hydration, dom, leptos
**Language:** EN

---

**Intro:**
The DOM structure rendered by an island must be identical between SSR and hydration.

**Problem:**
islands with dynamic DOM shape cause hydration mismatch and runtime panic

**Solution:**
keep island view structure static and fixed

**Signals:**
- failed_to_cast_marker_node
- hydration panic
- tachys cursor error

**Search Intent:**
leptos island hydration mismatch fix

**Keywords:**
leptos islands hydration, island dom shape, tachys hydration error, SSR hydrate mismatch

---

## Principle

An island view! macro must produce identical DOM structure on both SSR and client hydration.

---

## Problem

When an island renders different DOM structures depending on runtime conditions or cfg flags, the tachys hydration cursor panics with failed_to_cast_marker_node.

---

## Patterns

### Forbidden Pattern

Two different view! blocks separated by cfg — one for SSR and one for hydrate — produce different DOM structures and always cause hydration panic.

### Canonical Pattern

One single view! block. cfg is permitted only for event handlers and logic, never for the view structure itself.

---

## Contract

### Enforcement

- one view! block per island
- cfg is permitted only for event handlers and logic
- cfg must never alter the DOM structure

### Exceptions

None.

---

## Version History

- 1.0.0 - Initial definition
