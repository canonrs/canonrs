# Canon Rule #90: Hydration Is DOM Replacement, Not Enhancement

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Version:** 1.0.0  
**Date:** 2026-01-14

**Category:** core-runtime
**Tags:** hydration, ssr, dom
**Language:** EN

---

**Intro:**
Assuming hydration enhances existing DOM leads to broken interactions and lost event bindings. Hydration replaces nodes entirely, invalidating pre-existing references.

**Problem:**
developers assume dom persists after hydration causing broken behavior

**Solution:**
treat hydration as dom replacement and avoid relying on node identity

**Signals:**
- event lost after hydration
- inconsistent behavior
- silent failures

**Search Intent:**
why hydration breaks dom references

**Keywords:**
hydration dom replacement, leptos hydration behavior, ssr dom identity issue, event listener lost hydration

---

## Principle

**Hydration replaces the DOM. It does not enhance it.**

Any assumption that pre-hydration DOM nodes persist is invalid.

---

## Consequences

- Node identity is not preserved
- Direct JS bindings are lost
- Pre-hydration listeners are discarded

---

## Common Failure Symptoms

- Works in prod, fails in dev
- Click detected but no action
- Correct DOM, broken behavior
- Silent runtime failures

---

## Canonical Response

- Use event delegation
- Inject JS after SSR render
- Never depend on node identity

---

## Related Rules

- Canon Rule #87 — Leptos SSR Script Placement
- Canon Rule #88 — Event Delegation Only
