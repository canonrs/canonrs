# Canon Rule #257: Tokens Engine Is a Mandatory Pre-Build Step

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Scope:** build, css, tokens
**Version:** 1.0.0  
**Date:** 2026-02-13  

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

