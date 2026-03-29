# Canon Rule #256: Generated Workspace Is Ephemeral and Immutable

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Scope:** workspace, governance
**Version:** 1.0.0  
**Date:** 2026-02-13  

---

## Principle

**`.canonrs/workspace` is an ephemeral artifact and must never be treated as source code.**

- It is generated.
- It is disposable.
- It is reproducible.
- It is immutable by humans.

---

## Characteristics

- May be deleted at any time.
- Always regenerable via CLI.
- Must not be version controlled.
- Must not be manually edited.

---

## Architectural Impact

This enforces:

- Deterministic builds
- Centralized profile governance
- Canonical build topology
- Clean product boundaries

---

## Forbidden

❌ Adding workspace to git  
❌ Editing generated files  
❌ Injecting manual build flags  

---

The workspace exists to isolate CanonRS build mechanics from product code.

