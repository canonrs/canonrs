# Badge

## Overview

### What it is
Badge is a governed data display component in CanonRS used to represent status, labels, or counts. It provides a clear semantic distinction between static and interactive badges through typed interactivity. This eliminates ambiguity in behavior and ensures consistent rendering. The component enforces strict contracts to prevent misuse across the UI system.

### The problem it solves
In typical implementations, badges often mix static and interactive behaviors without clear intent. Developers may unintentionally create clickable badges that behave inconsistently. This leads to accessibility issues and unpredictable UX. Badge solves this by enforcing interactivity through explicit types.

### How CanonRS enforces it
BadgeInteractivity defines whether the badge is static or interactive. This is encoded into data-rs attributes at the primitive level. The compiler ensures that invalid combinations are not allowed. This guarantees consistent semantics and prevents accidental interaction patterns.

## Usage

### Basic usage
```rust
view! {
  <Badge>"New"</Badge>
}
```
This creates a static badge with default styling. The component ensures consistent structure. No manual class management is needed. Behavior is predictable.

### With variants
Variants define visual styling such as default or emphasis. They are controlled through BadgeVariant. This ensures visual consistency. Developers cannot introduce arbitrary styles.

### With sizes
Badge does not define explicit sizes. It inherits typography and spacing from context. This ensures flexibility. It avoids coupling layout with logic.

### With states
State is derived from interactivity. Static badges remain passive. Interactive badges respond to user input. This distinction is enforced at compile time.

### Composition patterns
Badge is non-composable and used as a leaf node. It can be placed inside layouts or containers. It does not accept nested interactive elements. This ensures semantic clarity.

## Variants

### Default
Default variant provides neutral styling. It is suitable for general labels. It maintains visual consistency across the UI. It does not imply urgency.
```rust
<Badge variant=BadgeVariant::Default>"Label"</Badge>
```

### Destructive
Destructive variant highlights critical status. It is used for errors or warnings. It draws user attention immediately. This ensures clear communication.
```rust
<Badge variant=BadgeVariant::Destructive>"Error"</Badge>
```

## Sizes

### Default
Badge size adapts to content. It ensures readability and consistency. It scales with typography. This avoids layout issues.

## States

### Static
Static state means the badge is non-interactive. It is purely informational. It does not respond to user input. This ensures clarity.
```rust
<Badge interactivity=BadgeInteractivity::Static>"New"</Badge>
```

### Interactive
Interactive state allows user interaction. It behaves like a button or link. This is explicitly defined. It prevents accidental clickability.
```rust
<Badge interactivity=BadgeInteractivity::Interactive>"Click"</Badge>
```

## API Reference

### Props
- variant: BadgeVariant (default: Default)
- interactivity: BadgeInteractivity (default: Static)
- class: String

### Types
- BadgeVariant
- BadgeInteractivity

## Governance

### Canon Rules applied
- CR-001
- CR-004

### What the compiler enforces
- Interactivity correctness
- Valid variant usage
- No mixed semantics

### Before vs After
Before: ambiguous clickable badge  
After: explicit interactivity contract

## Use Cases

### status labels
Badge is used to display status such as active or inactive. It ensures clear semantics. It avoids confusion between clickable and static elements. This improves UX.

### notifications
Badge can represent notification counts. It remains consistent across contexts. It avoids layout drift. Behavior remains predictable.

## Related Components

### alert
Alert provides richer feedback. Badge is minimal and inline. Use alert for messages. Use badge for labels.

### banner
Banner displays page-level messages. Badge is inline. Use banner for global context. Use badge for localized info.
