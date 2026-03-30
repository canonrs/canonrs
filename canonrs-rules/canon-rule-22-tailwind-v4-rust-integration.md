# Canon Rule #22: Tailwind v4 + Rust Integration

**Status:** ENFORCED

**Severity:** CRITICAL
**Scope:** build, css
**Version:** 1.0.0
**Date:** 2025-01-16

---

## Principle
Tailwind v4 JIT **cannot reliably parse Rust syntax**. All utilities must be **precompiled**.

---

## ❌ DOES NOT WORK
```rust
class="bg-[hsl(var(--color-primary))]"
class="h-[var(--size-control-md)]"
```

---

## ✅ WORKS
```rust
class="bg-primary"
class="h-control-md"
```

---

## Architecture

### Layer 1: Tokens
```css
--color-primary: 38 92% 50%;
```

### Layer 2: Utilities
```css
.bg-primary { background-color: hsl(var(--color-primary)); }
```

### Layer 3: Rust
```rust
"bg-primary text-primary-foreground"
```

---

## Critical Rules

### No Arbitrary Values
```rust
// ❌
bg-[...]

// ✅
bg-primary
```

### Predefined Utilities Only

### CSS Order Matters

---

## Build Pipeline

```bash
cargo leptos watch
npm run watch:css
```

---

## Pitfalls

- Cache issues  
- Missing utilities  
- Inline styles don't help  

---

## Validation
- [ ] Utilities defined  
- [ ] No arbitrary values  

---

## Lessons Learned
1. JIT ≠ Rust parsing  
2. Precompile > runtime  
3. PostCSS > CLI  
