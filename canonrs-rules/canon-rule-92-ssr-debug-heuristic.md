# Canon Rule #92: If It Works in Prod but Not Dev, Suspect Hydration Order

**Status:** ENFORCED  
**Severity:** MEDIUM  
**Version:** 1.0.0  
**Date:** 2026-01-14

**Category:** governance
**Tags:** debugging, ssr, hydration
**Language:** EN

---

**Intro:**
Differences between development and production behavior often indicate hydration timing issues. Recognizing this pattern reduces debugging time.

**Problem:**
developers misdiagnose issues caused by hydration order differences

**Solution:**
use heuristic to identify hydration timing problems early

**Signals:**
- works in prod fails in dev
- no errors
- inconsistent behavior

**Search Intent:**
why app works in prod but

**Keywords:**
ssr debug heuristic, hydration timing issue, dev vs prod behavior, leptos hydration debugging

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
