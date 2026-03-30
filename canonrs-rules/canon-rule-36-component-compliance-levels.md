# Canon Rule #36: Component Compliance Levels

**Status:** ENFORCED

**Severity:** MEDIUM
**Version:** 1.0.0
**Date:** 2025-01-16


## Principle
Components are classified by **compliance level** to balance canon purity with real-world constraints. Levels define which rules can be relaxed, under what conditions, and with what governance.

## Compliance Levels

### Level 1 Strict Canon Pure

**Definition:** 100% compliance with ALL canon rules. Zero exceptions.

**Requirements:**
- ✅ All tokens from canonical set
- ✅ No hardcoded values
- ✅ SSR-safe
- ✅ Fully accessible (WCAG 2.1 AA)
- ✅ Density-aware
- ✅ Theme-compatible
- ✅ Type-safe props
- ✅ Documented with examples

**Use Cases:**
- Core design system components
- Public-facing components
- Components in component library showcase
- Reusable primitives

**Examples:**
```
ui/button/
ui/input/
ui/select/
primitives/focus-trap/
```

**Marker:**
**Category:** governance
**Tags:** components, compliance, rules, architecture
**Language:** EN

---

**Intro:**
Without defined compliance levels, components cannot balance strict rules and real constraints. Classification is required.

**Problem:**
components lack compliance classification causing inconsistent rule enforcement

**Solution:**
define compliance levels to control rule strictness and allowed exceptions

**Signals:**
- inconsistent rules
- partial compliance
- architecture drift

**Search Intent:**
how to classify component compliance

**Keywords:**
component compliance levels design system, frontend rule enforcement levels, ui component governance classification, design system compliance model

---

```rust
/// @canon-level strict
/// Compliance: 100%
#[component]
pub fn Button(...) -> impl IntoView { ... }
```


































































