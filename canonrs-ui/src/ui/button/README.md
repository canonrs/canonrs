# Button Component

**Type:** Pure/Stateful (Type 1/2)  
**Status:** Production-Ready  
**Tokens:** 100% Canonical (A+B+C)

---

## Purpose

Primary interactive element for user actions. Fully token-driven, SSR-safe, accessible.

---

## Usage
```rust
use rs_design::ui::button::*;

// Basic
<Button>"Click me"</Button>

// With variant
<Button variant=ButtonVariant::Outline>"Cancel"</Button>

// With size
<Button size=ButtonSize::Lg>"Large Button"</Button>

// Danger action
<Button variant=ButtonVariant::Danger>"Delete"</Button>

// Custom type
<Button button_type=ButtonType::Button>"Not Submit"</Button>

// With validation state
<Button validation=ValidationState::Error>"Fix Errors"</Button>
```

---

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `variant` | `Option<ButtonVariant>` | `Solid` | Visual style |
| `size` | `Option<ButtonSize>` | `Md` | Button size |
| `disabled` | `bool` | `false` | Disabled state |
| `validation` | `Option<ValidationState>` | `None` | Validation feedback |
| `button_type` | `ButtonType` | `Submit` | HTML type attribute |
| `class` | `String` | `""` | Additional classes |
| `children` | `Children` | - | Button content |

---

## Variants

### ButtonVariant
- `Solid` (default) - Primary action
- `Outline` - Secondary action
- `Ghost` - Tertiary/subtle action
- `Danger` - Destructive action
- `Warning` - Warning action
- `Success` - Success confirmation
- `Muted` - Low emphasis
- `Elevated` - Raised surface
- `Inverted` - Inverted colors

### ButtonSize
- `Xs` - Extra small (compact UIs)
- `Sm` - Small (dense layouts)
- `Md` (default) - Medium (standard)
- `Lg` - Large (prominent actions)
- `Xl` - Extra large (hero CTAs)

### ButtonType
- `Button` - Generic button
- `Submit` (default) - Form submission
- `Reset` - Form reset

### ValidationState
- `Error` - Error feedback
- `Success` - Success feedback
- `Warning` - Warning feedback

---

## Tokens Used

### Core (A)
- `radius.md` - Border radius
- `space.{xs,sm,md,lg,xl}` - Horizontal padding
- `border.width.thin` - Border width
- `shadow.sm` - Subtle elevation
- `z.base` - Stacking context

### Typography (B)
- `font.family.sans` - Font family
- `font.size.{xs,sm,md,lg,xl}` - Text size per size variant
- `font.weight.medium` - Default weight
- `line.height.normal` - Line spacing

### Color (B)
- `color.bg.{surface,muted,elevated}` - Background colors
- `color.fg.{default,muted,inverted}` - Text colors
- `color.border.{default,muted}` - Border colors
- `color.primary.{bg,fg,border}` - Primary variant
- `color.danger.{bg,fg,border}` - Danger variant
- `color.warning.{bg,fg,border}` - Warning variant
- `color.success.{bg,fg,border}` - Success variant

### State (B)
- `state.focus.ring` - Focus indicator
- `state.disabled.opacity` - Disabled appearance
- `state.hover.opacity` - Hover feedback
- `state.active.opacity` - Active/pressed state

### Motion (B)
- `motion.duration.fast` - Transition speed
- `motion.ease.standard` - Easing function

### Forms (C)
- `field.height` - Button height
- `field.padding` - Vertical padding
- `validation.{error,success,warning}` - Validation colors

**Total: 100% Canonical Coverage**

---

## Accessibility

- ✅ Keyboard navigable (Tab, Enter, Space)
- ✅ Focus visible (`state.focus.ring`)
- ✅ Disabled state (`disabled` attribute + visual)
- ✅ Semantic HTML (`<button>`)
- ✅ Type attribute (`button`, `submit`, `reset`)

---

## Canon Rules Applied

- **Rule #1 (Types):** Type 1 (pure rendering, no effects)
- **Rule #2 (Ownership):** Uses `Children` correctly
- **Rule #5 (SSR):** No browser APIs, fully SSR-safe
- **Rule #6 (Visual State):** 100% token-driven, zero hardcoded values

---

## States

| State | Behavior |
|-------|----------|
| Default | Normal interactive state |
| Hover | `state.hover.opacity` applied |
| Active | `state.active.opacity` applied |
| Focus | `state.focus.ring` visible |
| Disabled | `state.disabled.opacity` + no pointer events |

---

## File Structure
```
button/
├── button.rs           # Main component
├── variants.rs         # ButtonVariant, ButtonSize, ValidationState
├── types.rs            # ButtonType enum
├── mod.rs              # Module exports
└── README.md           # This file
```

---

## Examples

### Form Submit Button
```rust
<Button button_type=ButtonType::Submit>
    "Submit Form"
</Button>
```

### Destructive Action
```rust
<Button 
    variant=ButtonVariant::Danger
    on:click=move |_| delete_item()
>
    "Delete Account"
</Button>
```

### Large CTA
```rust
<Button 
    size=ButtonSize::Xl
    variant=ButtonVariant::Solid
>
    "Get Started"
</Button>
```

### With Validation
```rust
<Button 
    validation=if has_errors { 
        Some(ValidationState::Error) 
    } else { 
        None 
    }
>
    "Save Changes"
</Button>
```

---

## Migration from Old Button
```rust
// Old (primitive)
<Button variant=ButtonVariant::Outline />

// New (UI)
<Button variant=ButtonVariant::Outline />

// Old (string type)
<Button button_type="button" />

// New (enum)
<Button button_type=ButtonType::Button />
```

---

**Status:** ✅ Production-Ready  
**Canonicity:** 100%  
**Last Updated:** 2025-12-30
