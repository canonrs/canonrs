# Canon Rule #324: Island View Must Be Isomorphic

**Status:** ENFORCED
**Severity:** CRITICAL
**Version:** 1.0.0
**Date:** 2026-04-04

**Category:** islands-architecture
**Tags:** islands, hydration, isomorphic, leptos
**Language:** EN

---

**Intro:**
The first render of an island must produce identical output on SSR and client.

**Problem:**
SSR and hydrate produce different HTML causing hydration panic

**Solution:**
initialize signals with deterministic values derived from props

**Signals:**
- hydration mismatch
- cursor panic
- different attribute values SSR vs client

**Search Intent:**
leptos island isomorphic render SSR hydrate

**Keywords:**
isomorphic rendering leptos, SSR hydration match, island first render, deterministic island

---

## Principle

The initial render of an island must be bit-for-bit identical between SSR and client hydration.

---

## Problem

When signals are initialized with non-deterministic values or reactive attributes produce different output on SSR vs client, the hydration cursor panics.

---

## Patterns

### Forbidden Pattern

Reactive attributes that compute differently on SSR vs hydrate, producing different initial HTML.

### Canonical Pattern

Signals initialized from props. Same prop value on SSR and hydrate produces identical initial HTML.

---

## Contract

### Enforcement

- signals must be initialized from props, not from runtime state
- reactive attributes must produce the same initial value on SSR and hydrate
- no random, time-based, or environment-dependent initial values

### Exceptions

None.

---

## Version History

- 1.0.0 - Initial definition
