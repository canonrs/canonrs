# Canon Rule #90: Hydration Is DOM Replacement, Not Enhancement

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Scope:** hydration, ssr, leptos
**Version:** 1.0.0  
**Date:** 2026-01-14

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

