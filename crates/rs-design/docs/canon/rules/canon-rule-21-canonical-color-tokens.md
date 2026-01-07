# Canon Rule #21: Canonical Color Tokens vs Semantic Intents

**Status:** ✅ Enforced  
**Version:** 1.0.0  
**Date:** 2025-01-01

## Principle
Components consume a **stable contract of theme tokens**. Semantic intents (success, warning, info) are **application-layer mappings**, not theme-layer requirements.

## Canonical Tokens (MANDATORY in ALL themes)

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
2. Makes themes less portable/vendable
3. Creates design drift between components

## Component Variant Rules

### ✅ ALLOWED (canonical)
```rust
pub enum ButtonVariant {
    Solid,       // uses primary
    Outline,     // uses background + border
    Ghost,       // uses transparent + muted-foreground
    Destructive, // uses destructive
    Secondary,   // uses secondary
    Muted,       // uses muted
    Accent,      // uses accent
}
```

### ❌ FORBIDDEN
```rust
pub enum ButtonVariant {
    Success,  // NO canonical token
    Warning,  // NO canonical token
    Info,     // NO canonical token
    Danger,   // use Destructive
}
```

## Semantic Intent Layer (Optional)

If your application needs semantic intents, create a **mapping layer**:
```rust
pub enum AlertIntent {
    Success,  // -> primary
    Warning,  // -> accent
    Info,     // -> secondary
    Error,    // -> destructive
}

impl AlertIntent {
    fn to_variant(&self) -> AlertVariant {
        match self {
            Self::Success => AlertVariant::Primary,
            Self::Warning => AlertVariant::Accent,
            Self::Info => AlertVariant::Secondary,
            Self::Error => AlertVariant::Destructive,
        }
    }
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
3. Update component usage

### Phase 3: Semantic Layer (if needed)
1. Create intent enums at app level
2. Map intents to canonical variants
3. Keep design system agnostic

## Compliance Checklist
- [ ] All component variants map to canonical tokens
- [ ] No `success`/`warning`/`info` in variant enums
- [ ] Themes only define canonical tokens
- [ ] Semantic intents in app layer only

## References
- shadcn/ui theme specification
- Radix Colors documentation
- Design Tokens W3C Community Group
