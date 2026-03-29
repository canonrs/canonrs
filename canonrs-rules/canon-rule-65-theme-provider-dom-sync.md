# Canon Rule #65: data-theme Sync is ThemeProvider Responsibility

**Status:** ENFORCED


**Severity:** HIGH
**Scope:** tokens, theming, providers
**Version:** 1.0.0
**Date:** 2025-01-16

---
  

---

## The Principle

**The ThemeProvider MUST synchronize theme state to the DOM.**

Controlling Rust signals is NOT enough. CSS selectors like `[data-theme="..."]` require actual DOM attributes.

---

## The Problem

### ❌ Wrong Pattern (State Only)
```rust
#[component]
pub fn ThemeProvider(children: Children) -> impl IntoView {
    let mode = RwSignal::new(ThemeMode::Light);
    let preset = RwSignal::new("amber-minimal".to_string());
    
    provide_context(ThemeContext { mode, preset });
    
    children()  // 🚫 NO DOM sync
}
```

**CSS expects:**
```css
[data-theme="amber-minimal"] {
  --color-background: 0 0% 100%;
}
```

**But HTML has:**
```html
<html

**Result:**
- Tokens defined ✅
- State controlled ✅
- CSS never activates ❌
- Theme doesn't apply ❌

---

## The Solution

### ✅ Correct Pattern (State + DOM Sync)
```rust
#[component]
pub fn ThemeProvider(
    #[prop(optional)] initial_preset: Option<String>,
    children: Children,
) -> impl IntoView {
    let mode = RwSignal::new(ThemeMode::Light);
    let preset = RwSignal::new(initial_preset.unwrap_or("amber-minimal".to_string()));
    
    // ✅ CRITICAL: Sync to DOM
    #[cfg(target_arch = "wasm32")]
    Effect::new(move |_| {
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(html) = document.document_element() {
                    // Sync preset
                    let _ = html.set_attribute("data-theme", &preset.get());
                    
                    // Sync mode
                    let is_dark = matches!(mode.get(), ThemeMode::Dark);
                    let _ = html.class_list().toggle_with_force("dark", is_dark);
                }
            }
        }
    });
    
    provide_context(ThemeContext { mode, preset });
    children()
}
```

**Result:**
```html
<html data-theme="amber-minimal" class="">  <!-- ✅ Synced -->
```

---

## Architecture Layers

### Layer 1: Rust State (ThemeContext)
**Responsibility:** Store theme in reactive signals
```rust
pub struct ThemeContext {
    pub mode: RwSignal<ThemeMode>,
    pub preset: RwSignal<String>,
}
```

### Layer 2: DOM Sync (Effect)
**Responsibility:** Mirror state to HTML attributes
```rust
Effect::new(move |_| {
    html.set_attribute("data-theme", &preset.get());
    html.class_list().toggle_with_force("dark", is_dark);
});
```

### Layer 3: CSS Consumption
**Responsibility:** Apply styles based on attributes
```css
[data-theme="amber-minimal"] {
  --color-background: 0 0% 100%;
}

.dark {
  --color-background: 0 0% 9%;
}
```

---

## Why Both Are Required

### Rust State Alone (Insufficient)
```rust
let preset = RwSignal::new("amber-minimal");
provide_context(preset);
```

**Can do:**
- ✅ UI components react to changes
- ✅ Conditional rendering works
- ✅ Settings panel updates

**Cannot do:**
- ❌ CSS `[data-theme]` selectors don't match
- ❌ Tokens don't activate
- ❌ Theme doesn't apply visually

### DOM Attribute Alone (Insufficient)
```rust
html.set_attribute("data-theme", "amber-minimal");
```

**Can do:**
- ✅ CSS selectors match
- ✅ Theme applies visually

**Cannot do:**
- ❌ No reactive updates
- ❌ UI components don't know current theme
- ❌ Settings panel out of sync

### Both Together (Correct)
```rust
let preset = RwSignal::new("amber-minimal");

Effect::new(move |_| {
    html.set_attribute("data-theme", &preset.get());
});
```

**Result:**
- ✅ Rust state for reactivity
- ✅ DOM attribute for CSS
- ✅ Changes propagate everywhere
- ✅ System stays in sync

---

## Implementation Checklist

When implementing ThemeProvider:

- [ ] Store theme in `RwSignal` (for reactivity)
- [ ] Use `Effect::new` to sync to DOM
- [ ] Set `data-theme` attribute on `<html>`
- [ ] Toggle `.dark` class based on mode
- [ ] Use `#[cfg(target_arch = "wasm32")]` guard
- [ ] Test: `curl localhost | grep data-theme` shows value

---

## Health Check Commands
```bash
# 1. Verify Rust state exists
# (In component code)
let ctx = use_context::<ThemeContext>();
# ✅ Should not panic

# 2. Verify DOM attribute set
curl -s http://localhost:3003 | grep 'data-theme='
# ✅ Should return: data-theme="amber-minimal"

# 3. Verify dark mode class
curl -s http://localhost:3003 | grep '<html.*class="dark"'
# ✅ Should match when dark mode active

# 4. Test in browser console
document.documentElement.getAttribute('data-theme')
# ✅ Should return: "amber-minimal"

# 5. Test dark mode toggle
document.documentElement.classList.contains('dark')
# ✅ Should return: true or false
```

---

## Common Mistakes

### ❌ Mistake 1: No Effect
```rust
// WRONG: State exists but never syncs
let preset = RwSignal::new("amber");
provide_context(preset);
// 🚫 DOM never updated
```

**Fix:**
```rust
Effect::new(move |_| {
    html.set_attribute("data-theme", &preset.get());
});
```

### ❌ Mistake 2: Missing wasm32 Guard
```rust
// WRONG: Runs on server (panics)
Effect::new(move |_| {
    let window = web_sys::window().unwrap();  // 🚫 Panics in SSR
});
```

**Fix:**
```rust
#[cfg(target_arch = "wasm32")]
Effect::new(move |_| {
    if let Some(window) = web_sys::window() { ... }
});
```

### ❌ Mistake 3: One-Time Sync
```rust
// WRONG: Only sets attribute once
html.set_attribute("data-theme", "amber");

// User changes theme → Nothing happens
```

**Fix:**
```rust
// CORRECT: Reactive sync
Effect::new(move |_| {
    html.set_attribute("data-theme", &preset.get());
});
```

### ❌ Mistake 4: Forgetting Dark Mode
```rust
// WRONG: Only syncs preset
Effect::new(move |_| {
    html.set_attribute("data-theme", &preset.get());
    // 🚫 Missing dark mode class
});
```

**Fix:**
```rust
Effect::new(move |_| {
    html.set_attribute("data-theme", &preset.get());
    let is_dark = matches!(mode.get(), ThemeMode::Dark);
    html.class_list().toggle_with_force("dark", is_dark);
});
```

---

## Debugging Guide

### Problem: "Theme not applying"

**Step 1: Check Rust state**
```rust
// Add debug log
Effect::new(move |_| {
    log!("Current preset: {}", preset.get());
});
```

**Step 2: Check DOM attribute**
```bash
curl -s http://localhost:3003 | grep data-theme
# If missing → Effect not running
```

**Step 3: Check CSS selector**
```css
/* Verify selector matches attribute */
[data-theme="amber-minimal"] { ... }
```

**Step 4: Check browser**
```javascript
getComputedStyle(document.documentElement).getPropertyValue('--color-background')
// If empty → CSS not loading or selector wrong
```

### Problem: "Theme changes don't update UI"

**Cause: Effect not reactive**
```rust
// WRONG: Captures value at init
let theme = preset.get();
Effect::new(move |_| {
    html.set_attribute("data-theme", &theme);  // Stale
});
```

**Fix: Access signal inside Effect**
```rust
Effect::new(move |_| {
    let theme = preset.get();  // Fresh value
    html.set_attribute("data-theme", &theme);
});
```

---

## SSR Considerations

### Client-Only Sync
```rust
// Effect only runs in browser
#[cfg(target_arch = "wasm32")]
Effect::new(move |_| {
    // Safe: only executes client-side
    let html = document.document_element().unwrap();
    html.set_attribute("data-theme", &preset.get());
});
```

### SSR Hydration
```rust
// Option: Inject initial theme in SSR HTML
pub fn render_shell(preset: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html data-theme="{}">
<head>...</head>
<body>...</body>
</html>"#,
        preset
    )
}
```

**Then client-side Effect takes over after hydration.**

---

## Testing
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn theme_provider_syncs_to_dom() {
        // This test requires browser environment
        // Use wasm-bindgen-test
        
        let preset = RwSignal::new("amber".to_string());
        
        // Verify Effect syncs
        Effect::new(move |_| {
            assert_eq!(
                document.document_element()
                    .unwrap()
                    .get_attribute("data-theme"),
                Some("amber".to_string())
            );
        });
    }
}
```

---

## Anti-Patterns to Avoid

### 🚫 Manual DOM Updates Outside Provider
```rust
// WRONG: Random component mutates DOM
#[component]
fn MyButton() -> impl IntoView {
    let on_click = move |_| {
        document.document_element()
            .unwrap()
            .set_attribute("data-theme", "blue");  // 🚫 Bypasses state
    };
}
```

### 🚫 Multiple Theme Providers
```rust
// WRONG: Conflicting providers
<ThemeProvider initial_preset="amber">
    <ThemeProvider initial_preset="blue">  // 🚫 Which wins?
        <App/>
    </ThemeProvider>
</ThemeProvider>
```

---

## Comparison: CanonRS vs Others

| Framework | State Management | DOM Sync | Auto-Sync |
|-----------|-----------------|----------|-----------|
| **CanonRS (Leptos)** | RwSignal | Manual Effect | ❌ No |
| Next.js (next-themes) | useState | useEffect | ✅ Yes |
| SvelteKit | writable | $: reactive | ✅ Yes |
| Solid.js | createSignal | createEffect | ❌ No |

**Veredito:** Leptos requires **explicit Effect**, similar to Solid.js.

---

## Related Rules

- **Rule #62:** Single Source of Truth for Design Tokens
- **Rule #63:** Leptos Reactivity (Effect usage)
- **Rule #64:** CSS Build Pipeline

---

## Normative Status

- ThemeProvider **MUST** sync state to DOM
- Effect **MUST** be guarded with `#[cfg(target_arch = "wasm32")]`
- Both `data-theme` and `.dark` class **MUST** be synced
- PRs adding ThemeProvider **MUST** include DOM sync
- Test: `curl localhost | grep data-theme` **MUST** pass

---

**Author:** Canon Working Group  
**Replaces:** None

---

## Economic Impact

**Time saved per incident:** ~45 minutes  
**Frequency without rule:** Every theme implementation  
**Annual savings (5 themes):** ~4 hours

**Root causes eliminated:**
- ❌ "CSS has tokens but theme doesn't apply"
- ❌ State works but DOM out of sync
- ❌ Theme toggle updates UI but not styles
- ❌ Confusion between Rust state and DOM state
