---
component: Avatar
layer: UI
status: Stable
since: v1.0
last_review: 2025-01-18
ownership: canonrs
keywords:
  - design system
  - dioxus
  - ssr
  - avatar
  - profile
  - user identity
  - media
path_primitive: /opt/docker/monorepo/packages-rust/rs-design/src/primitives/avatar.rs
path_ui: /opt/docker/monorepo/packages-rust/rs-design/src/ui/avatar/
---

# Avatar

## 1. Conceptual Introduction

The Avatar is a **UI component** that displays user or entity identity through profile images or fallback initials. It serves as the canonical enterprise solution for user profiles, comment systems, identity chips, team rosters, and any interface requiring visual representation of people or entities.

The Avatar exists in the **UI layer** because it provides:
- Size variants (xs, sm, md, lg, xl) for different contexts
- Ergonomic composition of image and fallback states
- Semantic identity representation
- Composition of primitives without business logic

**What it is NOT:**
- Not a primitive (it composes primitives)
- Not a button (use AvatarButton if clickable)
- Not a badge (use Badge for status indicators)
- Not a layout component (it's a single identity representation)

---

## 2. Architectural Responsibility (Contract)

### Responsibility

The Avatar UI component:
- **Composes** `AvatarPrimitive`, `AvatarImage`, and `AvatarFallback`
- **Declares** size via `data-size` attribute (xs, sm, md, lg, xl)
- **Exposes** ergonomic API with `size` prop (default: `md`)
- **Emits** SSR-safe HTML with data-attributes for styling
- **Guarantees** circular container with image or fallback display

### Non-Responsibility

The Avatar UI component explicitly does NOT:
- ❌ Execute browser APIs (image loading, error handling beyond CSS)
- ❌ Manage online/offline status (use separate StatusIndicator)
- ❌ Apply CSS classes or inline styles
- ❌ Register event listeners (no click behavior)
- ❌ Perform side effects or mutations

**Side effects are PROHIBITED.**  
Avatar is purely presentational—image loading is native browser behavior.

---

## 3. Position in CanonRS Ecosystem

The Avatar participates in the canonical CanonRS flow:
```text
Page/Block (usage)
  ↓
UI Component (Avatar) — size variants, composition
  ↓
Primitives (AvatarPrimitive, AvatarImage, AvatarFallback) — HTML + data-attributes
  ↓
SSR Render — static HTML emitted
  ↓
CSS — styling via [data-avatar-*] selectors
  ↓
Browser — renders avatar (image load handled natively)
```

**SSR Context:**
- Avatar renders complete HTML structure on server
- Image `src` present in initial HTML (browser handles loading)
- Fallback visible until image loads (native `<img>` behavior)

**Hydration:**
- Avatar requires no hydration (no interactive behavior)
- CSS applies styling via data-attributes
- Image loading is native browser behavior

---

## 4. Tokens Applied

The Avatar UI component does **not directly apply tokens**—it delegates to CSS.  
The CSS layer (`style/ui/avatar.css`) consumes the following token families:

### Data & Media Family (F)
Avatar belongs to **Family F — Data & Media** (identity representation).

### Layout Tokens
- `--space-md` (implicit: size tokens derive from spacing scale)

### Typography Tokens
- `--font-family-sans` (fallback text)
- `--font-size-xs` (xs, sm sizes)
- `--font-size-sm` (md size)
- `--font-size-md` (lg, xl sizes)
- `--font-weight-medium` (fallback text weight)
- `--line-height-tight` (fallback text line height)

### Color Tokens
- `--color-bg-muted` (background, fallback background)
- `--color-fg-muted` (fallback text)
- `--color-border-muted` (border)
- `--color-bg-surface` (outlined variant background)
- `--color-border-default` (outlined variant border)

### Border & Radius Tokens
- `--border-width-hairline` (border)
- `--radius-lg` (circular: 50% effective, but token consistency)

### State Tokens
- `--state-disabled-opacity` (disabled state)

### Avatar Family Tokens (Defined in CSS)
- `--avatar-size` (size per variant: 1.5rem to 4rem)
- `--avatar-fallback-font-size` (scales with size)

**Token Resolution:**  
UI component emits `data-size` → CSS applies size tokens → Browser renders.

---

## 5. Technical Structure (How It Works)

### SSR Render Phase

The Avatar component renders to static HTML:
```html
<span data-avatar="" data-size="md">
  <img data-avatar-image="" src="/avatar.jpg" alt="John Doe" />
  <span data-avatar-fallback="">JD</span>
</span>
```

**Key contracts:**
- `data-avatar` marks root container (circular, overflow hidden)
- `data-size="xs|sm|md|lg|xl"` determines dimensions
- `data-avatar-image` marks profile image
- `data-avatar-fallback` marks fallback initials (shown if image fails to load)

### CSS Styling Phase

CSS selectors target data-attributes:
```css
[data-avatar] {
  width: var(--avatar-size, 2.5rem);
  height: var(--avatar-size, 2.5rem);
  border-radius: var(--radius-lg); /* Effectively circular */
  overflow: hidden;
}

[data-avatar][data-size="md"] {
  --avatar-size: 2.5rem;
  --avatar-fallback-font-size: var(--font-size-sm);
}

[data-avatar-image] {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

[data-avatar-fallback] {
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: var(--avatar-fallback-font-size);
  color: var(--color-fg-muted);
}
```

**Image Loading Behavior:**
- Browser natively handles image loading
- If image loads successfully: image visible, fallback hidden
- If image fails: image hidden, fallback visible (native `<img>` error behavior + CSS)

**No classes. No inline styles. Only data-attributes.**

---

## 6. Execution Flow
```text
1. SSR Render (Server)
   ↓
   Avatar component executes
   ↓
   Emits span with data-avatar, data-size
   ↓
   Includes img (with src) and fallback span

2. HTML Delivery (Network)
   ↓
   Browser receives HTML
   ↓
   Fallback visible initially

3. CSS Application (Client)
   ↓
   Browser applies avatar.css
   ↓
   Circular container with size applied

4. Image Load (Native Browser)
   ↓
   Browser requests image from src
   ↓
   If successful: image visible, fallback hidden
   ↓
   If failed: fallback remains visible

5. Final Render
   ↓
   User sees avatar image OR fallback initials
   ↓
   No JavaScript required
```

**Critical:** Avatar is fully functional without JavaScript.  
Image loading and fallback are native browser behaviors.

---

## 7. Canonical Use Cases

### User Profile Avatar (Medium Size)
```rust
use dioxus::prelude::*;
use rs_design::ui::avatar::*;

fn UserAvatar() -> Element {
    rsx! {
        Avatar {
            size: AvatarSize::Md,
            
            AvatarImage {
                src: "/users/johndoe.jpg".to_string(),
                alt: "John Doe".to_string(),
            }
            AvatarFallback { "JD" }
        }
    }
}
```

### Small Avatar in Comment Thread
```rust
Avatar {
    size: AvatarSize::Sm,
    
    AvatarImage {
        src: "/users/jane.jpg".to_string(),
        alt: "Jane Smith".to_string(),
    }
    AvatarFallback { "JS" }
}
```

### Large Avatar in Profile Header
```rust
Avatar {
    size: AvatarSize::Lg,
    
    AvatarImage {
        src: "/users/profile.jpg".to_string(),
        alt: "User Profile".to_string(),
    }
    AvatarFallback { "UP" }
}
```

### Avatar Without Image (Fallback Only)
```rust
Avatar {
    size: AvatarSize::Md,
    
    AvatarFallback { "AB" }
}
```

### Extra Small Avatar in Roster
```rust
Avatar {
    size: AvatarSize::Xs,
    
    AvatarImage {
        src: "/team/member.jpg".to_string(),
        alt: "Team Member".to_string(),
    }
    AvatarFallback { "TM" }
}
```

---

## 8. Anti-Patterns (PROHIBITED)

### ❌ Anti-Pattern 1: Using Avatar as a Button
```rust
// WRONG — Avatar has no click behavior
Avatar {
    size: AvatarSize::Md,
    onclick: |_| { /* ... */ }, // FORBIDDEN
    
    AvatarImage { /* ... */ }
}
```

**Why it breaks:**
- Avatar is presentational only
- No interactive behavior built-in
- Not focusable or clickable

**Correct approach:**  
Wrap Avatar in a `<button>` or create `AvatarButton` component.

---

### ❌ Anti-Pattern 2: Inline Styles for Size
```rust
// WRONG — inline style instead of size prop
rsx! {
    span {
        style: "width: 3rem; height: 3rem", // FORBIDDEN
        img { src: "/avatar.jpg" }
    }
}
```

**Why it breaks:**
- Bypasses component API
- Not SSR-trackable
- Breaks token system

**Correct approach:**  
Use Avatar component with `size` prop.

---

### ❌ Anti-Pattern 3: Using Avatar for Logos or Icons
```rust
// WRONG — Avatar is for user/entity identity, not logos
Avatar {
    size: AvatarSize::Md,
    
    AvatarImage {
        src: "/logo.svg".to_string(),
        alt: "Company Logo".to_string(),
    }
}
```

**Why it breaks:**
- Avatar implies user/entity identity
- Logos are not user avatars
- Semantic mismatch

**Correct approach:**  
Use `<img>` directly or create `Logo` component.

---

### ❌ Anti-Pattern 4: Nesting Status Indicators Inside Avatar
```rust
// WRONG — status indicator should be sibling, not child
Avatar {
    AvatarImage { /* ... */ }
    AvatarFallback { "JD" }
    
    div { class: "status-dot", /* ... */ } // FORBIDDEN nesting
}
```

**Why it breaks:**
- Avatar has `overflow: hidden` (hides positioned elements)
- Status should be absolutely positioned relative to parent container
- Architectural confusion

**Correct approach:**  
Wrap Avatar + StatusIndicator in a positioned container:
```rust
div { style: "position: relative",
    Avatar { /* ... */ }
    StatusIndicator { /* positioned absolutely */ }
}
```

---

## 9. SSR, Hydration, and Runtime

### SSR Impact

**Server-Side:**
- Avatar renders complete HTML structure
- Image `src` present in initial HTML
- Fallback text rendered
- No JavaScript required for display

**Benefits:**
- SEO-friendly (alt text visible to crawlers)
- Instant display (no layout shift)
- Zero JavaScript bundle cost
- Works with JavaScript disabled

**Constraints:**
- Cannot use browser APIs during SSR
- Must emit pure HTML

---

### Hydration Process

Avatar **does not require hydration** because it has no interactive behavior.

1. **HTML Delivery:** Browser receives static HTML
2. **CSS Application:** Browser applies `avatar.css` styles
3. **Image Load:** Browser natively loads image from `src`
4. **Final Render:** Avatar displays image or fallback

No JavaScript execution needed.

---

### Runtime Global Constraints

**No Runtime Constraints:**
- Avatar has no runtime behavior
- No event listeners
- No state mutations
- Image loading is native browser behavior

**AutoReload/Hot Reload:**
- Avatar preserves across reloads (static HTML)
- No state to lose

---

## 10. Conformance Checklist

- [x] SSR-safe (no browser APIs)
- [x] No imperative JS (pure presentational)
- [x] Uses tokens (indirectly via CSS)
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
  Avatar composes SSR-safe primitives that emit only HTML and data-attributes, never browser-dependent logic.

- **Canon Rule #108 — UI Components Provide Ergonomic Composition**  
  Avatar exists in the UI layer to provide size variants (`xs`, `sm`, `md`, `lg`, `xl`) and reduce boilerplate in application code.

- **Canon Rule #109 — State Lives in DOM, Not Memory**  
  Avatar has no state—size is declared via `data-size` attribute. Image load state is managed by native browser behavior.

- **Canon Rule #110 — CSS Targets Data-Attributes, Never Classes**  
  Avatar styling uses `[data-avatar-*]` selectors exclusively, ensuring token-based theming and avoiding utility class pollution.

- **Canon Rule #112 — Presentational Components Require No Hydration**  
  Avatar is fully functional without JavaScript, relying on SSR, CSS, and native image loading. No hydration step needed.

- **Canon Rule #113 — Avatar Belongs to Family F (Data & Media)**  
  Avatar represents user/entity identity through media (images) and data (fallback initials), never layout or navigation.

---

**End of Documentation**
