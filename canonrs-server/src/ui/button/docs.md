# Button

## Overview

### What it is
Button is a core interactive component in CanonRS designed to trigger actions in a deterministic and governed way. It enforces visual and behavioral consistency through typed enums for variant and size. Unlike traditional buttons, it eliminates reliance on string-based class composition. This guarantees consistent rendering across SSR and hydration.

### The problem it solves
Typical button implementations rely on loosely defined class names, leading to inconsistent styles and states. Developers often mix variants and sizes incorrectly, causing UI drift. Accessibility attributes are frequently forgotten or misused. Button solves this by enforcing strict contracts at compile time.

### How CanonRS enforces it
ButtonVariant and ButtonSize are strongly typed enums that define allowed combinations. DisabledState ensures correct disabled behavior without manual attribute handling. The primitive encodes all behavior into data-rs attributes. The compiler guarantees that invalid combinations cannot exist.

## Usage

### Basic usage
```rust
view! {
  <Button>"Submit"</Button>
}
```
This creates a primary button with default size. The component ensures consistent styling. No manual classes are required. Behavior is predictable.

### With variants
Variants control the visual and semantic intent of the button. Each variant is enforced through enums. This prevents invalid styling combinations. It ensures consistency across the UI.

### With sizes
Sizes define the physical scale of the button. They are mapped to core sizes internally. This ensures consistent spacing and typography. Developers cannot introduce arbitrary sizes.

### With states
State is controlled via DisabledState. This ensures accessibility and correct interaction behavior. No manual disabled attributes are needed. The state is encoded in the DOM.

### Composition patterns
Button is a leaf component and does not accept nested interactive elements. It integrates with layouts and groups. Composition does not break its contract. Behavior remains isolated.

## Variants

### Primary
Primary is used for main actions. It draws the most attention. It should be used sparingly. It represents the main CTA.
```rust
<Button variant=ButtonVariant::Primary>"Submit"</Button>
```

### Secondary
Secondary is used for alternative actions. It has less visual weight. It complements the primary button. It maintains hierarchy.
```rust
<Button variant=ButtonVariant::Secondary>"Cancel"</Button>
```

### Outline
Outline provides minimal emphasis. It is useful for secondary actions. It blends with background. It maintains clarity.
```rust
<Button variant=ButtonVariant::Outline>"More"</Button>
```

### Ghost
Ghost removes background styling. It is used for subtle actions. It integrates into layouts. It reduces visual noise.
```rust
<Button variant=ButtonVariant::Ghost>"Edit"</Button>
```

### Link
Link variant mimics hyperlink behavior. It is used for navigation-like actions. It maintains semantic clarity. It avoids misuse of anchor tags.
```rust
<Button variant=ButtonVariant::Link>"Go"</Button>
```

### Destructive
Destructive highlights critical actions. It warns the user. It should be used carefully. It ensures clear intent.
```rust
<Button variant=ButtonVariant::Destructive>"Delete"</Button>
```

## Sizes

### Xs
Extra small buttons are used in dense layouts. They reduce visual footprint. They are suitable for inline actions. They maintain usability.

### Sm
Small buttons are used in compact interfaces. They balance readability and space. They are common in toolbars. They maintain consistency.

### Md
Medium is the default size. It fits most use cases. It ensures readability and accessibility. It is widely used.

### Lg
Large buttons emphasize important actions. They improve visibility. They are used in landing pages. They draw attention.

### Xl
Extra large buttons are used for major CTAs. They maximize prominence. They are used sparingly. They ensure focus.

## States

### Enabled
Enabled state allows interaction. It represents default behavior. It ensures accessibility. It is the normal state.

### Disabled
Disabled state prevents interaction. It communicates unavailable actions. It is enforced via DisabledState. It ensures correct semantics.
```rust
<Button disabled=DisabledState::Disabled>"Submit"</Button>
```

## API Reference

### Props
- variant: ButtonVariant (default: Primary)
- size: ButtonSize (default: Md)
- disabled: DisabledState (default: Enabled)
- class: String
- aria_label: Option<String>

### Types
- ButtonVariant
- ButtonSize
- DisabledState

## Governance

### Canon Rules applied
- CR-001
- CR-004
- CR-148

### What the compiler enforces
- valid variant/size combinations
- accessibility attributes
- disabled behavior

### Before vs After
Before: class-based styling  
After: typed contract

## Use Cases

### form submit
Button ensures consistent submission behavior. It maintains accessibility. It prevents invalid states. It integrates with forms.

### cta actions
Buttons highlight primary actions. They guide user interaction. They maintain hierarchy. They ensure clarity.

## Related Components

### button_group
ButtonGroup groups buttons logically. Button is standalone. Use ButtonGroup for grouped actions. Use Button for single actions.

### link
Link is for navigation. Button is for actions. Use link for routing. Use button for events.
