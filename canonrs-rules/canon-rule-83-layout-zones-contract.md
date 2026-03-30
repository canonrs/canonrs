# Canon Rule #83: Layout Zones Contract


**Status:** ENFORCED
**Severity:** CRITICAL
**Version:** 1.0.0
**Date:** 2025-01-16

**Category:** component-architecture
**Tags:** layout, zones, hierarchy
**Language:** EN

---

**Intro:**
Unstructured layouts lead to inconsistent UI hierarchy and unpredictable responsive behavior. Canon enforces fixed spatial zones to standardize component placement and eliminate layout ambiguity.

**Problem:**
components are placed without spatial rules causing layout inconsistency

**Solution:**
enforce five canonical layout zones with strict nesting and responsibilities

**Signals:**
- layout chaos
- content outside container
- z-index conflicts

**Search Intent:**
how to structure layout zones in ui architecture

**Keywords:**
layout zones architecture, ui spatial hierarchy system, canonical layout containers, component placement rules

---

## The Principle

**Every element knows where it belongs.**

Canon defines **5 spatial zones** that control WHERE components can exist. This prevents layout chaos, ensures consistent spatial hierarchy, and makes responsive behavior predictable.

---

## The 5 Layout Zones
```
┌─────────────────────────────────────────────────────────┐
│ OVERLAY (z-index layers)                                │
├─────────────────────────────────────────────────────────┤
│ ┌─────────┬─────────────────────────────────────────┐  │
│ │         │ PAGE (application container)            │  │
│ │         │ ┌───────────────────────────────────┐   │  │
│ │ PANEL   │ │ RAIL (content width container)    │   │  │
│ │         │ │ ┌─────────────────────────────┐   │   │  │
│ │ (side   │ │ │ CONTENT (actual information)│   │   │  │
│ │  bar)   │ │ │                             │   │   │  │
│ │         │ │ └─────────────────────────────┘   │   │  │
│ └─────────┴─┴───────────────────────────────────┴───┴──┘
└─────────────────────────────────────────────────────────┘
```

---

## Zone 1 PAGE

**Purpose:** Application-level container  
**Token:** `--color-bg-surface`  
**Max instances:** 1 per route

### Characteristics
- Full viewport width/height
- Background for entire application
- Sets base surface color
- Contains all other zones

### CSS Contract
```css
.canon-page {
  display: flex;
  justify-content: center;
  width: 100%;
  min-height: 100vh;
  background: var(--color-bg-surface);
}
```

### What Lives Here
- ✅ RAIL zone
- ✅ PANEL zone (sidebar)
- ❌ NEVER: Direct content
- ❌ NEVER: Components

### Example: AppLayout
```rust
view! {
    <div class="canon-page">
        <Sidebar />  // PANEL zone
        <main class="canon-rail">
            // RAIL zone content
        </main>
    </div>
}
```

---

## Zone 2 PANEL

**Purpose:** Persistent navigation/tooling  
**Token:** `--color-bg-muted`  
**Max instances:** 2 (left + right)

### Characteristics
- Vertical slice
- Fixed or collapsible width
- Muted background (distinct from content)
- Independent scroll

### CSS Contract
```css
.canon-panel {
  width: var(--sidebar-width); /* 16rem default */
  height: 100vh;
  background: var(--color-bg-muted);
  border-right: var(--border-width-hairline) solid var(--color-border-default);
  overflow-y: auto;
}
```

### What Lives Here
- ✅ Navigation menus
- ✅ Tool palettes
- ✅ Filters
- ✅ Inspector panels
- ❌ NEVER: Primary content
- ❌ NEVER: Forms (use CONTENT)

### Positioning Rules
| Side | Use Case |
|------|----------|
| Left | Primary navigation |
| Right | Context panels (inspector, properties) |

**Golden Rule:** If it's persistent across routes → PANEL

---

## Zone 3 RAIL

**Purpose:** Content width constraint  
**Token:** N/A (structural only)  
**Max width:** `72rem` (1152px)

### Characteristics
- Centers content horizontally
- Provides reading-optimal width
- Responsive padding
- Contains CONTENT zone

### CSS Contract
```css
.canon-rail {
  width: 100%;
  max-width: 72rem; /* 1152px */
  padding: var(--space-xl);
  display: flex;
  flex-direction: column;
  gap: var(--space-xl);
}
```

### What Lives Here
- ✅ CONTENT zone
- ✅ Section spacing
- ✅ Vertical rhythm
- ❌ NEVER: Direct text/components
- ❌ NEVER: Fixed width content

### Responsive Behavior
```css
@media (max-width: 768px) {
  .canon-rail {
    padding: var(--space-md);
  }
}
```

**Golden Rule:** If it needs reading width → RAIL

---

## Zone 4 CONTENT

**Purpose:** Actual information display  
**Token:** `--color-bg-elevated` (when highlighted)  
**Max instances:** Unlimited

### Characteristics
- Information-bearing
- Can be elevated surface
- Semantic HTML
- Accessible structure

### CSS Contract (Elevated)
```css
.canon-content-elevated {
  background: var(--color-bg-elevated);
  border: var(--border-width-hairline) solid var(--color-border-muted);
  border-radius: var(--radius-lg);
  box-shadow: var(--shadow-sm);
  padding: var(--space-xl);
}
```

### CSS Contract (Flat)
```css
.canon-content-flat {
  background: transparent;
  /* Inherits PAGE surface */
}
```

### What Lives Here
- ✅ Component previews
- ✅ Documentation blocks
- ✅ Tables
- ✅ Forms
- ✅ Cards
- ✅ API docs
- ✅ Text content

### Content Types

| Type | Surface | Border | Shadow |
|------|---------|--------|--------|
| Hero | `base` | No | No |
| Preview | `elevated` | Yes | Yes |
| API Block | `elevated` | Yes | Yes |
| Paragraph | `base` | No | No |

**Golden Rule:** If users read/interact with it → CONTENT

---

## Zone 5 OVERLAY

**Purpose:** Temporary UI above main flow  
**Token:** `--z-*` (z-index hierarchy)  
**Max instances:** Stacked (modal > dropdown > tooltip)

### Characteristics
- Positioned above PAGE
- Uses z-index system
- Temporary visibility
- Backdrop when modal

### Z-Index Hierarchy
```css
:root {
  --z-base: 0;          /* PAGE/RAIL/PANEL */
  --z-popover: 1000;    /* Dropdowns, popovers */
  --z-dropdown: 2000;   /* Context menus */
  --z-modal: 3000;      /* Dialogs */
  --z-toast: 4000;      /* Notifications */
  --z-tooltip: 5000;    /* Tooltips */
}
```

### CSS Contract (Modal)
```css
.canon-overlay-modal {
  position: fixed;
  inset: 0;
  z-index: var(--z-modal);
  display: flex;
  align-items: center;
  justify-content: center;
  background: hsl(0 0% 0% / 0.5); /* Backdrop */
}

.canon-overlay-content {
  background: var(--color-bg-elevated);
  border-radius: var(--radius-xl);
  box-shadow: var(--shadow-2xl);
  max-width: 32rem;
  padding: var(--space-2xl);
}
```

### What Lives Here
- ✅ Modals
- ✅ Dropdowns
- ✅ Tooltips
- ✅ Command palette
- ✅ Toast notifications
- ❌ NEVER: Persistent UI
- ❌ NEVER: Primary navigation

**Golden Rule:** If it blocks interaction temporarily → OVERLAY

---

## Zone Nesting Rules

### ✅ Valid Nesting
```
PAGE
└── PANEL (sidebar)
└── RAIL
    └── CONTENT (elevated)
    └── CONTENT (flat)
```

### ❌ Invalid Nesting
```
RAIL
└── PAGE  /* WRONG - PAGE must be root */

CONTENT
└── RAIL  /* WRONG - RAIL contains CONTENT, not reverse */

PANEL
└── RAIL  /* WRONG - Both are siblings under PAGE */
```

---

## Component Zone Mapping

| Component | Zone | Surface |
|-----------|------|---------|
| AppLayout | PAGE | `base` |
| Sidebar | PANEL | `muted` |
| ComponentHeader | CONTENT (flat) | `base` |
| PreviewBlock | CONTENT (elevated) | `elevated` |
| ApiBlock | CONTENT (elevated) | `elevated` |
| ComparisonBlock | CONTENT (elevated) | `elevated` |
| RulesBlock | CONTENT (elevated) | `elevated` |
| Modal | OVERLAY | `elevated` |
| Dropdown | OVERLAY | `elevated` |
| Toast | OVERLAY | `elevated` |

---

## Real World Example

### Structure
```rust
// PAGE zone
<div class="canon-page">
    
    // PANEL zone (sidebar)
    <Sidebar />
    
    // RAIL zone
    <main class="canon-rail">
        
        // CONTENT zone (hero - flat)
        <ComponentHeader
            name="Button"
            description="Primary interactive element"
        />
        
        // CONTENT zone (elevated)
        <PreviewBlock>
            <ButtonShowcase />
        </PreviewBlock>
        
        // CONTENT zone (elevated)
        <ApiBlock props=api_props />
        
        // CONTENT zone (elevated)
        <ComparisonBlock rows=comparison />
        
    </main>
</div>
```

### CSS Mapping
```css
/* PAGE */
.canon-page {
  background: var(--color-bg-surface);
}

/* PANEL */
.sidebar {
  background: var(--color-bg-muted);
}

/* RAIL */
.canon-rail {
  max-width: 72rem;
  padding: var(--space-xl);
}

/* CONTENT (flat) */
.canon-component-hero {
  background: var(--color-bg-surface); /* Inherits PAGE */
}

/* CONTENT (elevated) */
.canon-preview-surface {
  background: var(--color-bg-elevated);
  border: 1px solid var(--color-border-muted);
  box-shadow: var(--shadow-xl);
}
```

---

## Responsive Behavior

### PANEL Collapse
```css
@media (max-width: 1024px) {
  .canon-panel {
    position: fixed;
    transform: translateX(-100%);
    transition: transform var(--motion-duration-normal);
  }
  
  .canon-panel[data-open="true"] {
    transform: translateX(0);
  }
}
```

### RAIL Padding
```css
@media (max-width: 768px) {
  .canon-rail {
    padding: var(--space-md);
    max-width: 100%;
  }
}
```

### CONTENT Stacking
```css
@media (max-width: 640px) {
  .canon-content-elevated {
    border-radius: var(--radius-md); /* Smaller radius on mobile */
    padding: var(--space-lg);
  }
}
```

---

## Anti Patterns

### ❌ Content Outside RAIL
```rust
// WRONG
<div class="canon-page">
    <h1>Title</h1>  /* Floating content - no RAIL */
</div>
```

**Fix:** Wrap in RAIL.

### ❌ RAIL Inside CONTENT
```rust
// WRONG
<PreviewBlock>
    <div class="canon-rail">  /* RAIL nested in CONTENT */
        ...
    </div>
</PreviewBlock>
```

**Fix:** RAIL contains CONTENT, not reverse.

### ❌ Hardcoded Widths in CONTENT
```css
/* WRONG */
.my-content {
  max-width: 800px; /* Hardcoded */
}
```

**Fix:** Let RAIL control width, CONTENT fills available space.

### ❌ Z-Index Without Semantic Token
```css
/* WRONG */
.my-modal {
  z-index: 9999; /* Arbitrary */
}
```

**Fix:** Use `var(--z-modal)`.

### ❌ Multiple RAILs
```rust
// WRONG
<div class="canon-page">
    <div class="canon-rail">Content 1</div>
    <div class="canon-rail">Content 2</div>  /* Duplicate RAIL */
</div>
```

**Fix:** One RAIL per PAGE. Multiple CONTENT sections inside RAIL.

---

## Benefits

### ✅ Predictable Layout
- Every component knows its zone
- No ambiguity about placement
- Consistent spatial hierarchy

### ✅ Responsive by Default
- PANEL collapses
- RAIL adapts padding
- CONTENT reflows

### ✅ Accessible Structure
```html
<div role="application" class="canon-page">
  <nav role="navigation" class="canon-panel">...</nav>
  <main role="main" class="canon-rail">...</main>
</div>
```

### ✅ Prevents Layout Bugs
- Can't nest zones incorrectly (linter enforces)
- Can't create rogue widths
- Can't break z-index hierarchy

---

## Validation Checklist

### Zone Audit
```bash
# Check for content outside RAIL
grep -r "canon-page" src/ | grep -v "canon-rail"

# Check for RAIL inside CONTENT
grep -r "canon-content" src/ | grep "canon-rail"

# Check for arbitrary z-index
grep -r "z-index: [0-9]" src/ | grep -v "var(--z-"
```

### Accessibility Test
```javascript
// Verify semantic structure
document.querySelector('[role="main"]') // Should be RAIL
document.querySelector('[role="navigation"]') // Should be PANEL
```

---

## System Status After Rule 83

| Aspect | Status |
|--------|--------|
| Spatial Zones | 🟢 DEFINED |
| Nesting Rules | 🟢 ENFORCED |
| Responsive Behavior | 🟢 PREDICTABLE |
| Z-Index Hierarchy | 🟢 CLOSED |
| Layout Chaos | 🟢 IMPOSSIBLE |

---

## Related Rules

- **Rule #82:** Visual Surfaces Contract
- **Rule #81:** Token Architecture & Theme Specificity
- **Rule #50:** Provider Singleton Pattern

---

## Normative Requirements

**MUST:**
- Every route MUST have exactly one PAGE zone
- RAIL MUST be direct child of PAGE (or sibling of PANEL)
- CONTENT MUST be inside RAIL
- OVERLAY MUST use semantic z-index tokens
- Zones MUST NOT be nested incorrectly

**MUST NOT:**
- Create content outside RAIL
- Hardcode widths in CONTENT
- Use arbitrary z-index values
- Nest RAIL inside CONTENT
- Create multiple RAIL zones per PAGE

**SHOULD:**
- Use PANEL for persistent navigation
- Collapse PANEL on mobile
- Use semantic roles (main, nav)

---

**Author:** Canon Working Group  
**Supersedes:** Implicit layout patterns  
**Related:** Canon Rule #82 (Visual Surfaces)