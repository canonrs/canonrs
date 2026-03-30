# Canon Rule #256: Generated Workspace Is Ephemeral and Immutable

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Version:** 1.0.0  
**Date:** 2026-02-13  

**Category:** governance
**Tags:** workspace, build, artifacts, determinism
**Language:** EN

---

**Intro:**
Treating generated workspace as source code breaks reproducibility and build determinism. It must remain disposable.

**Problem:**
generated workspace is treated as source causing drift and instability

**Solution:**
treat workspace as ephemeral artifact and regenerate via cli

**Signals:**
- manual edit
- git commit workspace
- build inconsistency

**Search Intent:**
why generated workspace should not be versioned

**Keywords:**
ephemeral workspace build, generated workspace immutability, frontend build artifact governance, cli workspace regeneration

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
