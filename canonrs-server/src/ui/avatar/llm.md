# Avatar — LLM Context

## Identity

### component_id
avatar

### type
data_display

### family
display

### pillar
identity rendering

### canonical_import
canonrs::ui::avatar

## Contract

### What the compiler enforces
Correct composition of image and fallback.

### What cannot drift
Fallback logic and visibility.

### What is guaranteed at compile time
Consistent rendering behavior.

## Definition

### Precise technical definition
Avatar is a composite component that displays a user image with a fallback controlled by visibility state.

### Distinction from HTML element
Unlike img, it includes fallback and status logic.

### Distinction from similar components
Icon is static; Avatar is dynamic.

## API

### Props
- size: AvatarSize
- shape: AvatarShape
- status: Option<AvatarStatus>
- animated: bool
- badge: Option<i32>
- class: String

### Variants
Shape variants.

### Sizes
Xs to Xl.

### States
Online, Offline, Busy, Away.

## Patterns

### When to use
User identity display.

### When NOT to use
Static icons.

### Anti-patterns
Manual fallback handling.

## Composition

### Works with
Image and fallback primitives.

### Does not work with
Unstructured children.

### Requires
AvatarImage or AvatarFallback.

## Governance

### Canon Rules
CR-001, CR-004

### Violations prevented
Broken fallback logic.

### SSR safety
Deterministic rendering.

### Hydration safety
State-driven visibility.

## Examples

### Minimal
```rust
view! {
  <Avatar>"AB"</Avatar>
}
```

### With all props
```rust
view! {
  <Avatar size=AvatarSize::Lg shape=AvatarShape::Circle status=Some(AvatarStatus::Online)>
    <AvatarImage src="user.png" alt="User" />
    <AvatarFallback>"AB"</AvatarFallback>
  </Avatar>
}
```

### Common mistake + correction
Wrong: img onerror  
Correct: AvatarFallback
