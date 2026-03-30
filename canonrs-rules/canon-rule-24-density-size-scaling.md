# Canon Rule #24: Density & Size Scaling

**Status:** ENFORCED

**Severity:** MEDIUM
**Scope:** tokens, design-system
**Version:** 1.0.0
**Date:** 2025-01-16

---

## Principle
UI density and component sizes must be **data-attribute driven** and **mathematically consistent** across the design system. Size scales follow a **modular scale** (1.25x ratio).

---

## Density Levels

```rust
pub enum Density {
    Compact,
    Comfortable,
    Spacious,
}
```

```css
:root {
  --density-multiplier: 1.0;
}

[data-density="compact"] {
  --density-multiplier: 0.75;
}

[data-density="spacious"] {
  --density-multiplier: 1.25;
}
```

---

## Size Scale

```css
--size-control-md: 2.5rem;
--space-md: 1rem;
```

---

## Button Sizes

```rust
pub enum ButtonSize {
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
    Icon,
}
```

---

## Typography

```css
--font-size-md: 1rem;
--line-height-normal: 1.5;
```

---

## Rules

- Use CSS variables  
- Maintain modular scale  
- Respect density  

---

## Prohibited

```rust
"h-10 px-4"
```

---

## Validation
- [ ] Uses tokens  
- [ ] No fixed pixels  

---

## References
- Modular Scale  
- WCAG  
