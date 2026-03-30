# Canon Rule #205: UI Tokens Must Bind to Semantic Tokens, Never to Presets

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Scope:** components, tokens
**Version:** 1.0.0  
**Date:** 2026-01-30

---

## Principle

**Family tokens (UI contracts like `--button-primary-bg`) must reference semantic tokens (`--semantic-*`), never preset tokens (`--color-*`).**

- Families consume semantic layer
- Components consume family layer
- No layer skipping

---

## Problem

When family tokens reference presets directly:

- **Context is bypassed** - theme light/dark has no effect
- **Semantic meaning is lost** - "primary" becomes "color X" instead of "action intent"
- **Refactoring breaks** - changing semantic layer doesn't propagate
- **Testing impossible** - can't verify semantic correctness without inspecting RGB

Real bug: `--button-primary-bg: hsl(var(--color-primary))` → button ignored theme context → broke in light mode.

---

## Forbidden Pattern

### Forbidden
```css
/* family-c-forms.css */
[data-theme="canonrs"] {
  /* ❌ Family directly referencing preset */
  --button-primary-bg: hsl(var(--color-primary));
  --button-primary-fg: hsl(var(--color-primary-foreground));
}
```
```rust
// family_c_forms.rs
FamilyToken::new(
  "button-primary-bg",
  "var(--color-primary)"  // ❌ Skips semantic layer
)
```

**Why forbidden:** Family token is now coupled to preset. Theme can't inject context. Semantic contract is violated.

---

## Canonical Pattern

### Canonical
```css
/* themes/dark/ui.css - Theme defines semantic meaning */
@layer theme {
  [data-theme="canonrs"].dark {
    --semantic-action-primary-bg: hsl(var(--color-primary));
  }
}
```
```rust
// family_c_forms.rs - Family binds to semantic
FamilyToken::new(
  "button-primary-bg",
  "var(--semantic-action-primary-bg)"  // ✅ Consumes semantic contract
)
```
```css
/* button_ui.css - Component consumes family */
[data-button][data-ui-variant="solid"] {
  background: var(--button-primary-bg);  // ✅ Stable contract
}
```

**Why correct:** Each layer respects the contract. Family is stable. Theme injects context. Component is agnostic.

---

## Rationale

### Token Flow
```
Preset:   --color-primary (data)
          ↓
Theme:    --semantic-action-primary-bg (meaning)
          ↓
Family:   --button-primary-bg (contract)
          ↓
Component: background: var(--button-primary-bg) (usage)
```

**Families are contracts, not implementations.**

### Architectural Invariants

1. **Families define interfaces** - what tokens components need
2. **Semantic defines implementation** - what those tokens mean
3. **Theme injects context** - how meaning changes in light/dark
4. **Preset defines data** - what colors exist

### Bugs Prevented

- Button using wrong color in light mode (bypassed theme)
- Semantic refactor having no effect (family hardwired to preset)
- Component coupled to palette (can't change palette independently)

### Why Not Opinion

This is **Dependency Inversion Principle**. Families depend on abstraction (semantic), not concretion (preset).

---

## Enforcement

### Build-time validation
```bash
# Family engine must only generate semantic references
grep "var(--color-" styles/.generated/family-*.css && exit 1
```

### Linter rule (Rust)
```rust
// In family_engine
fn validate_token(token: &FamilyToken) {
  if token.value.contains("--color-") {
    panic!("Family token {} references preset directly. Use --semantic-* instead.", token.name);
  }
}
```

### Review checklist

- [ ] All family tokens reference `--semantic-*`
- [ ] No `--color-*` in family definitions
- [ ] Family engine validates semantic-only references

---

## Exceptions

**No exceptions. This rule is absolute.**

If a family needs a value that isn't semantic, the semantic layer is incomplete. Add the missing semantic token.

---

## Version History

- **1.0.0** — Initial version (2026-01-30)
