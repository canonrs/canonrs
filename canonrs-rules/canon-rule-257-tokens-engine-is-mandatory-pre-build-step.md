# Canon Rule #257: Tokens Engine Is a Mandatory Pre-Build Step

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Version:** 1.0.0  
**Date:** 2026-02-13  

**Category:** build-tooling
**Tags:** build, tokens, css, pipeline
**Language:** EN

---

**Intro:**
Skipping token generation leads to incomplete css cascade and inconsistent builds. Generation must be enforced.

**Problem:**
tokens engine is not executed before build causing missing css

**Solution:**
enforce tokens engine execution before any build or dev process

**Signals:**
- missing css
- style break
- build warning

**Search Intent:**
how to enforce prebuild token generation

**Keywords:**
tokens engine prebuild step, css generation pipeline, frontend build order enforcement, design system build sequence

---

## Principle

**The tokens-engine must execute before any CanonRS application build or development server starts.**

No application may compile without a fully generated CSS cascade.

---

## Build Order

1. tokens-engine generates CSS
2. Workspace topology validated/generated
3. cargo leptos watch/build executes

This order is mandatory and non-bypassable.

---

## Architectural Boundaries

- CSS is not handwritten for runtime.
- CSS is not generated during application compilation.
- tokens-engine is the sole authority over cascade generation.

---

## Enforcement

- CLI must always execute tokens-engine first.
- Direct `cargo build` is unsupported.
- Missing generated CSS must raise warnings.

---

This ensures styling architecture remains deterministic and centralized.
