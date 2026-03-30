# Canon Rule #60: CSS Artifacts Are Immutable

**Status:** ENFORCED


**Severity:** HIGH
**Version:** 1.0.0
**Date:** 2025-01-16

**Category:** build-tooling
**Tags:** css, artifacts, build
**Language:** EN

---

**Intro:**
Editing generated CSS artifacts introduces drift and breaks reproducibility. All changes must originate from the generator and propagate via rebuild.

**Problem:**
manual edits in generated css create divergence from source

**Solution:**
treat css artifacts as immutable and regenerate on every change

**Signals:**
- manual css patch
- dist css edited
- inconsistent builds

**Search Intent:**
why generated css should not be edited manually

**Keywords:**
immutable build artifacts css, design system build consistency, css generation pipeline, no manual patch dist css

---

---

## Principle

Generated CSS artifacts are **immutable outputs**.

They are:
- not edited
- not post-processed manually
- not patched in applications

---

## Canon Law

If CSS needs change:
→ Fix generator  
→ Rebuild  
→ Replace artifact  

Never patch artifacts.

---

## Enforcement Checklist

- [ ] No commits editing `dist/*.css`
- [ ] No post-build mutation scripts
- [ ] Artifacts always regenerated
