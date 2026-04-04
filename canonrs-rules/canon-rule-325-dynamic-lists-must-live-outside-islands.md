# Canon Rule #325: Dynamic Lists Must Live Outside Islands

**Status:** ENFORCED
**Severity:** CRITICAL
**Version:** 1.0.0
**Date:** 2026-04-04

**Category:** islands-architecture
**Tags:** islands, lists, hydration, leptos
**Language:** EN

---

**Intro:**
Dynamic lists rendered inside islands cause hydration marker mismatches in Leptos 0.8.

**Problem:**
iterators, For component and inner_html inside islands break hydration

**Solution:**
move dynamic lists to SSR shell components outside the island

**Signals:**
- failed_to_cast_marker_node at list position
- hydration panic on collection render
- tachys cursor error inside island

**Search Intent:**
leptos island dynamic list hydration error

**Keywords:**
leptos island list, For component island, iterator island hydration, island children dynamic

---

## Principle

Islands must have a fixed number of child nodes. Dynamic collections must be rendered in SSR shell components.

---

## Problem

All dynamic list patterns inside islands cause hydration marker mismatch in Leptos 0.8: iterators with collect, For component, and inner_html with dynamic content.

---

## Patterns

### Forbidden Pattern

Rendering a dynamic list of marks or items inside the island view.

### Canonical Pattern

Island has a fixed DOM shape. A SSR shell component wraps the island and renders the dynamic list outside of it.

---

## Contract

### Enforcement

- islands must not contain iterators, For, or inner_html with dynamic content
- dynamic collections belong in component shell wrappers
- island child count must be statically known at compile time

### Exceptions

None.

---

## Version History

- 1.0.0 - Initial definition
