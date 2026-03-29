# Canon Rule #92: If It Works in Prod but Not Dev, Suspect Hydration Order

**Status:** ENFORCED  
**Severity:** MEDIUM  
**Scope:** hydration, ssr
**Version:** 1.0.0  
**Date:** 2026-01-14

---

## Principle

**Behavior differences between dev and prod indicate hydration or script timing issues.**

---

## Heuristic

```text
Works in prod
Fails in dev
No JS errors
→ Hydration order violation
```

---

## Immediate Checks

1. Script location (must be in shell)
2. Event delegation usage
3. Primitive touching browser APIs
4. Pre-hydration execution

---

## Purpose

- Prevents false bug hunts
- Reduces debugging time
- Encodes institutional knowledge

---

## Related Rules

- Canon Rule #87 — Leptos SSR Script Placement
- Canon Rule #90 — Hydration Is DOM Replacement

