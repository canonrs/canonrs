# Canon Rule #162: Providers Are Infrastructure, Not UI

**Status:** ENFORCED  
**Severity:** HIGH  
**Version:** 1.0.0  
**Date:** 2026-01-27

**Category:** component-architecture
**Tags:** providers, architecture, state, ui
**Language:** EN

---

**Intro:**
Treating providers as UI components mixes state management with presentation and breaks architectural boundaries. This leads to coupling and poor composition.

**Problem:**
providers are used as ui components causing architectural leakage

**Solution:**
keep providers as infrastructure and let ui consume their context

**Signals:**
- provider as ui
- coupling
- state leakage

**Search Intent:**
how to separate providers from ui components

**Keywords:**
provider vs ui separation, context architecture frontend, state provider pattern, ui composition rules

---

## Principle

**Providers must never be implemented or exposed as UI components.**

- Objective
- Verifiable
- One clear boundary

---

## Problem

Treating providers as UI causes architectural leakage.

- State logic mixed with presentation
- Broken composition
- Tight coupling to layouts
- Unclear responsibility boundaries

---

## Forbidden Pattern

### ❌ Forbidden

```rust
<ThemeProvider>
    <Button />
</ThemeProvider>
```

Used as a visual component.

---

## Canonical Pattern

### ✅ Canonical

```rust
<ThemeProvider>
    {children()}
</ThemeProvider>
```

UI consumes context, never replaces provider.

---

## Rationale

Providers define system state, not visuals.

- Enforces separation of concerns
- Preserves composability
- Prevents UI-state coupling
- Canonical infrastructure boundary

---

## Enforcement

- Code review
- Static analysis (provider usage)
- Architecture audits

---

## Exceptions

> **No exceptions. This rule is absolute.**

---

## Version History

- **1.0.0** — Initial version