# Canon Rule #72: Layout H1 Prohibition

**Status:** ENFORCED
**Severity:** MEDIUM
**Scope:** accessibility, layout
**Version:** 1.0.0
**Date:** 2025-01-16

---

## Rule

**Layouts MUST NOT contain `<h1>` elements. The `<h1>` tag is reserved exclusively for Page components.**

## Classification

| Aspect | Value |
|--------|-------|
| Category | HTML Semantics |
| Severity | ❌ Critical |
| Scope | Layouts |
| Enforcement | Manual Review + Linting |

## Rationale

### Semantic HTML Hierarchy
- One `<h1>` per page is a fundamental web standard
- Multiple `<h1>` elements confuse:
  - Screen readers
  - Search engines
  - Document outline algorithms

### Separation of Concerns
- **Layout** provides structure (shell)
- **Page** provides content (identity)
- `<h1>` is content identity, not structure

## Anti-Pattern
```rust
// ❌ WRONG - Layout with h1
#[component]
pub fn AppLayout() -> impl IntoView {
    view! {
        <header>
            <h1>"My App"</h1>  // ❌ Layout choosing h1
        </header>
        <main>
            <Outlet/>
        </main>
    }
}

// Result: Page also has h1
#[component]
pub fn DashboardPage() -> impl IntoView {
    view! {
        <h1>"Dashboard"</h1>  // Conflict! Multiple h1
    }
}
```

**Problems:**
- Two `<h1>` elements on same page
- Violates WCAG 2.1
- SEO penalty
- Screen reader confusion

## Correct Pattern
```rust
// ✅ CORRECT - Layout without h1
#[component]
pub fn AppLayout() -> impl IntoView {
    view! {
        <header role="banner">
            <div class="text-lg font-semibold">
                "My App"  // ✅ div, not h1
            </div>
        </header>
        <main>
            <Outlet/>
        </main>
    }
}

// Page owns the h1
#[component]
pub fn DashboardPage() -> impl IntoView {
    view! {
        <h1>"Dashboard"</h1>  // ✅ Single h1 per page
    }
}
```

**Benefits:**
- One `<h1>` per page (semantic)
- Layout provides branding via `<div>` or `<h2>`
- Page controls document identity

## Alternatives for Layout Branding

### Option 1: Plain `<div>` (Recommended)
```rust
<div class="text-lg font-semibold">"App Name"</div>
```

### Option 2: `<h2>` with visual parity
```rust
<h2 class="text-lg font-semibold">"App Name"</h2>
```

### Option 3: ARIA landmark
```rust
<div role="banner" aria-label="Application branding">
    <span class="text-lg font-semibold">"App Name"</span>
</div>
```

## Detection

### Manual Check
```bash
# Search for h1 in layouts
grep -r "<h1" src/layouts/
```

### Expected Output
```
# Should return: (no matches)
```

## Enforcement

### Pre-commit Hook
```bash
#!/bin/bash
if grep -r "<h1" src/layouts/; then
    echo "❌ Canon Rule #72 violated: h1 found in layout"
    exit 1
fi
```

### Clippy Extension (Future)
```rust
// Deny h1 in layout scope
#[deny(layout_h1)]
```

## Exceptions

**None.** This rule has no exceptions.

- Layouts never need `<h1>`
- If you think you need it, you're mixing concerns

## Related Rules

- **Canon Rule #74**: Block Semantic HTML
- **Canon Rule #31**: Accessibility Contract

## References

- [WCAG 2.1 - Headings and Labels](https://www.w3.org/WAI/WCAG21/Understanding/headings-and-labels.html)
- [MDN - Using HTML sections and outlines](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/Heading_Elements)

## Version

- **Created**: 2026-01-12
- **Status**: ✅ Active
