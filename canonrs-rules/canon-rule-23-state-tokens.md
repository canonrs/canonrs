# Canon Rule #23: State Tokens (Hover, Focus, Disabled, Pressed)

**Status:** ENFORCED

**Severity:** HIGH
**Scope:** tokens, state, css
**Version:** 1.0.0
**Date:** 2025-01-16

---

## Principle
Interactive states must use **consistent, theme-independent opacity values** rather than color variants. This ensures state feedback works across all themes without requiring theme authors to define state-specific colors.

## Canonical State Tokens

### Interaction States (CSS Variables)
```css
:root {
  /* Opacity-based states (theme-independent) */
  --state-hover-opacity: 0.9;
  --state-active-opacity: 0.8;
  --state-disabled-opacity: 0.5;
  
  /* Focus ring */
  --state-focus-ring: hsl(var(--color-ring));
  --border-width-focus: 2px;
}
```

### State Application Pattern

#### ✅ CORRECT: Opacity-Based
```rust
const BASE_CLASSES: &str = "\
    hover:opacity-[var(--state-hover-opacity)] \
    active:opacity-[var(--state-active-opacity)] \
    disabled:opacity-[var(--state-disabled-opacity)] \
    focus-visible:ring-[color:var(--state-focus-ring)]";
```

#### ❌ INCORRECT: Color Variants
```rust
// Don't create separate hover colors
const HOVER_CLASSES: &str = "\
    hover:bg-primary-hover \      // ❌ Requires theme definition
    hover:border-primary-hover";  // ❌ Not portable
```

## State Categories

### 1. Hover State
**Token:** `--state-hover-opacity: 0.9`

**Usage:**
```rust
"hover:opacity-[var(--state-hover-opacity)]"
```

**Rationale:** Reduces brightness by 10%, works on any color

### 2. Active/Pressed State
**Token:** `--state-active-opacity: 0.8`

**Usage:**
```rust
"active:opacity-[var(--state-active-opacity)]"
```

**Rationale:** Deeper press feedback than hover

### 3. Disabled State
**Token:** `--state-disabled-opacity: 0.5`

**Usage:**
```rust
"disabled:opacity-[var(--state-disabled-opacity)] \
 disabled:cursor-not-allowed \
 disabled:pointer-events-none"
```

**Rationale:** 50% opacity universally signals "not available"

### 4. Focus State
**Token:** `--state-focus-ring: hsl(var(--color-ring))`

**Usage:**
```rust
"focus-visible:outline-none \
 focus-visible:ring-2 \
 focus-visible:ring-[color:var(--state-focus-ring)] \
 focus-visible:ring-offset-2"
```

**Rationale:** Uses the theme ring color for consistency

### 5. Loading State
**Token:** `--state-loading-opacity: 0.7`

**Usage:**
```rust
"data-[loading=true]:opacity-[var(--state-loading-opacity)] \
 data-[loading=true]:cursor-wait"
```

## State Combinations

### Layered States
```rust
"hover:opacity-[var(--state-hover-opacity)] \
 focus-visible:ring-2 \
 disabled:opacity-[var(--state-disabled-opacity)]"
```

### Priority Order
1. `disabled`  
2. `active` / `pressed`  
3. `focus-visible`  
4. `hover`  

---

## Prohibited Patterns

### ❌ State Color Variants
```css
--color-primary-hover: ...;
```

### ❌ Hardcoded Opacity
```rust
"hover:opacity-90"
```

### ❌ Mixed State Changes
```rust
"hover:opacity-90 hover:bg-blue-600"
```

---

## Accessibility Requirements

### Focus Indicators
```rust
"focus-visible:outline-none \
 focus-visible:ring-2 \
 focus-visible:ring-[color:var(--state-focus-ring)] \
 focus-visible:ring-offset-2"
```

### Disabled State
```rust
"disabled:pointer-events-none \
 disabled:cursor-not-allowed \
 disabled:opacity-[var(--state-disabled-opacity)]"
```

---

## Theme Contract

### MUST Provide
- `--color-ring`

### SHOULD NOT Provide
- State color variants  

---

## Validation Checklist
- [ ] Uses opacity tokens  
- [ ] No hardcoded opacity  
- [ ] Accessible focus states  

---

## References
- WCAG 2.1  
- Material Design  
