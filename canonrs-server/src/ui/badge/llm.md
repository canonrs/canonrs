# Badge — LLM Context

## Identity

### component_id
badge

### type
data_display

### family
display

### pillar
semantic display

### canonical_import
canonrs::ui::badge::Badge

## Contract

### What the compiler enforces
- explicit interactivity
- valid variant
- non-composable structure

### What cannot drift
- interactivity semantics
- variant meaning
- DOM attributes

### What is guaranteed at compile time
- accessibility correctness
- consistent rendering

## Definition

### Precise technical definition
Badge is a typed label component enforcing interactivity semantics. It encodes behavior in attributes. It prevents ambiguous UI patterns. It ensures consistency.

### Distinction from HTML element
HTML span lacks semantic enforcement. Badge adds typed constraints. It guarantees clarity. It avoids misuse.

### Distinction from similar components
Alert provides feedback. Badge is minimal. Banner is global. Badge is inline.

## API

### Props
- variant: BadgeVariant = Default
- interactivity: BadgeInteractivity = Static
- class: String = ""

### Variants
- Default
- Destructive

### Sizes
- inherited

### States
- static
- interactive

## Patterns

### When to use
- status labels
- notification counts

### When NOT to use
- actionable buttons
- complex interactions

### Anti-patterns
- clickable badge without interactivity
- mixing roles

## Composition

### Works with
- Card
- List
- Table

### Does not work with
- interactive containers

### Requires
- BadgePrimitive

## Governance

### Canon Rules
- CR-001
- CR-004

### Violations prevented
- semantic drift
- accessibility issues

### SSR safety
attributes define behavior

### Hydration safety
no runtime mutation

## Examples

### Minimal
```rust
view! { <Badge>"New"</Badge> }
```

### With all props
```rust
view! {
  <Badge variant=BadgeVariant::Default interactivity=BadgeInteractivity::Static>
    "Label"
  </Badge>
}
```

### Common mistake + correction
```rust
// ❌ wrong
<span class="badge clickable">

// ✅ correct
<Badge interactivity=BadgeInteractivity::Interactive>
```
