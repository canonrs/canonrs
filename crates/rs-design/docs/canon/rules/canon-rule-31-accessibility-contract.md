# Canon Rule #31: Accessibility Contract (ARIA + Roles)

**Status:** ✅ Enforced  
**Version:** 1.0.0  
**Date:** 2025-01-01

## Principle
Accessibility is **non-negotiable**. All components must be **keyboard navigable**, **screen reader friendly**, and **WCAG 2.1 AA compliant**. ARIA attributes and semantic HTML are mandatory, not optional.

## WCAG 2.1 AA Compliance

### Mandatory Requirements
✅ **Perceivable:** Content visible to all users  
✅ **Operable:** Keyboard navigation works  
✅ **Understandable:** Clear labels and instructions  
✅ **Robust:** Works with assistive technologies

### Success Criteria Checklist
- [ ] 1.4.3 Contrast (Minimum) — 4.5:1 for text
- [ ] 2.1.1 Keyboard — All functionality via keyboard
- [ ] 2.1.2 No Keyboard Trap — Focus can move away
- [ ] 2.4.3 Focus Order — Logical tab sequence
- [ ] 2.4.7 Focus Visible — Clear focus indicators
- [ ] 2.5.5 Target Size — 44×44px minimum touch targets
- [ ] 3.2.1 On Focus — No context changes on focus
- [ ] 4.1.2 Name, Role, Value — Proper ARIA attributes

## Semantic HTML

### Use Correct Elements
```rust
// ✅ CORRECT
view! {
    <button onClick=handler>"Click me"</button>
    <a href="/page">"Navigate"</a>
    <input type="text" />
}

// ❌ FORBIDDEN
view! {
    <div onClick=handler>"Click me"</div>  // Use <button>
    <span onClick=navigate>"Navigate"</span>  // Use <a>
}
```

### Heading Hierarchy
```rust
// ✅ CORRECT: Logical order
view! {
    <h1>"Page Title"</h1>
    <h2>"Section"</h2>
    <h3>"Subsection"</h3>
}

// ❌ FORBIDDEN: Skipping levels
view! {
    <h1>"Page Title"</h1>
    <h3>"Section"</h3>  // Skipped H2
}
```

## ARIA Roles

### Standard Roles
```rust
// Navigation
view! {
    <nav role="navigation" aria-label="Main navigation">
}

// Main content
view! {
    <main role="main">
}

// Complementary content
view! {
    <aside role="complementary">
}

// Search
view! {
    <div role="search">
        <input type="search" aria-label="Search" />
    </div>
}
```

### Interactive Roles
```rust
// Button (when not using <button>)
view! {
    <div role="button" tabindex="0" onClick=handler aria-label="Close">
}

// Checkbox (when custom)
view! {
    <div 
        role="checkbox" 
        aria-checked=is_checked
        tabindex="0"
        onClick=toggle
    >
}

// Tab interface
view! {
    <div role="tablist">
        <button role="tab" aria-selected="true">
    </div>
    <div role="tabpanel">
}
```

### Widget Roles
```rust
// Dialog
view! {
    <div role="dialog" aria-labelledby="dialog-title" aria-modal="true">
        <h2 id="dialog-title">"Confirm Action"</h2>
    </div>
}

// Alert
view! {
    <div role="alert" aria-live="assertive">
        "Error: Invalid input"
    </div>
}

// Menu
view! {
    <div role="menu">
        <button role="menuitem">
    </div>
}
```

## ARIA Attributes

### Labels
```rust
// aria-label: Invisible label
view! {
    <button aria-label="Close dialog">
        <XIcon />
    </button>
}

// aria-labelledby: Reference visible label
view! {
    <div role="dialog" aria-labelledby="title">
        <h2 id="title">"Settings"</h2>
    </div>
}

// aria-describedby: Additional description
view! {
    <input 
        id="email" 
        aria-describedby="email-hint"
    />
    <span id="email-hint">"We'll never share your email"</span>
}
```

### States
```rust
// aria-checked
view! {
    <div role="checkbox" aria-checked=is_checked>
}

// aria-pressed (toggle button)
view! {
    <button aria-pressed=is_active>
}

// aria-selected (tabs, lists)
view! {
    <button role="tab" aria-selected=is_active>
}

// aria-expanded (collapsible)
view! {
    <button aria-expanded=is_open aria-controls="panel">
    <div id="panel">
}

// aria-disabled
view! {
    <button aria-disabled="true" disabled>
}
```

### Properties
```rust
// aria-required
view! {
    <input aria-required="true" required />
}

// aria-invalid
view! {
    <input aria-invalid=has_error />
}

// aria-live
view! {
    <div aria-live="polite">  // Announcements
}

// aria-hidden
view! {
    <div aria-hidden="true">  // Decorative content
}
```

### Relationships
```rust
// aria-controls
view! {
    <button aria-controls="menu" aria-expanded=is_open>
    <div id="menu">
}

// aria-owns
view! {
    <div aria-owns="child-list">
    <ul id="child-list">
}

// aria-activedescendant (combobox)
view! {
    <input 
        role="combobox" 
        aria-activedescendant=active_option_id
        aria-controls="listbox"
    />
    <ul id="listbox" role="listbox">
}
```

## Keyboard Navigation

### Focus Management
```rust
// Focusable elements
const FOCUSABLE: &str = "focus:outline-none focus:ring-2 focus:ring-primary";

// Tab order
view! {
    <button tabindex="0">"First"</button>
    <button tabindex="0">"Second"</button>
    <button tabindex="-1">"Skip in tab order"</button>
}
```

### Keyboard Event Handlers
```rust
#[component]
pub fn KeyboardComponent() -> impl IntoView {
    let handle_key = move |ev: KeyboardEvent| {
        match ev.key().as_str() {
            "Enter" | " " => {
                // Activate
                ev.prevent_default();
            }
            "Escape" => {
                // Close/cancel
                ev.prevent_default();
            }
            "ArrowUp" | "ArrowDown" => {
                // Navigate
                ev.prevent_default();
            }
            _ => {}
        }
    };
    
    view! {
        <div onKeyDown=handle_key>
    }
}
```

### Focus Trapping (Modals)
```rust
// Keep focus inside dialog
#[component]
pub fn Dialog() -> impl IntoView {
    let dialog_ref = NodeRef::<html::Div>::new();
    
    Effect::new(move |_| {
        if let Some(dialog) = dialog_ref.get() {
            // Focus first focusable element
            if let Some(first) = dialog
                .query_selector("button, [href], input, select, textarea, [tabindex]:not([tabindex='-1'])")
                .ok()
                .flatten()
            {
                let _ = first.dyn_ref::<web_sys::HtmlElement>()
                    .map(|el| el.focus());
            }
        }
    });
    
    view! {
        <div node_ref=dialog_ref role="dialog" aria-modal="true">
    }
}
```

## Screen Reader Support

### Announcements
```rust
// Live regions
view! {
    <div role="status" aria-live="polite">
        "Item added to cart"
    </div>
}

// Assertive announcements (errors)
view! {
    <div role="alert" aria-live="assertive">
        "Form submission failed"
    </div>
}
```

### Hidden Content
```rust
// Visually hidden but screen reader accessible
const SR_ONLY: &str = "sr-only";

view! {
    <button>
        <Icon />
        <span class=SR_ONLY>"Close"</span>
    </button>
}
```

### Skip Links
```rust
// Allow keyboard users to skip navigation
view! {
    <a href="#main-content" class="sr-only focus:not-sr-only">
        "Skip to main content"
    </a>
    <nav>...</nav>
    <main id="main-content">
}
```

## Form Accessibility

### Labels
```rust
// Always associate labels
view! {
    <label for="email">"Email"</label>
    <input id="email" type="email" />
}

// OR wrap input
view! {
    <label>
        "Email"
        <input type="email" />
    </label>
}
```

### Error Messages
```rust
view! {
    <div>
        <label for="password">"Password"</label>
        <input 
            id="password"
            aria-invalid=has_error
            aria-describedby=move || if has_error.get() { Some("password-error") } else { None }
        />
        <Show when=has_error>
            <span id="password-error" role="alert" class="text-destructive">
                "Password must be at least 8 characters"
            </span>
        </Show>
    </div>
}
```

### Required Fields
```rust
view! {
    <label for="name">
        "Name"
        <span aria-label="required">"*"</span>
    </label>
    <input id="name" required aria-required="true" />
}
```

## Touch Targets

### Minimum Size (WCAG 2.5.5)
```css
/* Ensure 44×44px minimum touch targets */
@media (pointer: coarse) {
  button, a, input[type="checkbox"], input[type="radio"] {
    min-width: 44px;
    min-height: 44px;
  }
}
```

### Spacing
```rust
// Adequate spacing between interactive elements
view! {
    <div class="flex gap-2">  // Minimum 8px gap
        <button>"Button 1"</button>
        <button>"Button 2"</button>
    </div>
}
```

## Testing

### Automated Testing
```bash
# Axe DevTools
npm install --save-dev @axe-core/cli

# Run accessibility audit
axe http://localhost:3000
```

### Manual Testing
✅ **Keyboard only:** Unplug mouse, navigate with Tab/Enter/Esc  
✅ **Screen reader:** Test with NVDA (Windows) or VoiceOver (Mac)  
✅ **Zoom:** Test at 200% zoom  
✅ **Color blindness:** Use Chrome DevTools emulation  
✅ **Touch:** Test on mobile device

### Checklist
- [ ] All interactive elements keyboard accessible
- [ ] Focus indicators visible
- [ ] Screen reader announces all content
- [ ] No keyboard traps
- [ ] Forms have labels and error messages
- [ ] Touch targets ≥44×44px on mobile
- [ ] Headings in logical order
- [ ] Color contrast passes WCAG AA

## Prohibited Patterns

### ❌ Inaccessible Clickables
```rust
// FORBIDDEN
view! {
    <div onClick=handler>"Click me"</div>
}

// CORRECT
view! {
    <button onClick=handler>"Click me"</button>
}
```

### ❌ Missing Labels
```rust
// FORBIDDEN
view! {
    <input type="text" />
}

// CORRECT
view! {
    <label for="name">"Name"</label>
    <input id="name" type="text" />
}
```

### ❌ Icon-Only Buttons Without Labels
```rust
// FORBIDDEN
view! {
    <button><XIcon /></button>
}

// CORRECT
view! {
    <button aria-label="Close">
        <XIcon />
    </button>
}
```

### ❌ Removing Focus Indicators
```css
/* FORBIDDEN */
*:focus {
  outline: none;
}
```

## Validation Checklist
- [ ] All components keyboard navigable
- [ ] All interactive elements have ARIA roles/labels
- [ ] Focus order is logical
- [ ] Screen reader announces all content correctly
- [ ] Forms have proper labels and error handling
- [ ] Touch targets meet 44×44px minimum
- [ ] Color contrast passes WCAG AA
- [ ] No keyboard traps
- [ ] Skip links present
- [ ] Headings in correct hierarchy

## References
- [WCAG 2.1 Guidelines](https://www.w3.org/WAI/WCAG21/quickref/)
- [ARIA Authoring Practices](https://www.w3.org/WAI/ARIA/apg/)
- [WebAIM](https://webaim.org/)
- [Inclusive Components](https://inclusive-components.design/)
- [Canon Rule #23: State Tokens](./canon-rule-23-state-tokens.md)
- [Canon Rule #30: Iconography System](./canon-rule-30-iconography-system.md)
