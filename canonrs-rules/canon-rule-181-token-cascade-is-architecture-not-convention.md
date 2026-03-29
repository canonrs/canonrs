# Canon Rule #181: Token Cascade Is Architecture, Not Convention

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Scope:** architecture
**Version:** 1.0.0  
**Date:** 2026-01-30

---

## Principle

**The token cascade (Preset → Theme → Semantic → Family → Component) is an architectural constraint enforced by the framework—not a coding convention.**

- Every layer has one job
- No layer can be skipped
- Order is enforced by build system
- Violations fail at build time, not runtime

---

## Problem

When token cascade is treated as convention:

- **Developers skip layers** - components reference presets directly
- **Inconsistent usage** - some files follow cascade, others don't
- **Refactoring breaks** - no guarantee layers are respected
- **Onboarding confusion** - "should I use `--color-*` or `--semantic-*`?"

Real result: Treating cascade as architecture (not convention) made theme switching work **perfectly on first try** after fixing token layer.

---

## Forbidden Pattern

### ❌ Forbidden (treating cascade as suggestion)

```css
/* Component violates cascade - no enforcement */
[data-button] {
  background: hsl(var(--color-primary)); /* ❌ Skips semantic + family */
}
```

```rust
// Family violates cascade - no enforcement
FamilyToken::new(
  "button-primary-bg",
  "hsl(38 92% 50%)"  // ❌ Skips semantic, hardcodes color
)
```

```bash
# Build succeeds despite violations
npm run build  # ✅ (but wrong)
```

**Why forbidden:** No enforcement = violations slip through. Architecture becomes suggestion. Eventually breaks.

---

## Canonical Pattern

### ✅ Canonical (architecture enforced)

```rust
// Family engine validates tokens
fn validate_family_token(token: &FamilyToken) -> Result<(), Error> {
  if !token.value.starts_with("var(--semantic-") {
    return Err(Error::InvalidTokenReference(
      format!("Family token '{}' must reference --semantic-*, got '{}'",
        token.name, token.value)
    ));
  }
  Ok(())
}
```

```bash
# Build FAILS if cascade violated
npm run build
# ❌ ERROR: Family token 'button-primary-bg' references '--color-primary' directly.
#    Family tokens must reference semantic layer (--semantic-*).
```

```yaml
# Linter enforces cascade
stylelint:
  rules:
    canonrs/token-cascade:
      - error
      - layers:
          - preset: "--color-*"
          - semantic: "--semantic-*"
          - family: "--{component}-*"
        enforce-references: true
```

**Why correct:** Violations are impossible. Build fails before wrong code ships. Architecture is guaranteed.

---

## Rationale

### Architecture vs Convention

```
Convention: "Please follow this pattern"
            → Humans forget
            → Violations accumulate
            → Eventually breaks

Architecture: "This pattern is enforced"
              → Build fails on violation
              → Impossible to ship wrong code
              → Always correct
```

### The Five Layers (enforced)

```
1. PRESET     → Define data (colors, sizes)
               ✅ Can reference: nothing
               ❌ Cannot reference: anything

2. THEME      → Define context (light/dark)
               ✅ Can reference: --color-*
               ❌ Cannot reference: --semantic-*, --family-*

3. SEMANTIC   → Define meaning (action-primary, surface-bg)
               ✅ Can reference: --color-*
               ❌ Cannot reference: --family-*, components

4. FAMILY     → Define contracts (button-*, card-*, table-*)
               ✅ Can reference: --semantic-*
               ❌ Cannot reference: --color-*

5. COMPONENT  → Define usage (button, card, table CSS)
               ✅ Can reference: --{family}-*
               ❌ Cannot reference: --color-*, --semantic-*
```

**Each layer has allowed references. Violations fail build.**

### Architectural Invariants

1. **Dependency direction is one-way** - higher layers depend on lower, never reverse
2. **No layer skipping** - each layer consumes only the layer directly below
3. **Build-time validation** - impossible to ship violations
4. **Self-documenting** - error messages teach correct usage

### Bugs Prevented

- Component referencing preset (skips 3 layers)
- Theme referencing family (wrong direction)
- Family hardcoding values (skips semantic)
- Any cascade violation shipping to production

### Why Not Opinion

This is **layered architecture** enforced by the compiler. Not stylistic preference—structural requirement.

---

## Enforcement

### Build-time validation (Rust)

```rust
// family_engine validates all tokens
pub fn build() -> Result<(), Error> {
  for family in FAMILIES {
    for token in family.tokens {
      validate_token_cascade(&token)?;
    }
  }
  Ok(())
}

fn validate_token_cascade(token: &FamilyToken) -> Result<(), Error> {
  // Family tokens MUST reference --semantic-*
  if !token.value.contains("--semantic-") {
    return Err(Error::CascadeViolation {
      token: token.name.clone(),
      layer: "family",
      expected: "--semantic-*",
      got: token.value.clone(),
    });
  }
  Ok(())
}
```

### Linter (CSS)

```javascript
// stylelint plugin
module.exports = {
  rules: {
    "canonrs/enforce-token-cascade": (enabled, options) => {
      return (root, result) => {
        root.walkDecls((decl) => {
          const layer = getFileLayer(decl.source.input.file);
          const refs = extractVarReferences(decl.value);

          refs.forEach((ref) => {
            if (!isValidReference(layer, ref)) {
              report({
                message: `Layer '${layer}' cannot reference '${ref}'`,
                node: decl,
              });
            }
          });
        });
      };
    },
  },
};
```

### Documentation generation

```bash
# Generate cascade diagram from enforced rules
npm run docs:cascade
# Outputs visual diagram showing allowed references
```

### Error messages (educational)

```
❌ ERROR in family-c-forms.css

  Family token 'button-primary-bg' references '--color-primary' directly.

  RULE: Canon Rule #169 - Token Cascade Is Architecture
  FIX:  Reference semantic layer instead:

        --button-primary-bg: var(--semantic-action-primary-bg);

  WHY:  Family tokens define contracts consumed by components.
        They must reference semantic tokens so theme context
        (light/dark) is correctly resolved.

  DOCS: /opt/docker/monorepo/libs/canonRS-rules/canon-rule-179
```

---

## Exceptions

**No exceptions. This rule is absolute.**

The cascade is the framework. Violating it is violating the architecture. If enforcement is too strict, relax the rule definition—don't bypass enforcement.

---

## Summary

**The token cascade is not a suggestion. It's a compiler.**

```
Preset   → defines data
Theme    → defines context
Semantic → defines meaning
Family   → defines contract
Component → consumes contract
```

**Every violation is a build failure. This is not optional.**

---

## Version History

- **1.0.0** — Initial version (2026-01-30)
