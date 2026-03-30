# Canon Rule #21: Canonical Color Tokens vs Semantic Intents

**Status:** ENFORCED

**Severity:** HIGH
**Scope:** design-system, tokens
**Version:** 1.0.0
**Date:** 2025-01-16

---

## Principle
Components consume a **stable contract of theme tokens**. Semantic intents (success, warning, info) are **application-layer mappings**, not theme-layer requirements.

## Canonical Tokens

### Brand / Action
- `primary` + `primary-foreground`

### Secondary / Neutral
- `secondary` + `secondary-foreground`

### Accent (emphasis without destruction)
- `accent` + `accent-foreground`

### Danger / Error
- `destructive` + `destructive-foreground`

### Base UI
- `background` + `foreground`
- `muted` + `muted-foreground`
- `card` + `card-foreground`
- `popover` + `popover-foreground`

### Structure
- `border`
- `input`
- `ring`

### Charts (optional but recommended)
- `chart-1` through `chart-5`

### Sidebar (optional)
- `sidebar-background` + `sidebar-foreground`
- `sidebar-primary` + `sidebar-primary-foreground`
- `sidebar-accent` + `sidebar-accent-foreground`
- `sidebar-border` + `sidebar-ring`

## Prohibited Tokens
❌ `success` / `success-foreground`  
❌ `warning` / `warning-foreground`  
❌ `info` / `info-foreground`  
❌ `danger` (use `destructive`)

**Rationale:**
1. Breaks shadcn/ui compatibility  
2. Makes themes less portable  
3. Creates design drift  

## Component Variant Rules

### Allowed
```rust
pub enum ButtonVariant {
    Solid,
    Outline,
    Ghost,
    Destructive,
    Secondary,
    Muted,
    Accent,
}
```

### Forbidden
```rust
pub enum ButtonVariant {
    Success,
    Warning,
    Info,
    Danger,
}
```

## Semantic Intent Layer
```rust
pub enum AlertIntent {
    Success,
    Warning,
    Info,
    Error,
}
```

## Migration Strategy

### Phase 1: Audit
```bash
grep -r "Success\|Warning\|Danger" src/ui/*/variants.rs
```

### Phase 2: Refactor
1. Remove non-canonical variants  
2. Replace with canonical equivalents  

### Phase 3: Semantic Layer
1. Create mapping layer  

---

## Compliance Checklist
- [ ] Only canonical tokens used  
- [ ] No success/warning/info in variants  

---

## References
- shadcn/ui  
- Radix Colors  
- Design Tokens W3C  
