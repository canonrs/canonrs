# Canon Rule #60: CSS Artifacts Are Immutable

**Status:** ENFORCED


**Severity:** HIGH
**Scope:** build, css
**Version:** 1.0.0
**Date:** 2025-01-16

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

