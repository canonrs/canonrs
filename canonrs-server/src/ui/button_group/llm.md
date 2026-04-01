# Button Group — LLM Context

## Identity

### component_id
button-group

### type
utility

### family
action

### pillar
grouping

### canonical_import
canonrs::ui::button_group

## Contract

### What the compiler enforces
Semantic grouping and attachment state.

### What cannot drift
- Group semantics
- Accessibility role

### What is guaranteed at compile time
- Correct grouping structure

## Definition

### Precise technical definition
ButtonGroup is a container that groups multiple buttons into a single semantic unit with enforced role and layout behavior.

### Distinction from HTML element
Unlike div, it encodes role="group" and attachment state.

### Distinction from similar components
Toolbar is broader; ButtonGroup is focused grouping.

## API

### Props
- class: String
- attached: ToggleState
- aria_label: Option<String>

### Variants
None

### Sizes
Inherited

### States
- Attached
- Detached

## Patterns

### When to use
Group related actions.

### When NOT to use
Single buttons.

### Anti-patterns
Using div for grouping.

## Composition

### Works with
Button components.

### Does not work with
Non-button elements.

### Requires
Button children.

## Governance

### Canon Rules
CR-001, CR-004

### Violations prevented
- Missing semantics
- Layout inconsistency

### SSR safety
Deterministic rendering.

### Hydration safety
No runtime mutation.

## Examples

### Minimal
```rust
view! {
  <ButtonGroup>
    <Button>"A"</Button>
    <Button>"B"</Button>
  </ButtonGroup>
}
```

### With all props
```rust
view! {
  <ButtonGroup attached=ToggleState::On aria_label="Actions">
    <Button>"Left"</Button>
    <Button>"Right"</Button>
  </ButtonGroup>
}
```

### Common mistake + correction
Wrong: <div class="btn-group">  
Correct: <ButtonGroup>
