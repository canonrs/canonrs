---
component: AspectRatio
layer: UI
status: Stable
since: v1.0
last_review: 2025-01-18
ownership: canonrs
keywords:
  - design system
  - dioxus
  - ssr
  - aspect ratio
  - media
  - responsive
path_primitive: /opt/docker/monorepo/packages-rust/rs-design/src/primitives/aspect_ratio.rs
path_ui: /opt/docker/monorepo/packages-rust/rs-design/src/ui/aspect_ratio/
---

# AspectRatio

## 1. Conceptual Introduction

The AspectRatio is a **UI component** that enforces consistent geometric proportions for media and data visualization containers. It serves as the canonical enterprise solution for responsive images, videos, charts, maps, and any content requiring fixed aspect ratios.

The AspectRatio exists in the **UI layer** because it provides:
- Ergonomic API with `width` and `height` props (default 16:9)
- Declarative aspect ratio enforcement without layout calculation
- Composition of primitives without business logic
- SSR-safe responsive container

**What it is NOT:**
- Not a primitive (it composes primitives)
- Not a layout component (it enforces proportion, not positioning)
- Not an image component (it's a container)
- Not a runtime behavior (no browser APIs)

---

## 2. Architectural Responsibility (Contract)

### Responsibility

The AspectRatio UI component:
- **Composes** `AspectRatioPrimitive`
- **Declares** geometric proportion via `data-width` and `data-height` attributes
- **Exposes** ergonomic API with `width: f32` and `height: f32` props
- **Emits** SSR-safe HTML with data-attributes for CSS calculation
- **Guarantees** responsive container that maintains aspect ratio

### Non-Responsibility

The AspectRatio UI component explicitly does NOT:
- ❌ Execute browser APIs (resize observers, calculations)
- ❌ Apply inline styles (CSS layer handles aspect-ratio)
- ❌ Manage content loading (image lazy-load is separate concern)
- ❌ Perform layout calculations (CSS does this)
- ❌ Register event listeners

**Side effects are PROHIBITED.**  
AspectRatio is purely declarative—CSS calculates the proportion.

---

## 3. Position in CanonRS Ecosystem

The AspectRatio participates in the canonical CanonRS flow:
```text
Page/Block (usage)
  ↓
UI Component (AspectRatio) — ergonomic API (width, height)
  ↓
Primitive (AspectRatioPrimitive) — HTML + data-attributes
  ↓
SSR Render — static HTML emitted
  ↓
CSS — calculates aspect-ratio from data-width/data-height
  ↓
Browser — renders proportional container
```

**SSR Context:**
- AspectRatio renders complete HTML structure on server
- No client-side JS required for proportion
- Works immediately on page load

**Hydration:**
- AspectRatio requires no hydration (no interactive behavior)
- CSS applies aspect ratio via data-attributes
- Works perfectly without JavaScript

---

## 4. Tokens Applied

The AspectRatio UI component does **not directly apply tokens**—it delegates to CSS.  
The CSS layer (`style/ui/aspect_ratio.css`) consumes the following token families:

### Data & Media Family (F)
AspectRatio belongs exclusively to **Family F — Data & Media**.

### Layout Tokens (Implicit)
- No direct layout token usage
- Inherits spacing from parent containers

### Future Token Support (Optional)
- `--aspect-ratio-16-9` (preset for common ratios)
- `--aspect-ratio-4-3`
- `--aspect-ratio-1-1`

Currently, AspectRatio uses **dynamic calculation** via `data-width` and `data-height`, which is more flexible than preset tokens.

**Token Resolution:**  
UI component emits `data-width` and `data-height` → CSS calculates `aspect-ratio` → Browser renders.

---

## 5. Technical Structure (How It Works)

### SSR Render Phase

The AspectRatio component renders to static HTML:
```html
<div data-aspect-ratio="" data-width="16" data-height="9">
  <img src="banner.jpg" alt="Banner" />
</div>
```

**Key contracts:**
- `data-aspect-ratio` marks container
- `data-width` specifies width proportion
- `data-height` specifies height proportion
- Children positioned absolutely to fill container

### CSS Styling Phase

CSS selectors target data-attributes:
```css
[data-aspect-ratio] {
  position: relative;
  width: 100%;
  aspect-ratio: attr(data-width number) / attr(data-height number);
}

/* Fallback for common ratios */
[data-aspect-ratio][data-width="16"][data-height="9"] {
  aspect-ratio: 16 / 9;
}

[data-aspect-ratio] > * {
  position: absolute;
  inset: 0;
  object-fit: cover;
}
```

**Progressive Enhancement:**
- Modern browsers use `attr(data-width number)` for dynamic calculation
- Older browsers fall back to explicit ratio selectors
- All browsers support absolute positioning of children

**No classes. No inline styles. Only data-attributes.**

---

## 6. Execution Flow
```text
1. SSR Render (Server)
   ↓
   AspectRatio component executes
   ↓
   Emits div with data-width and data-height
   ↓
   Children wrapped inside

2. HTML Delivery (Network)
   ↓
   Browser receives HTML
   ↓
   CSS calculates aspect-ratio

3. CSS Application (Client)
   ↓
   Browser applies aspect_ratio.css
   ↓
   Calculates proportion from data-attributes
   ↓
   Positions children absolutely

4. Final Render
   ↓
   Container maintains aspect ratio
   ↓
   Content fills container proportionally
   ↓
   No JavaScript required
```

**Critical:** AspectRatio is fully functional without JavaScript.  
It's a pure CSS-based layout constraint.

---

## 7. Canonical Use Cases

### Responsive Video Embed (16:9)
```rust
use dioxus::prelude::*;
use rs_design::ui::aspect_ratio::*;

fn VideoEmbed() -> Element {
    rsx! {
        AspectRatio {
            width: 16.0,
            height: 9.0,
            
            iframe {
                src: "https://www.youtube.com/embed/dQw4w9WgXcQ",
                title: "Video",
                allow: "accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture",
            }
        }
    }
}
```

### Square Image (1:1)
```rust
AspectRatio {
    width: 1.0,
    height: 1.0,
    
    img {
        src: "/avatar.jpg",
        alt: "User avatar",
    }
}
```

### Widescreen Banner (21:9)
```rust
AspectRatio {
    width: 21.0,
    height: 9.0,
    
    img {
        src: "/banner.jpg",
        alt: "Hero banner",
    }
}
```

### Classic Photo (4:3)
```rust
AspectRatio {
    width: 4.0,
    height: 3.0,
    
    img {
        src: "/photo.jpg",
        alt: "Landscape photo",
    }
}
```

### Chart Container
```rust
AspectRatio {
    width: 16.0,
    height: 9.0,
    
    canvas { id: "chart-canvas" }
}
```

---

## 8. Anti-Patterns (PROHIBITED)

### ❌ Anti-Pattern 1: Using AspectRatio for Layout
```rust
// WRONG — AspectRatio is for proportion, not layout
AspectRatio {
    width: 16.0,
    height: 9.0,
    
    div {
        class: "flex gap-4", // FORBIDDEN layout styling
        // Multiple layout items
    }
}
```

**Why it breaks:**
- AspectRatio enforces geometric proportion
- Not a layout container (use Grid/Flex)
- Children are positioned absolutely (unsuitable for multi-item layouts)

**Correct approach:**  
Use AspectRatio for single media/chart, use Grid/Stack for layout.

---

### ❌ Anti-Pattern 2: Inline Styles for Aspect Ratio
```rust
// WRONG — inline style instead of props
rsx! {
    div {
        style: "aspect-ratio: 16 / 9", // FORBIDDEN
        img { src: "/image.jpg" }
    }
}
```

**Why it breaks:**
- Bypasses component API
- Not SSR-trackable
- Breaks token system

**Correct approach:**  
Use AspectRatio component with `width` and `height` props.

---

### ❌ Anti-Pattern 3: Nesting AspectRatios
```rust
// WRONG — nesting aspect ratios
AspectRatio {
    width: 16.0,
    height: 9.0,
    
    AspectRatio { // FORBIDDEN nesting
        width: 4.0,
        height: 3.0,
    }
}
```

**Why it breaks:**
- Conflicting proportions
- Absolute positioning breaks inner ratio
- No valid use case

**Correct approach:**  
Single AspectRatio per container.

---

### ❌ Anti-Pattern 4: Using AspectRatio Without Media Content
```rust
// WRONG — AspectRatio for text content
AspectRatio {
    width: 16.0,
    height: 9.0,
    
    p { "This is some text content..." } // FORBIDDEN
}
```

**Why it breaks:**
- Text should flow naturally, not be constrained to fixed ratio
- Accessibility issue (text may overflow)
- AspectRatio is for media/data viz, not prose

**Correct approach:**  
Use AspectRatio only for images, videos, charts, maps, canvases.

---

## 9. SSR, Hydration, and Runtime

### SSR Impact

**Server-Side:**
- AspectRatio renders complete HTML structure
- All `data-*` attributes present in initial HTML
- No JavaScript required for proportion calculation
- CSS handles aspect ratio enforcement

**Benefits:**
- SEO-friendly (content visible to crawlers)
- Instant layout (no layout shift)
- Zero JavaScript bundle cost
- Works with JavaScript disabled

**Constraints:**
- Cannot use browser APIs during SSR
- Must emit pure HTML

---

### Hydration Process

AspectRatio **does not require hydration** because it has no interactive behavior.

1. **HTML Delivery:** Browser receives static HTML
2. **CSS Application:** Browser applies `aspect_ratio.css`
3. **Aspect Ratio Calculation:** CSS calculates proportion from `data-width` / `data-height`
4. **Final Render:** Container maintains aspect ratio

No JavaScript execution needed.

---

### Runtime Global Constraints

**No Runtime Constraints:**
- AspectRatio has no runtime behavior
- No resize observers
- No event listeners
- No state mutations

**AutoReload/Hot Reload:**
- AspectRatio preserves across reloads (static HTML)
- No state to lose

---

## 10. Conformance Checklist

- [x] SSR-safe (no browser APIs)
- [x] No imperative JS (pure CSS-based)
- [x] Uses tokens (indirectly via Family F)
- [x] All tokens documented in section 4
- [x] Anti-patterns documented with explanations
- [x] Canon Rules cited in section 11
- [x] Execution flow documented
- [x] Use cases provided
- [x] Hydration contract explicit (none required)
- [x] Runtime constraints documented (none)
- [x] Belongs to Family F (Data & Media)

---

## 11. Canon Rules Applied

### Canon Rules Applied

- **Canon Rule #107 — Primitives Are SSR-Safe Structural Components**  
  AspectRatio composes SSR-safe primitives that emit only HTML and data-attributes, never browser-dependent logic.

- **Canon Rule #108 — UI Components Provide Ergonomic Composition**  
  AspectRatio exists in the UI layer to provide ergonomic `width` and `height` props with sensible defaults (16:9).

- **Canon Rule #109 — State Lives in DOM, Not Memory**  
  AspectRatio has no state—proportion is declared via `data-width` and `data-height` attributes.

- **Canon Rule #110 — CSS Targets Data-Attributes, Never Classes**  
  AspectRatio styling uses `[data-aspect-ratio]` selectors exclusively, ensuring token-based theming.

- **Canon Rule #112 — Presentational Components Require No Hydration**  
  AspectRatio is fully functional without JavaScript, relying on SSR and CSS alone. No hydration step needed.

- **Canon Rule #113 — AspectRatio Belongs to Family F (Data & Media)**  
  AspectRatio enforces geometric proportion for media and data visualization, never layout or navigation.

---

**End of Documentation**
