---
component: Accordion
layer: UI
status: Stable
since: v1.0
last_review: 2025-01-18
ownership: canonrs
keywords:
  - design system
  - dioxus
  - ssr
  - accordion
  - disclosure
  - enterprise ui
path_primitive: /opt/docker/monorepo/packages-rust/rs-design/src/primitives/accordion.rs
path_ui: /opt/docker/monorepo/packages-rust/rs-design/src/ui/accordion/
---

# Accordion

## 1. Conceptual Introduction

The Accordion is a **UI component** that provides ergonomic composition of collapsible content sections in an exclusive (single-open) interaction pattern. It serves as the canonical enterprise solution for FAQ pages, documentation navigation, settings panels, and structured information disclosure.

The Accordion exists in the **UI layer** because it provides:
- Default behavior (exclusive single-open)
- Ergonomic API with sensible defaults
- Semantic intent (collapsible structured content)
- Composition of primitives without business logic

**What it is NOT:**
- Not a primitive (it composes primitives)
- Not a runtime behavior (no browser APIs)
- Not a styling layer (delegates to CSS via data-attributes)
- Not a navigation component (use Tabs for horizontal navigation)

---

## 2. Architectural Responsibility (Contract)

### Responsibility

The Accordion UI component:
- **Composes** `AccordionPrimitive`, `AccordionItem`, `AccordionTrigger`, and `AccordionContent`
- **Declares** exclusive open behavior via `data-collapsible` attribute
- **Exposes** ergonomic API with `default_value` and `collapsible` props
- **Emits** SSR-safe HTML with data-attributes for runtime hydration
- **Guarantees** semantic structure and accessibility contracts

### Non-Responsibility

The Accordion UI component explicitly does NOT:
- ❌ Execute browser APIs (clipboard, focus, keyboard)
- ❌ Manage state imperatively (state lives in DOM via data-attributes)
- ❌ Apply CSS classes or inline styles
- ❌ Register global event listeners
- ❌ Perform side effects or mutations

**Side effects are PROHIBITED.**  
All runtime behavior is delegated to the Shell Runtime JS layer.

---

## 3. Position in CanonRS Ecosystem

The Accordion participates in the canonical CanonRS flow:
```text
Page/Block (usage)
  ↓
UI Component (Accordion) — ergonomic API, composition
  ↓
Primitives (AccordionPrimitive, AccordionItem, etc.) — HTML + data-attributes
  ↓
SSR Render — static HTML emitted
  ↓
Shell Runtime JS — event delegation, state management
  ↓
Browser API — DOM manipulation, ARIA, keyboard navigation
  ↓
CSS — styling via [data-accordion-*] selectors
```

**SSR Context:**
- Accordion renders complete HTML structure on server
- No client-side JS required for initial render
- Runtime JS hydrates interactive behavior on client

**Hydration:**
- Shell Runtime reads `data-collapsible` and `data-default-value`
- Attaches delegated event listeners to `[data-accordion-trigger]`
- Manages `data-state="open|closed"` on `[data-accordion-item]`

---

## 4. Tokens Applied

The Accordion UI component does **not directly apply tokens**—it delegates to CSS.  
The CSS layer (`style/ui/accordion.css`) consumes the following token families:

### Layout Tokens
- `--space-sm` (accordion gap)
- `--space-md` (trigger padding, content padding)

### Typography Tokens
- `--font-family-sans`
- `--font-size-sm`
- `--font-weight-medium`
- `--line-height-normal`

### Color Tokens
- `--color-fg-default` (text color)
- `--color-fg-muted` (icon color)
- `--color-bg-muted` (hover/active background)
- `--color-border-muted` (item separator)

### Border & Radius Tokens
- `--border-width-hairline`
- `--radius-sm`

### State Tokens
- `--state-hover-opacity`
- `--state-active-opacity`
- `--state-focus-opacity`

### Accordion Family Tokens (Defined in CSS)
- `--accordion-gap`
- `--accordion-trigger-padding`
- `--accordion-content-padding`
- `--accordion-icon-size`
- `--accordion-icon-rotation-open`
- `--accordion-icon-svg` (themeable icon)

**Token Resolution:**  
UI component emits data-attributes → CSS applies tokens → Browser renders.

---

## 5. Technical Structure (How It Works)

### SSR Render Phase

The Accordion component renders to static HTML:
```html
<div data-accordion="" data-collapsible="false" data-default-value="item-1">
  <div data-accordion-item="">
    <button data-accordion-trigger="" type="button">
      Section Title
    </button>
    <div data-accordion-content="">
      <div>Content goes here</div>
    </div>
  </div>
</div>
```

**Key contracts:**
- `data-accordion` marks root container
- `data-collapsible="true|false"` controls whether all items can be closed
- `data-default-value="id"` specifies initially open item
- `data-accordion-trigger` marks interactive button
- `data-accordion-content` marks collapsible region

### Runtime Hydration Phase

Shell Runtime JS:
1. Queries `[data-accordion]` elements
2. Reads `data-collapsible` and `data-default-value` attributes
3. Sets initial `data-state="open|closed"` on items
4. Attaches delegated click handler to `[data-accordion-trigger]`
5. Manages exclusive open behavior (closes siblings on trigger click)

### CSS Styling Phase

CSS selectors target data-attributes:
```css
[data-accordion-item][data-state="open"] [data-accordion-content] {
  grid-template-rows: 1fr; /* expand */
}

[data-accordion-trigger]::after {
  background-image: var(--accordion-icon-svg);
  transform: rotate(0deg);
}

[data-accordion-item][data-state="open"] [data-accordion-trigger]::after {
  transform: rotate(var(--accordion-icon-rotation-open));
}
```

**No classes. No inline styles. Only data-attributes.**

---

## 6. Execution Flow
```text
1. SSR Render (Server)
   ↓
   Accordion component executes
   ↓
   Composes primitives with data-attributes
   ↓
   Emits static HTML

2. HTML Delivery (Network)
   ↓
   Browser receives HTML
   ↓
   Renders static structure (no interactivity yet)

3. Shell Runtime Hydration (Client)
   ↓
   Runtime JS executes
   ↓
   Reads data-accordion attributes
   ↓
   Sets initial data-state based on default_value
   ↓
   Attaches event delegation

4. User Interaction
   ↓
   User clicks [data-accordion-trigger]
   ↓
   Event delegation catches click
   ↓
   Runtime toggles data-state on item
   ↓
   CSS reacts to [data-state] change
   ↓
   Browser animates content expansion
```

**Critical:** State lives in DOM (`data-state`), not in JS memory.  
This ensures SSR compatibility and simplifies debugging.

---

## 7. Canonical Use Cases

### FAQ Page
```rust
use rs_design::ui::Accordion;

rsx! {
    Accordion {
        default_value: "faq-1".to_string(),
        collapsible: true,
        
        AccordionItem {
            AccordionTrigger { "What is CanonRS?" }
            AccordionContent {
                p { "CanonRS is an enterprise design system..." }
            }
        }
        
        AccordionItem {
            AccordionTrigger { "How does SSR work?" }
            AccordionContent {
                p { "SSR renders HTML on the server..." }
            }
        }
    }
}
```

### Settings Panel (Non-Collapsible)
```rust
Accordion {
    default_value: "general".to_string(),
    collapsible: false, // Always one section open
    
    AccordionItem {
        AccordionTrigger { "General Settings" }
        AccordionContent { /* settings form */ }
    }
    
    AccordionItem {
        AccordionTrigger { "Privacy Settings" }
        AccordionContent { /* privacy form */ }
    }
}
```

### Documentation Navigation
```rust
Accordion {
    default_value: None, // All closed initially
    collapsible: true,
    
    AccordionItem {
        AccordionTrigger { "Getting Started" }
        AccordionContent { /* guide content */ }
    }
}
```

---

## 8. Anti-Patterns (PROHIBITED)

### ❌ Anti-Pattern 1: Imperative State Management
```rust
// WRONG — trying to manage state in component
let mut open_item = use_signal(|| "item-1");

Accordion {
    // This violates SSR-safety and CanonRS architecture
    on_change: move |item| open_item.set(item)
}
```

**Why it breaks:**
- State belongs in DOM via `data-state`
- Component layer has no business logic
- Breaks SSR hydration contract

**Correct approach:**  
Let Shell Runtime manage state via data-attributes.

---

### ❌ Anti-Pattern 2: Direct Browser API Usage
```rust
// WRONG — accessing window in UI component
#[component]
pub fn Accordion(...) -> Element {
    use_effect(move |_| {
        window().document().query_selector(...); // FORBIDDEN
    });
}
```

**Why it breaks:**
- UI components must be SSR-safe
- No `window()`, `document()`, or `web_sys` allowed
- Violates layer separation

**Correct approach:**  
Shell Runtime handles all browser APIs.

---

### ❌ Anti-Pattern 3: Inline Styles or Classes
```rust
// WRONG — applying classes or styles directly
rsx! {
    AccordionPrimitive {
        class: "bg-gray-100 rounded-lg", // FORBIDDEN
        style: "padding: 1rem", // FORBIDDEN
    }
}
```

**Why it breaks:**
- Primitives emit data-attributes only
- CSS layer owns all styling
- Breaks token system and theming

**Correct approach:**  
Emit `data-accordion` and let CSS handle styling.

---

### ❌ Anti-Pattern 4: Multiple Accordions Without Context
```rust
// WRONG — nesting accordions without semantic reason
Accordion {
    AccordionItem {
        AccordionTrigger { "Parent" }
        AccordionContent {
            Accordion { // Nested accordion — usually wrong
                AccordionItem { ... }
            }
        }
    }
}
```

**Why it breaks:**
- Cognitive overload
- Confusing navigation
- Violates enterprise UX patterns

**Correct approach:**  
Use flat structure or switch to Tree/Disclosure for nested navigation.

---

## 9. SSR, Hydration, and Runtime

### SSR Impact

**Server-Side:**
- Accordion renders complete HTML structure
- All `data-*` attributes present in initial HTML
- No JavaScript required for structure
- Initial state determined by `default_value` prop

**Benefits:**
- SEO-friendly (content visible to crawlers)
- Fast First Contentful Paint
- Works without JavaScript (graceful degradation)

**Constraints:**
- Cannot use browser APIs during SSR
- Cannot access `window`, `document`, `localStorage`
- Must emit pure HTML

---

### Hydration Process

1. **HTML Delivery:** Browser receives static HTML with `data-accordion` attributes
2. **CSS Application:** Browser applies `accordion.css` styles
3. **Runtime Load:** Shell Runtime JS executes
4. **State Initialization:** Runtime reads `data-default-value`, sets `data-state="open"` on matching item
5. **Event Delegation:** Runtime attaches single click handler to document for `[data-accordion-trigger]`
6. **Interactive:** Component is now fully interactive

**Hydration Contract:**
- Runtime never re-renders HTML
- Runtime only mutates `data-state` attribute
- CSS reacts to state changes

---

### Runtime Global Constraints

**AutoReload Behavior:**
- AutoReload during dev can break script order
- Shell Runtime must be inline in SSR (Canon Rule #103)
- External scripts may load out of order

**Mitigation:**
- Critical runtime JS inlined in `<head>`
- Non-critical behavior degrades gracefully
- Never depend on external script load timing

**Hot Reload:**
- Accordion preserves state across hot reloads
- `data-state` persists in DOM
- No state loss during development

---

## 10. Conformance Checklist

- [x] SSR-safe (no browser APIs in component)
- [x] No imperative JS (no `use_effect`, `window()`, `document()`)
- [x] Uses tokens (indirectly via CSS)
- [x] All tokens documented in section 4
- [x] Anti-patterns documented with explanations
- [x] Canon Rules cited in section 11
- [x] Execution flow documented
- [x] Use cases provided
- [x] Hydration contract explicit
- [x] Runtime constraints documented

---

## 11. Canon Rules Applied

### Canon Rules Applied

- **Canon Rule #102 — Runtime JS Is Shell Infrastructure**  
  Accordion delegates all browser interaction to Shell Runtime, never executing imperative JS in the component layer.

- **Canon Rule #103 — Critical Runtime JS Must Be Inline in SSR**  
  Accordion hydration depends on Shell Runtime being available immediately; inline runtime ensures no race conditions.

- **Canon Rule #104 — AutoReload Breaks Script Order Guarantees**  
  Accordion state persists in DOM (`data-state`) to survive AutoReload script re-execution during development.

- **Canon Rule #107 — Primitives Are SSR-Safe Structural Components**  
  Accordion composes SSR-safe primitives that emit only HTML and data-attributes, never browser-dependent logic.

- **Canon Rule #108 — UI Components Provide Ergonomic Composition**  
  Accordion exists in the UI layer to provide default behavior (`collapsible`, `default_value`) and reduce boilerplate in application code.

- **Canon Rule #109 — State Lives in DOM, Not Memory**  
  Accordion state is declared via `data-state` attribute, making it inspectable, debuggable, and SSR-compatible.

- **Canon Rule #110 — CSS Targets Data-Attributes, Never Classes**  
  Accordion styling uses `[data-accordion-*]` selectors exclusively, ensuring token-based theming and avoiding Tailwind/utility class pollution.

---

**End of Documentation**
