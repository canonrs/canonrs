# Canon Rule #75: Primitive CSS Prohibition

**Status:** ENFORCED


**Severity:** CRITICAL
**Version:** 1.0.0
**Date:** 2025-01-16

**Category:** design-system
**Tags:** primitives, css, architecture
**Language:** EN

---

**Intro:**
Adding CSS or visual logic to primitives breaks architectural separation and portability. Primitives must remain pure structural HTML without styling or responsive logic.

**Problem:**
primitives include css and viewport logic violating separation of concerns

**Solution:**
restrict primitives to semantic html and move styling to ui layer

**Signals:**
- tailwind in primitives
- viewport logic in context
- style leakage

**Search Intent:**
why primitives should not contain css

**Keywords:**
primitive vs ui separation, no css in primitives, design system layering, leptos primitive architecture

---

---

## Principle

Primitives provide **semantic HTML structure only**. They MUST NOT contain:
- CSS classes (Tailwind or custom)
- Viewport logic (breakpoints, is_mobile)
- Debug logging
- Visual styling decisions

This separation ensures **portability**, **testability**, and **clean architectural boundaries**.

---

## Forbidden Patterns
```rust
// ❌ NEVER ALLOWED IN PRIMITIVES

// CSS Classes
#[component]
pub fn SidebarPrimitive(children: Children) -> impl IntoView {
    view! {
        <aside 
            class="w-64 transition-all duration-300 overflow-hidden"
            class:w-0=move || !ctx.open.get()
        >
            {children()}
        </aside>
    }
}

// Viewport Logic
pub struct SidebarContext {
    pub is_mobile: bool,  // ❌ NO
    pub breakpoint: String,  // ❌ NO
}

// Debug Logging
impl SidebarContext {
    pub fn toggle(&self) {
        console::log_1(&format!("Toggle called"));  // ❌ NO
        self.open.update(|open| *open = !*open);
    }
}
```

### Why This is Forbidden

1. **CSS in Primitives breaks portability**
   - Tailwind classes tie primitive to specific CSS framework
   - Width/animations are visual decisions, not structure
   - Makes primitive unusable without Tailwind loaded

2. **Viewport logic breaks separation of concerns**
   - Primitives don't know about screens, devices, or breakpoints
   - Layout layer handles responsive behavior
   - Prevents SSR/testing without browser globals

3. **Logging breaks production builds**
   - Debug code leaks into production
   - No feature flag control
   - Violates "primitives are dumb HTML" principle

---

## Canonical Architecture

### Primitive Layer (HTML Only)
```rust
// ✅ CORRECT PRIMITIVE

#[component]
pub fn SidebarPrimitive(children: Children) -> impl IntoView {
    view! {
        <aside data-sidebar="">
            {children()}
        </aside>
    }
}

pub struct SidebarContext {
    pub open: RwSignal<bool>,
    pub variant: SidebarVariant,
}

impl SidebarContext {
    pub fn toggle(&self) {
        self.open.update(|open| *open = !*open);
    }
}
```

**Primitives provide:**
- Semantic HTML tags (`<aside>`, `<nav>`, `<button>`)
- `data-*` attributes for state (`data-open`, `data-active`)
- Structural context (open/closed state)

**Primitives do NOT provide:**
- Visual appearance
- Responsive behavior
- Animation timing
- Debug information

---

### UI Layer (Styling + Tokens)
```rust
// ✅ CORRECT UI COMPONENT

#[component]
pub fn Sidebar(children: Children) -> impl IntoView {
    let ctx = expect_context::<SidebarContext>();
    
    view! {
        <SidebarPrimitive>
            <div 
                class="sidebar-root"
                data-open=move || if ctx.open.get() { "true" } else { "false" }
                style="
                    width: var(--sidebar-width);
                    padding: var(--space-lg);
                    background: var(--color-bg-surface);
                    transition: width var(--motion-duration-normal);
                "
            >
                {children()}
            </div>
        </SidebarPrimitive>
    }
}
```

**UI layer provides:**
- All CSS (classes, inline styles, tokens)
- Responsive wrappers (if needed)
- Animation timing
- Visual state binding (`data-open="true"`)

---

## Enforcement Checklist

### Primitive Files Must Have

- [ ] Only HTML tags
- [ ] Only `data-*` attributes
- [ ] Zero CSS classes
- [ ] Zero inline `style=""`
- [ ] Zero viewport/breakpoint logic
- [ ] Zero console.log or debug output
- [ ] Structural context only (open/closed, active/inactive)

### UI Files Must Have

- [ ] Consumes primitive as base
- [ ] Applies all tokens
- [ ] Handles visual state
- [ ] Manages responsive behavior

---

## State Representation

### ✅ Correct (Primitive)
```rust
// State via data-attribute
<button 
    data-sidebar-menu-button=""
    data-active=is_active
>
```

### ❌ Incorrect (Primitive)
```rust
// Visual styling in primitive
<button 
    class="bg-primary text-white"
    class:bg-gray=!is_active
>
```

---

## Real World Example

### Before (Violates Rule)
```rust
// ❌ PRIMITIVE with CSS
#[component]
pub fn SidebarPrimitive(children: Children) -> impl IntoView {
    let ctx = expect_context::<SidebarContext>();
    
    view! {
        <aside
            class:w-64=move || ctx.open.get()
            class:w-0=move || !ctx.open.get()
            class="transition-all duration-300"
        >
            {children()}
        </aside>
    }
}
```

**Problems:**
- Width decision in primitive
- Tailwind dependency
- Animation timing hardcoded
- No token usage

### After (Compliant)
```rust
// ✅ PRIMITIVE: Structure only
#[component]
pub fn SidebarPrimitive(children: Children) -> impl IntoView {
    view! {
        <aside data-sidebar="">
            {children()}
        </aside>
    }
}

// ✅ UI: Styling with tokens
#[component]
pub fn Sidebar(children: Children) -> impl IntoView {
    let ctx = expect_context::<SidebarContext>();
    
    view! {
        <SidebarPrimitive>
            <div 
                class="sidebar-root"
                data-open=move || if ctx.open.get() { "true" } else { "false" }
            >
                {children()}
            </div>
        </SidebarPrimitive>
    }
}
```

**CSS (separate file):**
```css
.sidebar-root[data-open="true"] {
    width: var(--sidebar-width);
    transition: width var(--motion-duration-normal);
}

.sidebar-root[data-open="false"] {
    width: 0;
}
```

---

## Exceptions

**NONE.**

This rule has no exceptions. Even for:
- "Quick prototypes"
- "Internal components"
- "It's just one class"

If styling is needed, move to UI layer.

---

## Violation Consequences

1. **Immediate:** Component cannot be styled independently
2. **Short-term:** Breaks reusability across projects
3. **Long-term:** Design system becomes unmaintainable
4. **Enterprise:** Cannot pass audit/review

---

## Canonical Justification

Primitives are **contracts**, not implementations.

A primitive that dictates visual appearance:
- Cannot be themed
- Cannot be customized
- Cannot be tested without DOM
- Cannot be ported to non-Tailwind projects

**Canon mandates:** Structure and style are orthogonal concerns.

---

## Canon References

- Canon Rule #74 — Block Semantic HTML
- Canon Rule #76 — Navigation vs Action Contract
- Canon Rule #81 — Flex Layout Ownership

---

## Related Symptoms

If you see:
- Tailwind classes in `primitives/` folder
- `is_mobile` or `breakpoint` in context
- `console.log` in primitive toggle functions

→ **This rule is violated.**

Go to: **SYMPTOMS.md → PRIMITIVE VIOLATIONS**