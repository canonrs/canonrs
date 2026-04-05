# Canon Rule #336: Token References Must Resolve to Existing Design Tokens

**Status:** ENFORCED
**Severity:** HIGH
**Version:** 1.0.0
**Date:** 2026-04-05

**Category:** tokens
**Tags:** tokens, css, design-system, variables, contracts
**Language:** EN

---

**Intro:**
Every CSS custom property used in a component token must reference a design token that exists in the token system. Unresolved references produce silent visual failures — the browser uses the inherited or initial value with no error.

**Problem:**
`--tabs-trigger-bg-active: var(--theme-primary-bg)` — `--theme-primary-bg` does not exist in the token system. The tab active state renders incorrectly with no build error.

**Solution:**
Before declaring a token reference, verify the target token exists. Use the same token chain used by equivalent components (e.g. Button primary uses `--theme-action-primary-bg`).

**Signals:**
- active state renders with wrong color
- component visually inconsistent with equivalent components
- no build error despite broken visual

**Search Intent:**
css variable not resolving, token reference broken, design token undefined

**Keywords:**
css custom property undefined, token chain broken, theme-primary-bg missing

---

## Principle

Token references are contracts. A broken reference is a silent contract violation. The token system must be the single source of truth — every reference must be verifiable.

---

## Problem
```rust
// ❌ broken — --theme-primary-bg does not exist
FamilyToken::new("tabs-trigger-bg-active", "var(--theme-primary-bg)"),
FamilyToken::new("tabs-trigger-fg-active", "var(--theme-primary-fg)"),
```

---

## Patterns

### Forbidden Pattern
- referencing `--theme-primary-bg` or any token not defined in the bundle
- copy-pasting token names without verifying they exist

### Canonical Pattern
```rust
// ✅ verify equivalent component first (Button uses these)
FamilyToken::new("tabs-trigger-bg-active", "var(--theme-action-primary-bg)"),
FamilyToken::new("tabs-trigger-fg-active", "var(--color-primary-foreground)"),
```

---

## Contract

### Enforcement
- every token value that references another token must be verified in `canonrs.bundle.css`
- equivalent components must use the same token chain for the same semantic role
- builder.md token declarations must not include prefixes that have no corresponding tokens

### Exceptions
None.

---

## Version History
- 1.0.0 - Initial definition
