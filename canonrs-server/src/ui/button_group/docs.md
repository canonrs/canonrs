# Button Group

## Overview

### What it is
Button Group is a utility component that groups multiple buttons into a single logical and semantic unit. It ensures that grouped actions are visually and semantically connected.

### The problem it solves
Typical grouped buttons are implemented using div containers, losing semantic grouping and accessibility context. This results in poor usability and inconsistent interaction patterns.

### How CanonRS enforces it
ButtonGroupPrimitive enforces role="group" and uses ToggleState to control attachment. This ensures consistent grouping semantics and visual cohesion.

## Usage

### Basic usage
```rust
view! {
  <ButtonGroup>
    <Button>"Left"</Button>
    <Button>"Right"</Button>
  </ButtonGroup>
}
```

### With variants
Variant control is delegated to child buttons.

### With sizes
Size is inherited from child buttons.

### With states
Attachment state defines visual grouping.

### Composition patterns
ButtonGroup wraps multiple Button components and ensures consistent layout.

## Variants

### Default
Single grouping behavior.

## Sizes

### Default
Inherited from children.

## States

### Attached
Buttons appear visually connected.

### Detached
Buttons appear separate.

## API Reference

### Props
- class: String
- attached: ToggleState
- aria_label: Option<String>

### Types
- ToggleState

## Governance

### Canon Rules applied
CR-001, CR-004

### What the compiler enforces
Semantic grouping and accessibility.

### Before vs After
Before: div-based grouping.  
After: semantic group with enforced behavior.

## Use Cases

### Toolbar actions
Group related actions together.

### Segmented controls
Provide multiple related choices.

## Related Components

### Button
Individual action component.

### IconButton
Icon-based actions.
