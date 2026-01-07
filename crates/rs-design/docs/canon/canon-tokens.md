# CanonRS Token System v1.0

**Status:** ✅ Official  
**Version:** 1.0.0  
**Date:** 2025-01-01  
**Governance:** Immutable without major version bump

---

## Principle

This document defines the **complete token contract** for RS-Design. All components, themes, and applications must comply with this specification.

**Token Categories:**
1. **Canonical Tokens** — Globally available to ALL components
2. **Family Tokens** — Additional tokens for specific component families

---

## CANONICAL TOKENS

Available to **every component** in the design system. Themes MUST provide all color tokens. Other canonicals are system constants.

### 🧱 Core
```
radius.sm              // 0.125rem (2px)
radius.md              // 0.375rem (6px)
radius.lg              // 0.5rem (8px)
radius.xl              // 1rem (16px)

space.xs               // 0.25rem (4px)
space.sm               // 0.5rem (8px)
space.md               // 1rem (16px)
space.lg               // 1.5rem (24px)
space.xl               // 2rem (32px)
space.2xl              // 3rem (48px)

border.width.hairline  // 0.5px
border.width.thin      // 1px
border.width.medium    // 2px
border.width.thick     // 4px

shadow.none            // No shadow
shadow.2xs             // Minimal lift
shadow.xs              // Subtle
shadow.sm              // Standard card
shadow.md              // Raised element
shadow.lg              // Dropdown/menu
shadow.xl              // Popover
shadow.2xl             // Modal/dialog

z.base                 // 0
z.dropdown             // 1000
z.sticky               // 1100
z.fixed                // 1200
z.modal_backdrop       // 1300
z.modal                // 1400
z.popover              // 1500
z.tooltip              // 1600
z.toast                // 1700
```

### 🔤 Typography
```
font.family.sans       // System sans-serif stack
font.family.mono       // Monospace stack
font.family.serif      // Serif stack

font.size.xs           // 0.75rem (12px)
font.size.sm           // 0.875rem (14px)
font.size.md           // 1rem (16px)
font.size.lg           // 1.125rem (18px)
font.size.xl           // 1.25rem (20px)
font.size.2xl          // 1.5rem (24px)
font.size.3xl          // 1.875rem (30px)

font.weight.regular    // 400
font.weight.medium     // 500
font.weight.semibold   // 600
font.weight.bold       // 700

line.height.tight      // 1.25
line.height.normal     // 1.5
line.height.relaxed    // 1.75
```

### 🎨 Color (Semantic — Canon Rule #21)

**REQUIRED in all themes:**
```
color.primary.bg              // Brand color
color.primary.fg              // Text on primary
color.primary.border          // Primary borders

color.secondary.bg            // Secondary action
color.secondary.fg            // Text on secondary
color.secondary.border        // Secondary borders

color.accent.bg               // Emphasis without action
color.accent.fg               // Text on accent
color.accent.border           // Accent borders

color.destructive.bg          // Error/danger state
color.destructive.fg          // Text on destructive
color.destructive.border      // Destructive borders

color.background              // Page background
color.foreground              // Default text

color.muted.bg                // Subtle backgrounds
color.muted.fg                // De-emphasized text

color.card.bg                 // Card backgrounds
color.card.fg                 // Card text

color.popover.bg              // Popover backgrounds
color.popover.fg              // Popover text

color.border                  // Default borders
color.input                   // Input field borders
color.ring                    // Focus ring color

color.chart.1                 // Chart color 1
color.chart.2                 // Chart color 2
color.chart.3                 // Chart color 3
color.chart.4                 // Chart color 4
color.chart.5                 // Chart color 5

color.sidebar.bg              // Sidebar background
color.sidebar.fg              // Sidebar text
color.sidebar.primary         // Sidebar brand
color.sidebar.primary_fg      // Sidebar brand text
color.sidebar.accent          // Sidebar emphasis
color.sidebar.accent_fg       // Sidebar emphasis text
color.sidebar.border          // Sidebar borders
color.sidebar.ring            // Sidebar focus
```

**❌ PROHIBITED (non-canonical):**
```
color.success.*    // Use primary or accent instead
color.warning.*    // Use accent instead
color.danger.*     // Use destructive
color.info.*       // Use secondary or accent
```

### 🎯 State (Canon Rule #23)
```
state.focus.ring           // Focus indicator color (maps to color.ring)
state.hover.opacity        // 0.9 (90% of original)
state.active.opacity       // 0.8 (80% of original)
state.disabled.opacity     // 0.5 (50% of original)
state.loading.opacity      // 0.7 (70% of original)
```

### 🎞 Motion (Canon Rule #27)
```
motion.duration.instant       // 0ms
motion.duration.fast          // 150ms
motion.duration.normal        // 300ms
motion.duration.comfortable   // 400ms
motion.duration.slow          // 500ms
motion.duration.slower        // 700ms
motion.duration.page          // 1000ms

motion.ease.linear            // linear
motion.ease.in                // ease-in
motion.ease.out               // ease-out
motion.ease.in_out            // ease-in-out
motion.ease.standard          // cubic-bezier(0.4, 0, 0.2, 1)
motion.ease.emphasized        // cubic-bezier(0.2, 0, 0, 1)
motion.ease.decelerate        // cubic-bezier(0, 0, 0.2, 1)
motion.ease.accelerate        // cubic-bezier(0.4, 0, 1, 1)
motion.ease.bounce            // cubic-bezier(0.68, -0.55, 0.265, 1.55)
motion.ease.spring            // cubic-bezier(0.175, 0.885, 0.32, 1.275)
```

### 📐 Size & Density (Canon Rule #24)
```
size.control.xs            // 1.5rem (24px)
size.control.sm            // 2rem (32px)
size.control.md            // 2.5rem (40px)
size.control.lg            // 3.125rem (50px)
size.control.xl            // 3.75rem (60px)

size.icon.xs               // 0.875rem (14px)
size.icon.sm               // 1rem (16px)
size.icon.md               // 1.25rem (20px)
size.icon.lg               // 1.5rem (24px)
size.icon.xl               // 2rem (32px)

density.multiplier         // 0.75 (compact) | 1.0 (comfortable) | 1.25 (spacious)
```

---

## FAMILY TOKENS

Additional tokens for specific component families. Components inherit canonicals + their family tokens.

### A) Overlay & Layering

**Components:** Dialog, Popover, DropdownMenu, HoverCard, Sheet, ContextMenu, Menubar, Select, Combobox, DatePicker
```
overlay.backdrop.opacity       // Modal backdrop opacity
overlay.dismiss.outside        // Dismiss on outside click
overlay.dismiss.escape         // Dismiss on Escape key
overlay.focus_trap             // Trap focus inside
overlay.portal                 // Portal to body (contract only)
```

### B) Selection & Lists

**Components:** Accordion, Checkbox, Command, DataTable, RadioGroup, Table, ToggleGroup
```
list.item.height               // List item height
list.item.padding              // List item padding
selection.mode.single          // Single selection mode
selection.mode.multi           // Multi selection mode
selection.indicator.check      // Checkbox indicator
selection.indicator.radio      // Radio indicator
empty.state.style              // Empty state appearance
```

### C) Forms & Validation

**Components:** Button, Field, Form, Input, InputOTP, NativeSelect, Slider, Switch, Textarea, Toggle, DatePicker
```
field.height                   // Input field height
field.padding.x                // Horizontal padding
field.padding.y                // Vertical padding
field.border                   // Field border style
field.placeholder              // Placeholder appearance
validation.error               // Error state (maps to color.destructive)
validation.success             // Success state (maps to color.primary)
validation.warning             // Warning state (maps to color.accent)
input.masking                  // Input masking (OTP, phone, etc)
```

### D) Navigation & Structure

**Components:** Breadcrumb, ButtonGroup, Kbd, NavigationMenu, Pagination, Sidebar, Tabs, Typography
```
nav.item.height                // Navigation item height
nav.indicator.thickness        // Active indicator thickness
nav.breadcrumb.separator       // Breadcrumb separator
sidebar.width                  // Sidebar width
pagination.size                // Pagination button size
```

### E) Feedback & Status

**Components:** Alert, Badge, Progress, Skeleton, Spinner, Toast, Sonner
```
toast.duration                 // Toast auto-dismiss duration
spinner.size                   // Spinner dimensions
skeleton.shimmer               // Skeleton shimmer animation
progress.height                // Progress bar height
alert.emphasis                 // Alert visual weight
```

### F) Data & Media

**Components:** Avatar, Calendar, Card, Carousel, Chart, DataTable
```
chart.palette                  // Chart color palette
chart.grid                     // Chart grid appearance
chart.tooltip                  // Chart tooltip style
carousel.gap                   // Carousel item spacing
carousel.snap                  // Carousel snap behavior
aspect.ratio.1_1               // Square (1:1)
aspect.ratio.16_9              // Widescreen (16:9)
aspect.ratio.4_3               // Standard (4:3)
```

---

## Component → Family Mapping

| Component | Families | Canonical + Family Tokens |
|-----------|----------|---------------------------|
| Button | C | ✅ All Canonical + Forms/Validation |
| Select | A + B + C | ✅ All Canonical + Overlay + Lists + Forms |
| Dialog | A | ✅ All Canonical + Overlay |
| Alert | E | ✅ All Canonical + Feedback |
| DataTable | B + F | ✅ All Canonical + Lists + Data |

**Rule:** Every component uses ALL canonical tokens + tokens from assigned families.

---

## Governance

### Adding Tokens

**Canonical Tokens:**
- Requires Canon Rule amendment
- Major version bump (2.0.0)
- Community RFC process

**Family Tokens:**
- Requires family review
- Minor version bump (1.1.0)
- Documentation PR

### Removing Tokens

**Never remove canonical tokens** — deprecate with migration path.

**Family tokens** can be deprecated in minor versions, removed in major.

---

## Validation

### CLI Command
```bash
./scripts/validate-canon.sh
```

### Checks
- ✅ All components use only their assigned families
- ✅ No hardcoded values (colors, spacing, shadows)
- ✅ No non-canonical color tokens (success, warning, info)
- ✅ All state changes use opacity vars
- ✅ All motion uses timing vars

---

## References

- [Canon Rule #21: Canonical Color Tokens](./rules/canon-rule-21-canonical-color-tokens.md)
- [Canon Rule #22: Tailwind v4 + Rust Integration](./rules/canon-rule-22-tailwind-v4-rust-integration.md)
- [Canon Rule #23: State Tokens](./rules/canon-rule-23-state-tokens.md)
- [Canon Rule #24: Density & Size Scaling](./rules/canon-rule-24-density-size-scaling.md)
- [Canon Rule #25: Theme Presets Contract](./rules/canon-rule-25-theme-presets-contract.md)
- [Canon Rule #26: Elevation & Shadow System](./rules/canon-rule-26-elevation-shadow-system.md)
- [Canon Rule #27: Motion & Timing Tokens](./rules/canon-rule-27-motion-timing-tokens.md)
- [Canon Rule #28: Responsive Grid Contract](./rules/canon-rule-28-responsive-grid-contract.md)

---

**Versioning:** Semantic (MAJOR.MINOR.PATCH)  
**Status:** Immutable without governance approval  
**Compliance:** Enforced via automated validation
