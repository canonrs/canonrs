# Canon Rule #253: Token Cascade Order Is Immutable

**Status:** ENFORCED
**Severity:** CRITICAL
**Scope:** css, tokens, architecture
**Version:** 1.0.0
**Date:** 2026-02-13

---

## Principle

Token generation order is fixed:

1. Primitives
2. Foundation
3. Themes
4. Semantic
5. Families
6. Root
7. Variants
8. UI
9. Blocks
10. Layouts
11. Globals

Reordering is forbidden without rule revision.

---

## Problem

Changing cascade order:

- Breaks variable resolution
- Produces silent style regressions
- Invalidates semantic mapping guarantees

---

## Enforcement

- entry_generator.rs must preserve canonical order
- CI must diff import order
- Any order change requires architecture review

---

## Rationale

Cascade order is architecture, not implementation detail.

---

## Exceptions

None.
