# Canon Rule #79: ComponentPage Template Contract

**Status:** ENFORCED


**Severity:** LOW
**Scope:** components
**Version:** 1.0.0
**Date:** 2025-01-16

---

---

## Principle

`ComponentPage` is a **template contract** that provides canonical layout structure for component documentation.

It MUST use:
- `.canon-page` (outer container)
- `.canon-rail` (content rail with max-width)

It MUST NOT use:
- `.page` (generic, conflicts with other systems)
- `.main` (semantic tag, not layout class)

This ensures **visual consistency** and **centralization** across all component documentation pages.

---

## Forbidden Patterns (ABSOLUTE)
```rust
// ❌ NEVER ALLOWED

#[component]
pub fn ComponentPage(...) -> impl IntoView {
    view! {
        <div class="page">  // ❌ Generic class name
            <div class="main">  // ❌ Semantic tag as class
                {children.run()}
            </div>
        </div>
    }
}
```

### Why This is Forbidden

1. **Name Conflicts**
   - `.page` is too generic
   - Clashes with application-level `.page` classes
   - No namespace protection

2. **Semantic Confusion**
   - `.main` is an HTML tag (`<main>`)
   - Using as class creates ambiguity
   - Screen readers may misinterpret

3. **No Centralization**
   - Generic classes don't have associated CSS
   - Each page implements own centering
   - Inconsistent max-width across pages

4. **Breaks Canon Convention**
   - Canon uses `canon-*` prefix for system classes
   - Unprefixed classes are application-level

---

## Canonical Architecture

### Template Structure
```rust
// ✅ CORRECT TEMPLATE

#[component]
pub fn ComponentPage(
    name: String,
    description: String,
    #[prop(into)] children: ViewFn,
    // ... other props
) -> impl IntoView {
    view! {
        <div class="canon-page">  // ← Outer container
            <div class="canon-rail">  // ← Content rail
                <ComponentHeader name=name description=description />
                
                <PreviewBlock>
                    {children.run()}
                </PreviewBlock>
                
                <ApiBlock props=api_props />
                <RulesBlock rules=rules anti_patterns=anti_patterns />
            </div>
        </div>
    }
}
```

### CSS Contract
```css
/* canonrs.css */

.canon-page {
    display: flex;
    justify-content: center;
    width: 100%;
    padding: var(--space-lg);
}

.canon-rail {
    width: 100%;
    max-width: 72rem;  /* 1152px */
    display: flex;
    flex-direction: column;
    gap: var(--space-xl);
}
```

---

## Layout Hierarchy
```
ComponentPage
└─ .canon-page (flex container, centers content)
   └─ .canon-rail (max-width container)
      ├─ ComponentHeader
      ├─ PreviewBlock
      ├─ ApiBlock
      └─ RulesBlock
```

**Key insight:** `.canon-page` provides centering, `.canon-rail` provides max-width.

---

## Usage Pattern

### Button Page Example
```rust
#[component]
pub fn ButtonPage() -> impl IntoView {
    view! {
        <ComponentPage
            name="Button".to_string()
            description="Primary interactive element".to_string()
            symbols=vec![Symbol { label: "SSR".to_string(), variant: SymbolVariant::SSR }]
            when_to_use=vec!["Primary actions".to_string()]
            api_props=vec![
                ApiProp {
                    name: "variant".to_string(),
                    prop_type: "ButtonVariant".to_string(),
                    description: "Visual style".to_string(),
                    required: false,
                }
            ]
            rules=vec!["Clear labels".to_string()]
        >
            <div class="space-y-4">
                <Button>"Solid"</Button>
                <Button variant=ButtonVariant::Outline>"Outline"</Button>
            </div>
        </ComponentPage>
    }
}
```

**Note:** Content goes in `children` slot, NOT as separate props.

---

## Children Slot Contract

### ✅ Correct: ViewFn Slot
```rust
#[component]
pub fn ComponentPage(
    #[prop(into)] children: ViewFn,  // ← Slot for preview content
) -> impl IntoView {
    view! {
        <div class="canon-page">
            <PreviewBlock>
                {children.run()}  // ← Render slot content
            </PreviewBlock>
        </div>
    }
}

// Usage
<ComponentPage>
    <Button>"Preview Content"</Button>
</ComponentPage>
```

### ❌ Incorrect: Separate Prop
```rust
#[component]
pub fn ComponentPage(
    preview: AnyView,  // ❌ Type incompatibility
) -> impl IntoView {
    view! {
        <PreviewBlock>{preview}</PreviewBlock>
    }
}

// Usage - causes type errors
let preview_content = view! { <Button/> }.into_any();
<ComponentPage preview=preview_content />
```

---

## Centralization Mechanics

### How Centering Works
```css
/* Parent (canon-page) centers child via flexbox */
.canon-page {
    display: flex;
    justify-content: center;  /* Horizontally center */
    width: 100%;
}

/* Child (canon-rail) has max-width */
.canon-rail {
    width: 100%;
    max-width: 72rem;  /* Prevents content from being too wide */
}
```

**Result:**
- Viewport < 1152px: Content fills viewport
- Viewport > 1152px: Content centered with max 1152px width

### Visual Diagram
```
┌────────────────────────────────────────────────────┐
│ .canon-page (viewport width)                       │
│                                                     │
│        ┌─────────────────────────────┐            │
│        │ .canon-rail (max 72rem)     │            │
│        │                             │            │
│        │  Component Content          │            │
│        │                             │            │
│        └─────────────────────────────┘            │
│                                                     │
└────────────────────────────────────────────────────┘
```

---

## Real-World Migration

### Before (Broken)
```rust
// ❌ Old template
#[component]
pub fn ComponentPage(...) -> impl IntoView {
    view! {
        <div class="page">
            <div class="main">
                <ComponentHeader />
                <PreviewBlock>{children.run()}</PreviewBlock>
            </div>
        </div>
    }
}
```

**Problems:**
- Content stuck to left
- No centralization CSS
- Generic class names

### After (Fixed)
```rust
// ✅ New template
#[component]
pub fn ComponentPage(...) -> impl IntoView {
    view! {
        <div class="canon-page">
            <div class="canon-rail">
                <ComponentHeader />
                <PreviewBlock>{children.run()}</PreviewBlock>
            </div>
        </div>
    }
}
```

**CSS added:**
```css
.canon-page { display: flex; justify-content: center; }
.canon-rail { max-width: 72rem; }
```

**Result:** Content properly centered ✓

---

## Block Components

### PreviewBlock
```rust
// Shows component variants and states
<PreviewBlock>
    <Button>"Variant 1"</Button>
    <Button variant=ButtonVariant::Outline>"Variant 2"</Button>
</PreviewBlock>
```

### ApiBlock
```rust
// Documents component props
<ApiBlock props=vec![
    ApiProp {
        name: "variant".to_string(),
        prop_type: "ButtonVariant".to_string(),
        description: "Visual style".to_string(),
        required: false,
    }
] />
```

### RulesBlock
```rust
// Shows usage rules and anti-patterns
<RulesBlock 
    rules=vec!["Use clear, action-oriented labels".to_string()]
    anti_patterns=vec!["Multiple primary buttons".to_string()]
/>
```

---

## Template Props Reference
```rust
#[component]
pub fn ComponentPage(
    name: String,                             // Component name (e.g., "Button")
    description: String,                      // Short description
    #[prop(optional)] symbols: Option<Vec<Symbol>>,  // SSR, Hydration badges
    #[prop(into)] children: ViewFn,           // Preview content
    #[prop(optional)] when_to_use: Option<Vec<String>>,
    #[prop(optional)] when_not_to_use: Option<Vec<String>>,
    #[prop(optional)] api_props: Option<Vec<ApiProp>>,
    #[prop(optional)] comparison: Option<Vec<ComparisonRow>>,
    #[prop(optional)] rules: Option<Vec<String>>,
    #[prop(optional)] anti_patterns: Option<Vec<String>>,
) -> impl IntoView
```

**Required:** `name`, `description`, `children`  
**Optional:** All other props

---

## Enforcement Checklist

- [ ] Uses `.canon-page` as outer container
- [ ] Uses `.canon-rail` for content width
- [ ] No `.page` or `.main` classes
- [ ] `children` is `ViewFn` type
- [ ] CSS defines centralization rules
- [ ] Max-width is 72rem (1152px)

---

## Testing Centralization
```javascript
// DevTools Console
const page = document.querySelector('.canon-page');
const rail = document.querySelector('.canon-rail');

console.log('Page width:', page.offsetWidth);
console.log('Rail width:', rail.offsetWidth);
console.log('Rail max-width:', getComputedStyle(rail).maxWidth);
console.log('Page justify-content:', getComputedStyle(page).justifyContent);

// Expected:
// - Page width: full viewport
// - Rail width: min(viewport, 1152px)
// - Rail max-width: 1152px
// - Page justify-content: center
```

---

## Canonical Justification

**Templates are contracts, not implementations.**

A template that uses generic class names:
- Cannot guarantee visual consistency
- Cannot be styled centrally
- Cannot evolve without breaking pages

**Canon mandates:** Namespaced, semantic class names with CSS contract.

---

## Canon References

- Canon Rule #73 — ComponentPage Template Contract (update existing)
- Canon Rule #74 — Block Semantic HTML
- Canon Rule #81 — Flex Layout Ownership

---

## Related Symptoms

If you see:
- Content stuck to left edge
- Inconsistent max-width across pages
- `.page` or `.main` classes in ComponentPage
- Centralization works randomly

→ **This rule is violated.**

Go to: **SYMPTOMS.md → COMPONENTPAGE VIOLATIONS**
