# Block Tokens Reference

## Purpose

Block tokens define **component appearance and behavior**. They control:
- Internal padding and spacing
- Colors and backgrounds
- Borders and shadows
- Typography
- Interactive states
- Animations and motion

## What Blocks DO NOT Control

❌ Page-level max-width
❌ Section gaps
❌ Layout grid systems
❌ Header/footer positioning

> **Rule:** If it's about page structure, it's a Layout token, not a Block token.

---

## Token Families Used by Blocks

**Available:** Core, Color, State, Motion, Typography, A-F (domain families)
**Never used:** `layout.grid.*`, `layout.max-width`, `layout.section.*`

---

## 1. Block — Internal Structure

Every block has internal spacing and borders:
```rust
// Example: Header block
// Location: /opt/docker/monorepo/packages-rust/rs-design/src/tokens/header.rs

pub const HEADER_PADDING_X: &str = "header.padding.x";
pub const HEADER_PADDING_Y: &str = "header.padding.y";
pub const HEADER_GAP: &str = "header.gap";
pub const HEADER_RADIUS: &str = "header.radius";
pub const HEADER_BORDER_WIDTH: &str = "header.border.width";
pub const HEADER_BORDER_STYLE: &str = "header.border.style";
pub const HEADER_BORDER_COLOR: &str = "header.border.color";
```

**Usage:**
```css
.canon-header {
  padding: var(--header-padding-y) var(--header-padding-x);
  gap: var(--header-gap);
  border-bottom: var(--header-border-width) var(--header-border-style) var(--header-border-color);
}
```

---

## 2. Block — Semantic Colors

Blocks use semantic color tokens (not raw colors):
```rust
pub const HEADER_BG: &str = "header.bg";
pub const HEADER_FG: &str = "header.fg";
pub const HEADER_MUTED_FG: &str = "header.muted.fg";
pub const HEADER_SURFACE_LEVEL: &str = "header.surface.level";
```

**Mapping to global tokens:**
```css
.canon-header {
  background: var(--header-bg);
  color: var(--header-fg);
}

/* header.bg maps to global color.bg.surface */
:root {
  --header-bg: var(--color-bg-surface);
  --header-fg: var(--color-fg-default);
}
```

---

## 3. Block — Interactive States

All interactive blocks consume state tokens:
```rust
pub const STATE_HOVER_OPACITY: &str = "state.hover.opacity";
pub const STATE_ACTIVE_OPACITY: &str = "state.active.opacity";
pub const STATE_DISABLED_OPACITY: &str = "state.disabled.opacity";
pub const STATE_FOCUS_RING: &str = "state.focus.ring";
pub const STATE_FOCUS_RING_COLOR: &str = "state.focus.ring.color";
```

**Usage:**
```css
.canon-button:hover {
  opacity: var(--state-hover-opacity);
}

.canon-button:focus-visible {
  outline: var(--state-focus-ring) solid var(--state-focus-ring-color);
}
```

---

## 4. Block — Typography

Typography tokens for text-based blocks:
```rust
pub const BLOCK_FONT_FAMILY: &str = "block.font.family";
pub const BLOCK_FONT_SIZE: &str = "block.font.size";
pub const BLOCK_FONT_WEIGHT: &str = "block.font.weight";
pub const BLOCK_LINE_HEIGHT: &str = "block.line.height";
pub const BLOCK_LETTER_SPACING: &str = "block.letter.spacing";
```

---

## 5. Block — Motion & Animation

Animation tokens for transitions:
```rust
pub const MOTION_DURATION_FAST: &str = "motion.duration.fast";
pub const MOTION_DURATION_NORMAL: &str = "motion.duration.normal";
pub const MOTION_DURATION_SLOW: &str = "motion.duration.slow";
pub const MOTION_EASE_STANDARD: &str = "motion.ease.standard";
pub const MOTION_EASE_EMPHASIZED: &str = "motion.ease.emphasized";
```

**Usage:**
```css
.canon-sidebar {
  transition: width var(--motion-duration-normal) var(--motion-ease-standard);
}
```

---

## 6. Domain-Specific Block Tokens

### Navigation Blocks (Family D)
```rust
// NavItem, Menu, Breadcrumb
pub const NAV_ITEM_HEIGHT: &str = "nav.item.height";
pub const NAV_ITEM_PADDING: &str = "nav.item.padding";
pub const NAV_INDICATOR_THICKNESS: &str = "nav.indicator.thickness";
pub const NAV_INDICATOR_COLOR: &str = "nav.indicator.color";
```

### Form Blocks (Family C)
```rust
// Input, Select, Checkbox
pub const FIELD_HEIGHT: &str = "field.height";
pub const FIELD_PADDING: &str = "field.padding";
pub const FIELD_BORDER_WIDTH: &str = "field.border.width";
pub const VALIDATION_ERROR_COLOR: &str = "validation.error.color";
pub const VALIDATION_SUCCESS_COLOR: &str = "validation.success.color";
```

### Data Blocks (Family F)
```rust
// Table, List, DataGrid
pub const TABLE_ROW_HEIGHT: &str = "table.row.height";
pub const TABLE_CELL_PADDING: &str = "table.cell.padding";
pub const LIST_ITEM_HEIGHT: &str = "list.item.height";
pub const LIST_ITEM_GAP: &str = "list.item.gap";
```

### Feedback Blocks (Family E)
```rust
// Alert, Toast, Spinner
pub const ALERT_PADDING: &str = "alert.padding";
pub const ALERT_ICON_SIZE: &str = "alert.icon.size";
pub const TOAST_DURATION: &str = "toast.duration";
pub const SPINNER_SIZE: &str = "spinner.size";
```

### Media Blocks (Family F)
```rust
// Image, Video, Chart
pub const ASPECT_RATIO_16_9: &str = "aspect.ratio.16_9";
pub const CAROUSEL_GAP: &str = "carousel.gap";
pub const CHART_PALETTE: &str = "chart.palette";
```

---

## Complete Example: Alert Block

**Token file:**
```rust
// /opt/docker/monorepo/packages-rust/rs-design/src/tokens/alert.rs

pub const ALERT_PADDING_X: &str = "alert.padding.x";
pub const ALERT_PADDING_Y: &str = "alert.padding.y";
pub const ALERT_GAP: &str = "alert.gap";
pub const ALERT_RADIUS: &str = "alert.radius";
pub const ALERT_BORDER_WIDTH: &str = "alert.border.width";
pub const ALERT_ICON_SIZE: &str = "alert.icon.size";

// Semantic colors (mapped per variant)
pub const ALERT_BG: &str = "alert.bg";
pub const ALERT_FG: &str = "alert.fg";
pub const ALERT_BORDER_COLOR: &str = "alert.border.color";
pub const ALERT_ICON_COLOR: &str = "alert.icon.color";
```

**CSS consumption:**
```css
/* /opt/docker/monorepo/packages-rust/rs-design/style/blocks/alert.css */

.canon-alert {
  padding: var(--alert-padding-y) var(--alert-padding-x);
  gap: var(--alert-gap);
  border-radius: var(--alert-radius);
  border: var(--alert-border-width) solid var(--alert-border-color);
  background: var(--alert-bg);
  color: var(--alert-fg);
}

/* Variant mapping */
[data-variant="error"] {
  --alert-bg: var(--color-bg-error);
  --alert-fg: var(--color-fg-error);
  --alert-border-color: var(--color-border-error);
}
```

---

## Absolute File Locations

**Token definitions:**
```
/opt/docker/monorepo/packages-rust/rs-design/src/tokens/header.rs
/opt/docker/monorepo/packages-rust/rs-design/src/tokens/footer.rs
/opt/docker/monorepo/packages-rust/rs-design/src/tokens/alert.rs
/opt/docker/monorepo/packages-rust/rs-design/src/tokens/button.rs
```

**CSS consumption:**
```
/opt/docker/monorepo/packages-rust/rs-design/style/blocks/header.css
/opt/docker/monorepo/packages-rust/rs-design/style/blocks/footer.css
/opt/docker/monorepo/packages-rust/rs-design/style/blocks/alert.css
```

**Component usage:**
```
/opt/docker/monorepo/packages-rust/rs-design/src/blocks/header/header.rs
/opt/docker/monorepo/packages-rust/rs-design/src/blocks/footer/footer.rs
/opt/docker/monorepo/packages-rust/rs-design/src/blocks/alert/alert.rs
```

---

## Quick Reference Table

| Layer | Tokens Used | Never Uses |
|-------|-------------|------------|
| **Block** | Core + Color + State + Motion + Typography + Domain (A-F) | layout.grid.*, layout.max-width, layout.section.* |
| **Layout** | layout.*, spacing.* | color.*, state.*, motion.*, validation.* |
| **Primitive** | ❌ None | All tokens |

---

## Token Families by Block Type

| Block Type | Primary Family | Secondary Families | Example Blocks |
|------------|---------------|-------------------|----------------|
| Navigation | D | Core, Color, State | Header, NavItem, Menu |
| Forms | C | Core, Color, State, Validation | Input, Select, Checkbox |
| Data | F | Core, Color, State | Table, List, DataGrid |
| Feedback | E | Core, Color, Motion | Alert, Toast, Progress |
| Media | F | Core, Aspect | Image, Video, Chart |
| Overlay | A | Core, Color, Motion | Dialog, Sheet, Popover |
| Lists | B | Core, Color, State | List, ListItem, Tree |

---

## Canon Rules Applied

- **Rule #106:** Model-first, CSS-second (tokens define contract)
- **Rule #107:** UI owns visual style (blocks consume tokens)
- **Rule #108:** States are data, not style (data-variant attributes)
- **Rule #109:** Single visual authority (token system)
- **Rule #110:** Reset awareness (no global pollution)
