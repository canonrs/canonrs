# Canon Rule #81: Flex Layout Ownership

**Status:** ENFORCED
**Severity:** HIGH
**Scope:** ui, layout, design-system
**Version:** 1.0.0
**Date:** 2026-01-12

---

## Principle

In flex container hierarchies, **child growth is determined by the parent**, not the child itself.

A component that needs to grow (`flex: 1`) MUST have this property applied **in its immediate parent context**, not internally.

**Critical distinction:**
- Primitives provide structure
- UI wrappers provide flex behavior
- Layout components orchestrate space distribution

---

## Forbidden Patterns
```rust
// ❌ NEVER ALLOWED

// Primitive trying to grow itself
#[component]
pub fn SidebarInsetPrimitive(children: Children) -> impl IntoView {
    view! {
        <div data-sidebar-inset="" class="flex-1">  // ❌ Primitive has flex
            {children()}
        </div>
    }
}

// UI component expecting parent to know its needs
#[component]
pub fn SidebarInset(children: ChildrenFn) -> impl IntoView {
    view! {
        <SidebarInsetPrimitive>
            <main>{children()}</main>  // ❌ No flex wrapper in UI
        </SidebarInsetPrimitive>
    }
}
```

### Why This is Forbidden

1. **Violates Separation of Concerns**
   - Primitive decides its own layout behavior
   - Breaks "dumb HTML" contract
   - Cannot be reused in non-flex contexts

2. **Creates Invisible Dependencies**
   - Parent must be flex container (not documented)
   - Component doesn't work if parent changes
   - Debugging requires understanding CSS cascade

3. **Breaks Composability**
   - Cannot wrap in additional containers
   - Cannot conditionally apply flex
   - Hard to override in specific contexts

---

## Canonical Architecture

### Three-Layer Flex Contract
```
┌─────────────────────────────────────┐
│ Parent (Flex Container)              │
│ display: flex;                       │
│                                      │
│  ┌────────────────────────────────┐ │
│  │ UI Wrapper (Flex Child)        │ │
│  │ flex: 1;  ← OWNS GROWTH        │ │
│  │                                │ │
│  │  ┌──────────────────────────┐ │ │
│  │  │ Primitive (Structure)    │ │ │
│  │  │ No flex properties       │ │ │
│  │  └──────────────────────────┘ │ │
│  └────────────────────────────────┘ │
└─────────────────────────────────────┘
```

**Rule:** Each layer has ONE responsibility.

---

## Correct Implementation

### Primitive (No Flex)
```rust
// ✅ CORRECT: Primitive is pure structure
#[component]
pub fn SidebarInsetPrimitive(children: Children) -> impl IntoView {
    view! {
        <div data-sidebar-inset="">
            {children()}
        </div>
    }
}
```

### UI Wrapper (Adds Flex)
```rust
// ✅ CORRECT: UI layer adds flex behavior
#[component]
pub fn SidebarInset(
    children: ChildrenFn,
    #[prop(default = String::new(), into)] class: String,
) -> impl IntoView {
    view! {
        <div class="flex-1">  // ← UI layer owns flex
            <SidebarInsetPrimitive>
                <main class=format!("flex-1 {}", class)>
                    {children()}
                </main>
            </SidebarInsetPrimitive>
        </div>
    }
}
```

### Layout Context (Flex Container)
```rust
// ✅ CORRECT: Parent establishes flex context
#[component]
pub fn SidebarProvider(children: Children) -> impl IntoView {
    view! {
        <SidebarProviderPrimitive>
            <div class="flex min-h-svh w-full">  // ← Flex container
                <Sidebar/>  // Child 1
                <SidebarInset/>  // Child 2 (will grow via flex-1)
            </div>
        </SidebarProviderPrimitive>
    }
}
```

---

## Real World Problem

### Symptom
```
Viewport: 1493px
Sidebar: 256px
Expected Inset: 1237px (1493 - 256)
Actual Inset: 521px ❌
```

### Root Cause
```javascript
const inset = document.querySelector('[data-sidebar-inset]');
getComputedStyle(inset).flex; // "0 1 auto" ❌
```

`flex: 0 1 auto` means:
- `flex-grow: 0` → **Does not grow**
- `flex-shrink: 1` → Can shrink
- `flex-basis: auto` → Use content size

**Result:** Inset stays at minimum content width (521px).

### Solution
```rust
// Add flex-1 wrapper in UI layer
<div class="flex-1">  // ← This makes flex: 1 1 0%
    <SidebarInsetPrimitive>
        ...
    </SidebarInsetPrimitive>
</div>
```

**Result:** Inset grows to 1237px ✓

---

## Flex Property Decision Tree
```
Is this a primitive?
│
├─ YES → NO flex properties allowed
│         (Not even flex-1)
│
└─ NO → Is this a UI component?
         │
         ├─ YES → Should it grow in flex container?
         │         │
         │         ├─ YES → Add <div class="flex-1"> wrapper
         │         └─ NO → No flex needed
         │
         └─ NO → Is this a layout component?
                  │
                  └─ YES → Use display: flex on container
                           Let children decide growth via UI wrappers
```

---

## Common Flex Mistakes

### Mistake 1: Flex in Primitive
```rust
// ❌ WRONG
#[component]
pub fn CardPrimitive(children: Children) -> impl IntoView {
    view! {
        <div class="flex-1">  // ❌ Primitive assumes flex parent
            {children()}
        </div>
    }
}
```

**Problem:** Breaks in non-flex contexts.

### Mistake 2: No Wrapper in UI
```rust
// ❌ WRONG
#[component]
pub fn Card(children: Children) -> impl IntoView {
    view! {
        <CardPrimitive>  // ❌ No flex wrapper
            {children()}
        </CardPrimitive>
    }
}
```

**Problem:** Card won't grow even in flex parent.

### Mistake 3: Conflicting Flex Values
```rust
// ❌ WRONG
<div class="flex-1" style="flex: 0 0 auto;">  // ❌ Inline overrides class
    <Content/>
</div>
```

**Problem:** Inline styles win, class ignored.

---

## Enforcement Checklist

### Primitive Files

- [ ] Zero `flex` properties
- [ ] Zero `flex-1` classes
- [ ] Zero `flex-grow/shrink/basis`
- [ ] Only structural HTML

### UI Files

- [ ] Flex behavior added via wrapper
- [ ] Wrapper is immediate child of primitive
- [ ] No conflicting inline styles
- [ ] Growth documented in component docs

### Layout Files

- [ ] Parent uses `display: flex`
- [ ] Children growth controlled via UI wrappers
- [ ] Flex direction documented
- [ ] Gap/spacing uses tokens

---

## Testing Flex Layout
```javascript
// ✅ Verify flex setup
const parent = document.querySelector('.flex-container');
const child = document.querySelector('[data-sidebar-inset]');
const wrapper = child.querySelector('.flex-1');

console.assert(
    getComputedStyle(parent).display === 'flex',
    'Parent must be flex container'
);

console.assert(
    getComputedStyle(wrapper).flex === '1 1 0%',
    'Wrapper must have flex: 1'
);

console.assert(
    getComputedStyle(child).flex === '0 1 auto',
    'Primitive should NOT have flex: 1'
);
```

---

## Flex Math Reference
```css
/* flex: [grow] [shrink] [basis] */

flex: 0 1 auto;  /* Default - don't grow, can shrink, auto size */
flex: 1;         /* Shorthand for: 1 1 0% */
flex: 1 1 0%;    /* Grow to fill, shrink if needed, start at 0 */
flex: 0 0 auto;  /* Fixed size, no grow/shrink */
```

**Canon preference:** Use `class="flex-1"` (Tailwind) instead of inline `flex: 1`.

---

## Architecture Diagram
```
SidebarProvider (Layout)
└─ <div class="flex">  ← Flex container
   ├─ Sidebar
   │  └─ SidebarPrimitive
   │     └─ <div class="sidebar-root">  ← Fixed width
   │
   └─ SidebarInset (UI)
      └─ <div class="flex-1">  ← GROWS ✓
         └─ SidebarInsetPrimitive
            └─ <div data-sidebar-inset>  ← Structure only
               └─ <main>
                  └─ Content
```

**Key insight:** `flex-1` lives **between** UI component and primitive.

---

## Exceptions

**NONE for primitives.**

Primitives NEVER have flex properties, even if:
- "It's always used in flex containers"
- "It makes the code shorter"
- "Everyone does it this way"

**Conditional for UI:**
- If component needs to grow → Add flex wrapper
- If component is fixed size → No flex needed

---

## Violation Consequences

1. **Immediate:** Layout breaks in production
2. **Short-term:** Impossible to debug without DevTools
3. **Long-term:** Cannot reuse components in different contexts
4. **Enterprise:** Fails responsive design audit

---

## Canonical Justification

Flex is a **parent-child contract**.

A child that declares its own flex behavior:
- Assumes knowledge of parent context
- Breaks when parent changes
- Cannot be composed flexibly

**Canon mandates:** Explicit flex ownership in UI layer.

---

## Canon References

- Canon Rule #75 — Primitive CSS Prohibition
- Canon Rule #79 — ComponentPage Template Contract
- Canon Rule #1 — Types (Primitive vs UI separation)

---

## Related Symptoms

If you see:
- Component width doesn't match available space
- `flex: 0 1 auto` on element that should grow
- Layout works randomly in some screens, not others
- DevTools shows "expected 1222px, got 521px"

→ **This rule is violated.**

Go to: **SYMPTOMS.md → FLEX LAYOUT VIOLATIONS**
