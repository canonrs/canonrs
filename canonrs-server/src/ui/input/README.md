# Input Component

**Type:** Pure Component (Type 1)  
**Status:** Production-Ready  
**Tokens:** 100% Canonical (A+B+C)  
**Scope:** Generic Input (see [inputs-scope.md](/docs/canon/inputs-scope.md))

---

## Purpose

Generic text input field for forms. Fully token-driven, SSR-safe, accessible. Supports validation states and multiple input types.

---

## Usage
```rust
use rs_design::ui::input::*;

// Basic
let value = RwSignal::new(String::new());
<Input value=value placeholder="Enter text" />

// With type
<Input 
    value=value 
    input_type=InputType::Email 
    placeholder="email@example.com" 
/>

// With size
<Input 
    value=value 
    size=InputSize::Lg 
    placeholder="Large input" 
/>

// With validation
<Input 
    value=value 
    validation=InputValidation::Error 
    placeholder="Fix this field" 
/>

// With callback
<Input 
    value=value 
    on_input=Callback::new(|val| {
        leptos::logging::log!("Changed: {}", val);
    })
/>
```

---

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `id` | `String` | `""` | HTML id attribute |
| `input_type` | `InputType` | `Text` | Input type (text, email, etc.) |
| `placeholder` | `String` | `""` | Placeholder text |
| `value` | `RwSignal<String>` | Required | Reactive value binding |
| `on_input` | `Option<Callback<String>>` | `None` | Input change callback |
| `size` | `Option<InputSize>` | `Md` | Input size |
| `validation` | `InputValidation` | `None` | Validation state |
| `disabled` | `bool` | `false` | Disabled state |
| `class` | `String` | `""` | Additional classes |

---

## Variants

### InputType
- `Text` (default) - Plain text
- `Email` - Email validation
- `Password` - Masked text
- `Number` - Numeric input
- `Tel` - Telephone number
- `Url` - URL validation
- `Search` - Search input
- `Date` - Date picker
- `Time` - Time picker
- `DateTime` - Date and time picker

### InputSize
- `Sm` - Small (compact forms)
- `Md` (default) - Medium (standard)
- `Lg` - Large (prominent fields)

### InputValidation
- `None` (default) - No validation state
- `Error` - Error feedback (red border/ring)
- `Success` - Success feedback (green border/ring)
- `Warning` - Warning feedback (yellow border/ring)

---

## Tokens Used

### Core (A)
- `radius.md` - Border radius
- `space.{sm,md,lg}` - Horizontal padding per size
- `border.width.thin` - Border width
- `shadow.sm` - Subtle elevation
- `z.base` - Stacking context

### Typography (B)
- `font.family.sans` - Font family
- `font.size.{sm,md,lg}` - Text size per size variant
- `font.weight.regular` - Default weight
- `line.height.normal` - Line spacing

### Color (B)
- `color.bg.surface` - Background color
- `color.fg.default` - Text color

### State (B)
- `state.focus.ring` - Focus indicator
- `state.disabled.opacity` - Disabled appearance
- `state.hover.opacity` - Hover feedback
- `state.active.opacity` - Active/typing state

### Motion (B)
- `motion.duration.fast` - Transition speed
- `motion.ease.standard` - Easing function

### Forms (C)
- `field.height` - Input height
- `field.padding` - Vertical padding
- `field.border` - Border token (applied via validation)
- `field.placeholder` - Placeholder text color
- `validation.{error,success,warning}` - Validation colors

**Note:** `input.masking` is NOT used. See [inputs-scope.md](/docs/canon/inputs-scope.md) for rationale.

**Total: 100% Canonical Coverage (Generic Input Scope)**

---

## Accessibility

- ✅ Keyboard navigable (Tab, typing)
- ✅ Focus visible (`state.focus.ring`)
- ✅ Disabled state (`disabled` attribute + visual)
- ✅ Semantic HTML (`<input>`)
- ✅ Type attribute for validation hints
- ✅ Placeholder for context
- ⚠️ **Important:** Always pair with `<Label>` for screen readers

---

## Canon Rules Applied

- **Rule #1 (Types):** Type 1 (pure rendering, no effects)
- **Rule #2 (Ownership):** Controlled via `RwSignal<String>`
- **Rule #5 (SSR):** No browser APIs, fully SSR-safe
- **Canon (Visual State):** 100% token-driven, zero hardcoded values
- **Canon (Input Scope):** Generic input - no masking logic

---

## States

| State | Behavior |
|-------|----------|
| Default | Normal input state |
| Hover | `state.hover.opacity` applied |
| Active (typing) | `state.active.opacity` applied |
| Focus | `state.focus.ring` visible |
| Disabled | `state.disabled.opacity` + no interaction |
| Error | Red border via `validation.error` |
| Success | Green border via `validation.success` |
| Warning | Yellow border via `validation.warning` |

---

## File Structure
```
input/
├── input.rs           # Main component
├── variants.rs        # InputSize, InputValidation
├── types.rs           # InputType enum
├── mod.rs             # Module exports
└── README.md          # This file
```

---

## Examples

### Email Input with Validation
```rust
let email = RwSignal::new(String::new());
let is_valid = Signal::derive(move || email.get().contains('@'));

<Input 
    value=email
    input_type=InputType::Email
    placeholder="your@email.com"
    validation=if !is_valid.get() && !email.get().is_empty() {
        InputValidation::Error
    } else {
        InputValidation::None
    }
/>
```

### Password Input
```rust
let password = RwSignal::new(String::new());

<Input 
    value=password
    input_type=InputType::Password
    placeholder="Enter password"
/>
```

### Large Search Input
```rust
let search = RwSignal::new(String::new());

<Input 
    value=search
    input_type=InputType::Search
    size=InputSize::Lg
    placeholder="Search..."
    on_input=Callback::new(|query| {
        // Trigger search
    })
/>
```

### Disabled Input
```rust
let readonly_value = RwSignal::new("Cannot edit".to_string());

<Input 
    value=readonly_value
    disabled=true
/>
```

---

## Related Components

For specialized inputs with formatting:
- `OTPInput` - One-time password (6-digit codes)
- `PhoneInput` - Phone number with mask
- `CPFInput` - Brazilian CPF with mask
- `DateInput` - Date with format enforcement

These components use `input.masking` token. See [inputs-scope.md](/docs/canon/inputs-scope.md).

---

## Migration from Old Input
```rust
// Old (string type)
<Input input_type="email" />

// New (enum)
<Input input_type=InputType::Email />

// Old (error prop)
<Input error=true />

// New (validation state)
<Input validation=InputValidation::Error />

// Old (no size control)
<Input />

// New (explicit size)
<Input size=InputSize::Md />
```

---

**Status:** ✅ Production-Ready  
**Canonicity:** 100% (Generic Input Scope)  
**Last Updated:** 2025-12-30
