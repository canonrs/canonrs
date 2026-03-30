# Canon Rule #180: Semantic Token Names Must Follow Canonical Suffixes

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Scope:** design-system, tokens
**Version:** 1.0.0  
**Date:** 2026-01-30

---

## Principle

**Semantic tokens must use standardized suffixes exclusively: `-bg`, `-fg`, `-border`—never custom variants like `-surface`, `-color`, or `-text`.**

- Only three suffixes are allowed
- No exceptions, no variations
- Enforced by family-engine validation

---

## Problem

When semantic tokens use non-standard suffixes:

- **Family references token that doesn't exist** - `--semantic-info-surface` not found
- **Theme defines correct name** - `--semantic-info-bg` exists
- **CSS breaks silently** - component has no background, no error shown
- **Maintenance nightmare** - every team invents different suffixes

Real bug: Family referenced `--semantic-info-surface` → theme defined `--semantic-info-bg` → callout had transparent background.

---

## Forbidden Pattern

### Forbidden
```rust
// family_e_feedback.rs
FamilyToken::new("callout-bg-info", "var(--semantic-info-surface)"), // ❌ -surface doesn't exist
FamilyToken::new("alert-fg-error", "var(--semantic-error-color)"),   // ❌ -color not allowed
FamilyToken::new("badge-text", "var(--semantic-text-primary-fg)"),   // ❌ inconsistent naming
```
```css
/* themes/dark/ui.css */
--semantic-info-surface: hsl(var(--color-muted));  /* ❌ Wrong suffix */
--semantic-error-color: hsl(var(--color-destructive));  /* ❌ Wrong suffix */
```

**Why forbidden:** Family and theme use different naming → token lookup fails → visual breaks.

---

## Canonical Pattern

### Canonical
```rust
// family_e_feedback.rs
FamilyToken::new("callout-bg-info", "var(--semantic-info-bg)"),    // ✅ Standard -bg
FamilyToken::new("alert-fg-error", "var(--semantic-error-fg)"),    // ✅ Standard -fg
FamilyToken::new("badge-border", "var(--semantic-success-border)"), // ✅ Standard -border
```
```css
/* themes/dark/ui.css */
--semantic-info-bg: hsl(var(--color-muted));       /* ✅ Correct suffix */
--semantic-error-fg: hsl(var(--color-destructive-foreground)); /* ✅ Correct suffix */
--semantic-success-border: hsl(var(--color-accent)); /* ✅ Correct suffix */
```

**Why correct:** Family and theme use identical naming convention. Token always resolves.

---

## Rationale

### The Three Canonical Suffixes
```
-bg     → background-color
-fg     → color (foreground/text)
-border → border-color
```

**Why only three?**
- Cover 99% of use cases
- Prevent naming drift
- Enable search/replace refactoring
- Enforceable by regex

### Architectural Invariants

1. **Token names are contracts** - family and theme must agree
2. **Suffixes are standardized** - no custom inventions
3. **Validation is automatic** - build fails on mismatch

### Bugs Prevented

- Token mismatch (family references non-existent token)
- Silent CSS failures (transparent backgrounds, invisible text)
- Team confusion (everyone invents different suffixes)
- Refactoring breaks (can't regex-replace `-surface` → `-bg`)

### Why Not Opinion

This is **naming consistency**. Like variable naming in code—arbitrary choice, but once chosen, must be universal.

---

## Enforcement

### Build-time validation (Rust)
```rust
// family_engine
fn validate_semantic_reference(token: &FamilyToken) -> Result<(), Error> {
  let value = &token.value;
  if value.contains("--semantic-") {
    if !value.ends_with("-bg)") && !value.ends_with("-fg)") && !value.ends_with("-border)") {
      return Err(Error::InvalidSemanticSuffix {
        token: token.name.clone(),
        reference: value.clone(),
        expected: "Semantic tokens must end with -bg, -fg, or -border"
      });
    }
  }
  Ok(())
}
```

### Linter rule (CSS)
```yaml
# stylelint
custom-property-pattern:
  - "^semantic-(.*-)?(bg|fg|border)$"
  - message: "Semantic tokens must use -bg, -fg, or -border suffix"
```

### Error message (educational)
```
❌ ERROR in family-e-feedback.rs

  Token 'callout-bg-info' references 'var(--semantic-info-surface)'

  RULE: Canon Rule #180 - Semantic Token Names Must Follow Canonical Suffixes
  
  ALLOWED SUFFIXES:
    --semantic-*-bg      (background)
    --semantic-*-fg      (foreground/text)
    --semantic-*-border  (border)

  FIX: Change reference to:
       var(--semantic-info-bg)

  DOCS: /opt/docker/monorepo/libs/canonRS-rules/canon-rule-180
```

---

## Exceptions

**No exceptions. This rule is absolute.**

If you need a semantic token that doesn't fit `-bg`, `-fg`, or `-border`, you're likely:
1. Creating a component-specific token (belongs in family, not semantic)
2. Misunderstanding semantic purpose (semantic = meaning, not property)

---

## Version History

- **1.0.0** — Initial version (2026-01-30)
