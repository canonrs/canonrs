# Canon Rule #74: Block Semantic HTML

**Status:** ENFORCED
**Severity:** HIGH
**Version:** 1.0.0
**Date:** 2025-01-16

**Category:** accessibility
**Tags:** html, aria, wcag
**Language:** EN

---

**Intro:**
Improper HTML semantics in design system blocks break accessibility, screen reader navigation, and document hierarchy. Blocks must enforce semantic structure with ARIA attributes and correct heading levels.

**Problem:**
blocks lack semantic html and proper aria leading to accessibility issues

**Solution:**
use section landmarks aria-labelledby and strict heading hierarchy

**Signals:**
- axe violations
- screen reader confusion
- invalid heading hierarchy

**Search Intent:**
how to implement accessible semantic html in components

**Keywords:**
wcag semantic html blocks, aria labelledby section pattern, heading hierarchy accessibility, accessible table markup

---

## Rule

**Blocks MUST use semantic HTML with proper ARIA attributes, heading hierarchy, and accessible table markup. All documentation blocks MUST follow WCAG 2.1 guidelines.**

## Classification

| Aspect | Value |
|--------|-------|
| Category | HTML Semantics + A11y |
| Severity | ❌ Critical |
| Scope | Design System Blocks |
| Enforcement | Manual Review + Axe |

## Rationale

### Accessibility is Not Optional
- Screen readers depend on semantic structure
- Keyboard navigation requires proper landmarks
- SEO benefits from document outline

### Enterprise Compliance
- WCAG 2.1 AA compliance required
- Legal requirements (ADA, Section 508)
- Audit-ready markup

## Block Structure Pattern

### General Pattern
```rust
#[component]
pub fn SomeBlock() -> impl IntoView {
    view! {
        <section 
            class="block-class" 
            aria-labelledby="block-heading"
        >
            <h2 id="block-heading" class="sr-only">
                "Screen reader description"
            </h2>
            
            // Block content
        </section>
    }
}
```

**Requirements:**
- ✅ `<section>` for semantic grouping
- ✅ `aria-labelledby` pointing to heading
- ✅ `<h2>` with unique id
- ✅ `.sr-only` for visual hiding

## Heading Hierarchy

### Correct Hierarchy
```
<h1> → Page title (ComponentHeader)
  <h2> → Block titles (UsageBlock, ApiBlock, etc.)
    <h3> → Sub-sections within blocks
```

### Anti-Pattern
```rust
// ❌ WRONG - Skipping h2
view! {
    <section>
        <h3>"API"</h3>  // ❌ Should be h2
    </section>
}
```

### Correct Pattern
```rust
// ✅ CORRECT
view! {
    <section aria-labelledby="api-heading">
        <h2 id="api-heading">"API"</h2>
        
        <div>
            <h3>"Props"</h3>      // ✅ Nested correctly
            <h3>"Methods"</h3>     // ✅ Same level
        </div>
    </section>
}
```

## Table Semantics

### Required Elements
```rust
<table>
    <caption class="sr-only">
        "Descriptive table purpose"
    </caption>
    
    <thead>
        <tr>
            <th scope="col">Column 1</th>
            <th scope="col">Column 2</th>
        </tr>
    </thead>
    
    <tbody>
        <tr>
            <th scope="row">Row header</th>
            <td>Data</td>
        </tr>
    </tbody>
</table>
```

**Rules:**
- ✅ `<caption class="sr-only">` always present
- ✅ `scope="col"` on column headers
- ✅ `scope="row"` on row headers
- ✅ `<thead>` and `<tbody>` explicit

### Anti-Pattern
```rust
// ❌ WRONG - No scope, no caption
<table>
    <tr>
        <th>Name</th>      // ❌ Missing scope
        <th>Type</th>
    </tr>
    <tr>
        <td>variant</td>   // ❌ Should be th scope="row"
        <td>string</td>
    </tr>
</table>
```

### Correct Pattern
```rust
// ✅ CORRECT - Full semantics
<table>
    <caption class="sr-only">"Component API properties"</caption>
    <thead>
        <tr>
            <th scope="col">"Name"</th>
            <th scope="col">"Type"</th>
        </tr>
    </thead>
    <tbody>
        <tr>
            <th scope="row">"variant"</th>
            <td>"string"</td>
        </tr>
    </tbody>
</table>
```

## Screen Reader Only Content

### The `.sr-only` Utility
```css
.sr-only {
  position: absolute;
  width: 1px;
  height: 1px;
  padding: 0;
  margin: -1px;
  overflow: hidden;
  clip: rect(0, 0, 0, 0);
  white-space: nowrap;
  border: 0;
}
```

**Usage:**
```rust
<h2 class="sr-only">"Section description"</h2>
<caption class="sr-only">"Table purpose"</caption>
```

**Rule:** Use `.sr-only` for semantic headings that aren't needed visually.

## Block Examples

### UsageBlock
```rust
#[component]
pub fn UsageBlock(
    when_to_use: Vec<String>,
    when_not_to_use: Vec<String>,
) -> impl IntoView {
    view! {
        <section class="usage-block" aria-labelledby="usage-heading">
            <h2 id="usage-heading" class="sr-only">"Usage"</h2>
            
            <div class="usage-section">
                <h3>"When to use"</h3>
                <ul class="usage-text">
                    {when_to_use.into_iter().map(|item| {
                        view! { <li>{item}</li> }
                    }).collect_view()}
                </ul>
            </div>
        </section>
    }
}
```

### ApiBlock
```rust
#[component]
pub fn ApiBlock(props: Vec<ApiProp>) -> impl IntoView {
    view! {
        <section class="api-block" aria-labelledby="api-heading">
            <h2 id="api-heading">"API"</h2>
            
            <table>
                <caption class="sr-only">
                    "Component API properties"
                </caption>
                <thead>
                    <tr>
                        <th scope="col">"Prop"</th>
                        <th scope="col">"Type"</th>
                        <th scope="col">"Required"</th>
                    </tr>
                </thead>
                <tbody>
                    {props.into_iter().map(|prop| {
                        view! {
                            <tr>
                                <th scope="row"><code>{prop.name}</code></th>
                                <td><code>{prop.prop_type}</code></td>
                                <td>{if prop.required { "Yes" } else { "No" }}</td>
                            </tr>
                        }
                    }).collect_view()}
                </tbody>
            </table>
        </section>
    }
}
```

## Validation

### Axe DevTools
```bash
# Install
npm install -D @axe-core/cli

# Run
axe http://localhost:3000/button
```

### Expected Output
```
✅ 0 violations found
```

### Manual Checklist
- [ ] Each block is a `<section>`
- [ ] Each section has `aria-labelledby`
- [ ] Headings follow h1 → h2 → h3 hierarchy
- [ ] Tables have `<caption class="sr-only">`
- [ ] Tables have `scope` on all headers
- [ ] `.sr-only` content is meaningful

## Exceptions

**None.** All blocks must follow semantic HTML rules.

## Related Rules

- **Canon Rule #72**: Layout H1 Prohibition
- **Canon Rule #31**: Accessibility Contract
- **Canon Rule #76**: Accessibility Utilities Location

## References

- [WCAG 2.1 Guidelines](https://www.w3.org/WAI/WCAG21/quickref/)
- [MDN - ARIA](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA)
- [WebAIM - Screen Reader Testing](https://webaim.org/articles/screenreader_testing/)

## Version

- **Created**: 2026-01-12
- **Status**: ✅ Active