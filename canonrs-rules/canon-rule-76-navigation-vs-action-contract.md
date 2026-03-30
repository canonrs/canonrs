# Canon Rule #76: Navigation vs Action Component Contract

**Status:** ENFORCED


**Severity:** HIGH
**Scope:** components, ui
**Version:** 1.0.0
**Date:** 2025-01-16

---


---

## Principle

**Navigation uses `<a>`.**
**Actions use `<button>`.**

NEVER nest anchor tags inside buttons (`<button><a>`). This creates invalid HTML, breaks accessibility, and violates semantic web standards.

Components MUST be architecturally separated by their intent:
- **Navigation** → Changes URL → `<a href>`
- **Action** → Triggers behavior → `<button>`

---

## Forbidden Patterns
```rust
// ❌ NEVER ALLOWED

#[component]
pub fn SidebarMenuButton(children: Children) -> impl IntoView {
    view! {
        <button data-sidebar-menu-button="">
            <a href="/tokens">  // ❌ INVALID HTML
                {children()}
            </a>
        </button>
    }
}
```

### Why This is Forbidden

1. **HTML Validity**
   - W3C specification prohibits interactive content inside buttons
   - Browsers handle click events inconsistently
   - Some browsers fire both button and link events

2. **Accessibility**
   - Screen readers announce "button link" (ambiguous)
   - Keyboard navigation breaks (Tab? Enter? Space?)
   - ARIA roles conflict

3. **Click Event Conflicts**
   - `onClick` on button fires
   - `href` navigation triggers
   - JavaScript event.preventDefault() workarounds required
   - Race conditions in event bubbling

4. **SEO Impact**
   - Search engines ignore nested links
   - Site structure becomes invisible
   - Crawlers cannot follow navigation

---

## Canonical Architecture

### Separate Components by Intent
```rust
// ✅ NAVIGATION COMPONENT
#[component]
pub fn SidebarMenuLink(
    href: String,
    children: Children,
    #[prop(default = false)] is_active: bool,
) -> impl IntoView {
    view! {
        
            href=href
            data-sidebar-menu-link=""
            data-active=if is_active { "true" } else { "false" }
            style="
                display: block;
                height: var(--nav-item-height);
                padding: var(--space-sm) var(--space-md);
                color: var(--color-fg-default);
                text-decoration: none;
            "
        >
            {children()}
        </a>
    }
}

// ✅ ACTION COMPONENT
#[component]
pub fn SidebarMenuButton(
    children: Children,
    on_click: Callback<()>,
    #[prop(default = false)] is_active: bool,
) -> impl IntoView {
    view! {
        <button
            data-sidebar-menu-button=""
            data-active=if is_active { "true" } else { "false" }
            on:click=move |_| on_click.call(())
            style="
                display: block;
                height: var(--nav-item-height);
                padding: var(--space-sm) var(--space-md);
            "
        >
            {children()}
        </button>
    }
}
```

---

## Usage Decision Tree
```
Does this component change the URL?
│
├─ YES → Use <a> (SidebarMenuLink)
│         Examples: /dashboard, /settings, /profile
│
└─ NO → Does it trigger an action?
         │
         ├─ YES → Use <button> (SidebarMenuButton)
         │         Examples: toggle sidebar, open modal, submit form
         │
         └─ NO → Use <div> or <span> (informational only)
```

---

## Real World Example

### Before (Violates Rule)
```rust
// ❌ HTML INVALID
<SidebarMenu>
    <SidebarMenuItem>
        <SidebarMenuButton>
            <A href="/tokens">Tokens</A>  // ❌ button > a
        </SidebarMenuButton>
    </SidebarMenuItem>
</SidebarMenu>
```

**Problems:**
- Invalid HTML
- Accessibility broken
- Click events conflict
- Cannot style link independently

### After (Compliant)
```rust
// ✅ CORRECT: Separate components
<SidebarMenu>
    <SidebarMenuItem>
        <SidebarMenuLink href="/tokens">
            Tokens
        </SidebarMenuLink>
    </SidebarMenuItem>
    
    <SidebarMenuItem>
        <SidebarMenuButton on_click=handle_collapse>
            Collapse All
        </SidebarMenuButton>
    </SidebarMenuItem>
</SidebarMenu>
```

---

## Token Application

Both components share tokens but differ semantically:
```rust
// Navigation (Link)
style="
    height: var(--nav-item-height);
    padding: var(--space-sm) var(--space-md);
    color: var(--color-fg-default);
    background: var(--color-bg-surface);
    text-decoration: none;  // ← Link-specific
"

// Action (Button)
style="
    height: var(--nav-item-height);
    padding: var(--space-sm) var(--space-md);
    color: var(--color-fg-default);
    background: var(--color-bg-surface);
    cursor: pointer;  // ← Button-specific
"
```

**Shared tokens:** height, padding, color, background
**Different properties:** text-decoration vs cursor

---

## Accessibility Requirements

### Navigation Links MUST Have

- [ ] Valid `href` attribute
- [ ] Clear, descriptive text (not "click here")
- [ ] `aria-current="page"` when active
- [ ] Keyboard accessible (Tab)
- [ ] Focus visible

### Action Buttons MUST Have

- [ ] Descriptive text or `aria-label`
- [ ] `type="button"` (prevents form submission)
- [ ] `disabled` when action unavailable
- [ ] Keyboard accessible (Tab + Enter/Space)
- [ ] Focus visible

---

## Enforcement Checklist

- [ ] No `<a>` inside `<button>`
- [ ] No `<button>` inside `<a>`
- [ ] Navigation components use `href`
- [ ] Action components use `on:click`
- [ ] Separate component names (Link vs Button)
- [ ] Shared styling via tokens, not inheritance

---

## Component Naming Convention
```rust
// ✅ CORRECT NAMING
SidebarMenuLink      // Navigation
SidebarMenuButton    // Action

CardLink             // Navigation
CardButton           // Action

DropdownLink         // Navigation
DropdownButton       // Action
```

**Rule:** Component name MUST reflect HTML element used.

---

## Exceptions

**NONE.**

This rule has no exceptions. Even for:
- "Just this one screen"
- "Our designer said so"
- "It looks the same"

HTML validity is non-negotiable.

---

## Violation Consequences

1. **Immediate:** Browser console warnings
2. **Short-term:** Screen reader users cannot navigate
3. **Long-term:** Fails WCAG 2.1 compliance
4. **Enterprise:** Legal liability (ADA violations)

---

## Testing
```rust
// ✅ Test navigation
assert!(page.find("a[href='/tokens']").exists());
assert_eq!(page.find("a[href='/tokens']").text(), "Tokens");

// ✅ Test action
page.find("button[data-sidebar-trigger]").click();
assert!(sidebar.is_collapsed());
```

---

## Canonical Justification

The web has two fundamental interactions:
1. **Go somewhere** (navigation)
2. **Do something** (action)

Mixing them creates **ambiguity at every level**:
- Semantic (what does this do?)
- Technical (which event fires?)
- Legal (is this accessible?)

**Canon mandates:** One intent, one element.

---

## Canon References

- Canon Rule #74 — Block Semantic HTML
- Canon Rule #75 — Primitive CSS Prohibition
- Canon Rule #31 — Accessibility Contract

---

## Related Symptoms

If you see:
- `<button><a>` or `<a><button>` in DOM inspector
- "Interactive element inside interactive element" console warning
- Screen reader announces "button link"
- Keyboard navigation skips elements

→ **This rule is violated.**

Go to: **SYMPTOMS.md → HTML VALIDITY VIOLATIONS**
