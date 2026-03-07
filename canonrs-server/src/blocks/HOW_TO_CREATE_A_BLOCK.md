# How to Create a CanonRS Block

## What is a Block?

A **Block** is a reusable component that satisfies ALL these criteria:

### ✅ Required Characteristics

1. **Multi-Product Relevance**
   - Makes sense in MORE THAN ONE product
   - Not tied to specific business logic
   - Example: ✅ `Header` (any app needs header) ❌ `WorkbenchSidebar` (only Workbench)

2. **Slot-Based Architecture**
   - Exposes clear, named slots
   - Consumers provide content, block provides structure
   - Example: `Header` has `logo`, `primary_nav`, `actions` slots

3. **Token Consumption Only**
   - Never defines visual values directly
   - Always consumes tokens via CSS custom properties
   - Example: ❌ `height: 64px` ✅ `height: var(--header-height)`

4. **Semantic Neutrality**
   - Doesn't decide "what it's for"
   - Generic naming (not "AdminHeader" but "Header")
   - Example: ✅ `Alert` ❌ `ErrorAlert`

### ❌ What is NOT a Block

- ❌ Business logic components (`LoginWithGoogle`, `CheckoutForm`)
- ❌ Product-specific components (`WorkbenchToolbar`, `DashboardWidget`)
- ❌ Components with hardcoded data (`CompanyFooter`, `PricingTable`)
- ❌ Visually opinionated components (`RedButton`, `GradientCard`)

---

## Complete Flow: Token → Component → CSS

### Step 1: Define Tokens (Contract)

**Location:** `/opt/docker/monorepo/packages-rust/rs-design/src/tokens/`

**Example:** Creating `alert` block tokens
```bash
# Create token file
cat > /opt/docker/monorepo/packages-rust/rs-design/src/tokens/alert.rs << 'TOKENS'
//! Alert block tokens
//! 
//! **Block:** Alert
//! **Primary Family:** E (Feedback & Status)
//! **Secondary Families:** Core (spacing, border), Color (semantic)

// Structural tokens
pub const ALERT_PADDING_X: &str = "alert.padding.x";
pub const ALERT_PADDING_Y: &str = "alert.padding.y";
pub const ALERT_GAP: &str = "alert.gap";
pub const ALERT_RADIUS: &str = "alert.radius";

// Border tokens
pub const ALERT_BORDER_WIDTH: &str = "alert.border.width";
pub const ALERT_BORDER_STYLE: &str = "alert.border.style";

// Icon tokens
pub const ALERT_ICON_SIZE: &str = "alert.icon.size";

// Semantic color tokens (mapped via variants)
pub const ALERT_BG: &str = "alert.bg";
pub const ALERT_FG: &str = "alert.fg";
pub const ALERT_BORDER_COLOR: &str = "alert.border.color";
pub const ALERT_ICON_COLOR: &str = "alert.icon.color";
TOKENS
```

**Register in mod.rs:**
```rust
// In /opt/docker/monorepo/packages-rust/rs-design/src/tokens/mod.rs
pub mod alert;
pub use alert::*;
```

---

### Step 2: Create Component (Rust)

**Location:** `/opt/docker/monorepo/packages-rust/rs-design/src/blocks/<block_name>/`

**Example:** Creating `alert` block component
```bash
# Create directory
mkdir -p /opt/docker/monorepo/packages-rust/rs-design/src/blocks/alert

# Create component file
cat > /opt/docker/monorepo/packages-rust/rs-design/src/blocks/alert/alert.rs << 'COMPONENT'
//! # Alert Block
//! 
//! **Purpose:** Generic feedback/status message
//! **Token Family:** E (Feedback & Status)
//! 
//! **Slots:**
//! - icon: Visual indicator (optional)
//! - title: Message title (optional)
//! - children: Message content
//! 
//! **Variants:**
//! - info (default)
//! - success
//! - warning
//! - error

use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum AlertVariant {
    Info,
    Success,
    Warning,
    Error,
}

impl AlertVariant {
    fn as_str(&self) -> &'static str {
        match self {
            AlertVariant::Info => "info",
            AlertVariant::Success => "success",
            AlertVariant::Warning => "warning",
            AlertVariant::Error => "error",
        }
    }
}

#[component]
pub fn Alert(
    /// Variant determines semantic styling
    #[prop(default = AlertVariant::Info)]
    variant: AlertVariant,
    /// Optional icon slot
    #[prop(optional)]
    icon: Option<Children>,
    /// Optional title slot
    #[prop(optional)]
    title: Option<String>,
    /// Additional CSS classes
    #[prop(default = String::new(), into)]
    class: String,
    /// Message content
    children: Children,
) -> impl IntoView {
    view! {
        <div
            class=format!("canon-alert {}", class)
            data-block="alert"
            data-variant=variant.as_str()
        >
            {icon.map(|i| view! {
                <div data-slot="icon" class="canon-alert-icon">
                    {i()}
                </div>
            })}

            <div class="canon-alert-content">
                {title.map(|t| view! {
                    <div class="canon-alert-title">
                        {t}
                    </div>
                })}
                <div class="canon-alert-message">
                    {children()}
                </div>
            </div>
        </div>
    }
}
COMPONENT

# Create mod.rs
cat > /opt/docker/monorepo/packages-rust/rs-design/src/blocks/alert/mod.rs << 'MOD'
pub mod alert;
pub use alert::{Alert, AlertVariant};
MOD
```

**Register in blocks/mod.rs:**
```rust
// In /opt/docker/monorepo/packages-rust/rs-design/src/blocks/mod.rs
pub mod alert;
pub use alert::{Alert, AlertVariant};
```

---

### Step 3: Create CSS (Implementation)

**Location:** `/opt/docker/monorepo/packages-rust/rs-design/style/blocks/`

**Example:** Creating `alert.css`
```bash
cat > /opt/docker/monorepo/packages-rust/rs-design/style/blocks/alert.css << 'CSS'
/**
 * Alert Block Styles
 * 
 * Token Family: E (Feedback & Status)
 * 
 * Consumes tokens:
 * - alert.padding.*, alert.gap, alert.radius
 * - alert.border.*, alert.icon.size
 * - alert.bg, alert.fg, alert.border.color, alert.icon.color
 */

.canon-alert {
  display: flex;
  gap: var(--alert-gap);
  padding: var(--alert-padding-y) var(--alert-padding-x);
  border-radius: var(--alert-radius);
  border: var(--alert-border-width) var(--alert-border-style);
  background: var(--alert-bg);
  color: var(--alert-fg);
  border-color: var(--alert-border-color);
}

.canon-alert-icon {
  flex-shrink: 0;
  width: var(--alert-icon-size);
  height: var(--alert-icon-size);
  color: var(--alert-icon-color);
}

.canon-alert-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.canon-alert-title {
  font-weight: 600;
}

.canon-alert-message {
  font-size: 0.875rem;
}

/* Variants - map semantic intent to token values */
[data-block="alert"][data-variant="info"] {
  --alert-bg: var(--color-bg-info);
  --alert-fg: var(--color-fg-info);
  --alert-border-color: var(--color-border-info);
  --alert-icon-color: var(--color-icon-info);
}

[data-block="alert"][data-variant="success"] {
  --alert-bg: var(--color-bg-success);
  --alert-fg: var(--color-fg-success);
  --alert-border-color: var(--color-border-success);
  --alert-icon-color: var(--color-icon-success);
}

[data-block="alert"][data-variant="warning"] {
  --alert-bg: var(--color-bg-warning);
  --alert-fg: var(--color-fg-warning);
  --alert-border-color: var(--color-border-warning);
  --alert-icon-color: var(--color-icon-warning);
}

[data-block="alert"][data-variant="error"] {
  --alert-bg: var(--color-bg-error);
  --alert-fg: var(--color-fg-error);
  --alert-border-color: var(--color-border-error);
  --alert-icon-color: var(--color-icon-error);
}
CSS
```

**Register in blocks.css:**
```css
/* In /opt/docker/monorepo/packages-rust/rs-design/style/blocks.css */
@import './blocks/alert.css';
```

---

## File Structure Summary

After creating a block, you should have:
```
/opt/docker/monorepo/packages-rust/rs-design/
├── src/
│   ├── blocks/
│   │   ├── alert/
│   │   │   ├── alert.rs         ← Component implementation
│   │   │   └── mod.rs           ← Module export
│   │   └── mod.rs               ← Register new block here
│   └── tokens/
│       ├── alert.rs             ← Token definitions
│       └── mod.rs               ← Register tokens here
└── style/
    ├── blocks/
    │   └── alert.css            ← CSS implementation
    └── blocks.css               ← Import alert.css here
```

---

## Usage Example

After creating the block, use it in products:
```rust
use rs_design::blocks::{Alert, AlertVariant};

view! {
    <Alert variant=AlertVariant::Success title="Success!">
        "Your changes have been saved."
    </Alert>

    <Alert 
        variant=AlertVariant::Error
        icon=|| view! { <span>"❌"</span> }
    >
        "An error occurred. Please try again."
    </Alert>
}
```

---

## Checklist

Before considering a block "complete":

- [ ] Token file created with ALL structural, slot, and semantic tokens
- [ ] Tokens registered in `tokens/mod.rs`
- [ ] Component created with clear slots and `data-block` attribute
- [ ] Component registered in `blocks/mod.rs`
- [ ] CSS file created consuming ONLY tokens (no hardcoded values)
- [ ] CSS registered in `blocks.css`
- [ ] Component works in multiple contexts (test in 2+ products)
- [ ] Documentation includes token families and slot descriptions

---

## Canon Rules Applied

This workflow enforces:
- **Rule #106**: Model-first, CSS-second
- **Rule #107**: UI owns visual style (via tokens)
- **Rule #108**: States are data, not style (via `data-variant`)
- **Rule #109**: Single visual authority (tokens)
- **Rule #110**: Reset awareness and boundaries (no global pollution)
