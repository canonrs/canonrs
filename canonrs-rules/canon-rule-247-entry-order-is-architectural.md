# Canon Rule #247: CSS Entry Order Is Architectural, Not Cosmetic

**Status:** ENFORCED
**Severity:** CRITICAL
**Version:** 1.0.0
**Date:** 2026-02-13

**Category:** styling-css
**Tags:** css, order, tokens, architecture
**Language:** EN

---

**Intro:**
Entry order defines authority in cascade and cannot be changed without breaking system behavior. Order is structural.

**Problem:**
entry order changes cause instability and incorrect overrides

**Solution:**
enforce fixed entry generation order and forbid manual edits

**Signals:**
- override break
- token shadowing
- inconsistent ui

**Search Intent:**
why css entry order matters architecture

**Keywords:**
css entry order architecture, cascade authority layers, design system import order, frontend css generation order

---

## Principle

The entry generation order in canonrs.css is a hard architectural contract.

Order defines authority.

---

## Canonical Order

1. PRIMITIVES
2. FOUNDATION (core)
3. THEMES
4. SEMANTIC
5. FAMILIES
6. ROOT
7. VARIANTS
8. UI
9. BLOCKS
10. LAYOUTS
11. GLOBALS (final overrides)

---

## Problem

Changing order causes:

- Semantic shadowing
- Root override loss
- UI consuming unstable tokens
- Non-deterministic visual output

---

## Forbidden Pattern

```
@import "./.generated/root.css";
@import "./.generated/semantic.css";  ❌ wrong order
```

---

## Why This Is Not Optional

Cascade order equals system authority.

Reordering = rewriting architecture.

---

## Enforcement

- entry_generator hardcodes order
- Any manual edit to canonrs.css is forbidden
- CI verifies import order

---

## Exceptions

None.

Entry order is immutable unless versioned major rewrite.

---

## Version History

- 1.0.0 — Initial definition