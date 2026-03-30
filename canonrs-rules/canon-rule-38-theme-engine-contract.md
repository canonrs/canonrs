# Canon Rule #38: Theme Engine Contract

**Status:** ENFORCED

**Severity:** CRITICAL
**Version:** 1.0.0
**Date:** 2025-01-16


## Principle
The **Theme Engine** is a single component that reads all provider contexts (Theme, Density, Language) and computes the final CSS custom properties injected into `<style>`. It is the **only** component that performs this computation. No other component may write CSS variables to the DOM.

## Responsibility

**Input Sources:**
1. `ThemeContext` → `{ mode, preset }`
2. `DensityContext` → `{ mode }`
3. `LanguageContext` → `{ current }`
4. `ThemeRegistry` → Preset definitions

**Output:**
- Injects `<style>` tag with CSS variables
- Updates on any context change
- Applies to `:root` selector

**Does NOT:**
**Category:** design-system
**Tags:** theme, tokens, engine, css
**Language:** EN

---

**Intro:**
Multiple components writing css variables causes inconsistency and conflicts. A single engine must control output.

**Problem:**
multiple components write css variables causing inconsistency

**Solution:**
centralize css variable computation in a single theme engine

**Signals:**
- css conflict
- variable override
- theme inconsistency

**Search Intent:**
how to design theme engine css

**Keywords:**
theme engine css variables, design system theme computation, centralized theming architecture, frontend css variable management

---

- ❌ Persist state
- ❌ Read cookies
- ❌ Call server functions
- ❌ Modify providers































































