# Canon Rule #255: CLI Is the Sole Authority Over Workspace Generation

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Version:** 1.0.0  
**Date:** 2026-02-13  

**Category:** governance
**Tags:** cli, workspace, architecture, generation
**Language:** EN

---

**Intro:**
Manual workspace edits break determinism and architecture control. Generation must be centralized.

**Problem:**
workspace structure is modified manually causing instability and drift

**Solution:**
delegate all workspace generation exclusively to cli

**Signals:**
- config drift
- build mismatch
- manual override

**Search Intent:**
why cli should control workspace generation

**Keywords:**
workspace generation cli control, build topology governance, frontend monorepo cli authority, cargo workspace automation

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
