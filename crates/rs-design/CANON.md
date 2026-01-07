# RS-Design Canon Rules

## Rule #21: Canonical Color Tokens vs Semantic Intents

### Principle
Components consume a **stable contract of theme tokens**. Semantic intents (success, warning, info) are **application-layer mappings**, not theme-layer requirements.

### Canonical Tokens (MANDATORY in ALL themes)

**Brand / Action**
- `primary` + `primary-foreground`

**Secondary / Neutral**
- `secondary` + `secondary-foreground`

**Accent (emphasis without destruction)**
- `accent` + `accent-foreground`

**Danger / Error**
- `destructive` + `destructive-foreground`

**Base UI**
- `background` + `foreground`
- `muted` + `muted-foreground`
- `card` + `card-foreground`
- `popover` + `popover-foreground`

**Structure**
- `border`
- `input`
- `ring`

**Charts (optional but recommended)**
- `chart-1` through `chart-5`

**Sidebar (optional, for sidebar-enabled apps)**
- `sidebar-background` + `sidebar-foreground`
- `sidebar-primary` + `sidebar-primary-foreground`
- `sidebar-accent` + `sidebar-accent-foreground`
- `sidebar-border` + `sidebar-ring`

### Prohibited Tokens
❌ `success` / `success-foreground`
❌ `warning` / `warning-foreground`
❌ `info` / `info-foreground`
❌ `danger` (use `destructive`)

**Rationale:** These introduce semantic opinions that:
1. Break shadcn/ui compatibility
2. Make themes less portable/vendable
3. Create design drift between components

### Component Variant Rules

**✅ ALLOWED (canonical)**
```rust
pub enum ButtonVariant {
    Solid,      // uses primary
    Outline,    // uses background + border
    Ghost,      // uses transparent + muted-foreground
    Destructive,// uses destructive
    Secondary,  // uses secondary
    Muted,      // uses muted
}
```

**❌ FORBIDDEN**
```rust
pub enum ButtonVariant {
    Success,  // NO canonical token
    Warning,  // NO canonical token
    Info,     // NO canonical token
}
```

### Semantic Intent Layer (Optional)

If your application needs semantic intents, create a **mapping layer** above components:
```rust
pub enum AlertIntent {
    Success,  // maps to -> primary
    Warning,  // maps to -> accent
    Info,     // maps to -> secondary
    Error,    // maps to -> destructive
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

### Migration Strategy

**Phase 1: Audit**
```bash
# Find all non-canonical variants
grep -r "Success\|Warning\|Danger" src/ui/*/variants.rs
```

**Phase 2: Refactor**
- Remove `Success`, `Warning`, `Danger` variants
- Replace with canonical equivalents
- Update component usage

**Phase 3: Semantic Layer (if needed)**
- Create intent enums at app level
- Map intents to canonical variants
- Keep components agnostic

### Compliance Checklist
- [ ] All component variants map to canonical tokens
- [ ] No `success`/`warning`/`info` in variant enums
- [ ] Themes only define canonical tokens
- [ ] Semantic intents live in app layer, not design system

---

**Status:** ✅ Enforced as of 2025-01-01
**Version:** 1.0.0
