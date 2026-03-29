# Canon Rule #255: CLI Is the Sole Authority Over Workspace Generation

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Scope:** cli, workspace
**Version:** 1.0.0  
**Date:** 2026-02-13  

---

## Principle

**The CanonRS CLI is the only authority allowed to generate, update, or restructure `.canonrs/workspace`.**

- Workspace topology is not user-defined.
- Products do not control workspace metadata.
- Profiles and Leptos metadata are injected exclusively by the CLI.
- Manual modifications are forbidden.

---

## Rationale

The workspace defines:

- Build graph
- Leptos metadata
- Release profiles
- Mode-specific behavior

If users edit this structure manually, architectural determinism is lost.

---

## Forbidden

❌ Editing `.canonrs/workspace/Cargo.toml` manually  
❌ Committing workspace topology changes directly  
❌ Defining workspace metadata in product Cargo.toml  

---

## Required

✅ Workspace must be generated through `canonrs dev` or `canonrs build`  
✅ Workspace content must always be reproducible  
✅ CLI must enforce topology deterministically  

---

This rule ensures CanonRS governs build topology, not the application.

