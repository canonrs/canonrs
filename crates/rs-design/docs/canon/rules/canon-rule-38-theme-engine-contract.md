# Canon Rule #38: Theme Engine Contract

**Status:** ✅ Enforced  
**Version:** 1.0.0  
**Date:** 2025-01-02

## Principle
The **Theme Engine** is a single component that reads all provider contexts (Theme, Density, Language) and computes the final CSS custom properties injected into `<style>`. It is the **only** component that performs this computation. No other component may write CSS variables to the DOM.

## Responsibility

**Input Sources:**
1. `ThemeContext` → `{ mode, preset }`
2. `DensityContext` → `{ mode }`
3. `LanguageContext` → `{ current }`
4. `ThemeRegistry` → Preset definitions

**Output:**
- Injects `<style>` tag with CSS variables
- Updates on any context change
- Applies to `:root` selector

**Does NOT:**
- ❌ Persist state
- ❌ Read cookies
- ❌ Call server functions
- ❌ Modify providers

---

## Architecture

### Location
`packages-rust/rs-design/src/providers/theme_engine.rs`

### Component Signature
```rust
#[component]
pub fn ThemeEngine() -> impl IntoView {
    let theme = use_theme();
    let density = use_density();
    let language = use_language();
    
    // Compute CSS variables
    let css_vars = Memo::new(move |_| {
        compute_theme_variables(
            theme.mode.get(),
            theme.preset.get(),
            density.mode.get(),
            language.current.get(),
        )
    });
    
    view! {
        <style>{move || css_vars.get()}</style>
    }
}
```

---

## Computation Flow

### Step 1: Read Contexts
```rust
let theme_mode = theme.mode.get();        // Light | Dark | System
let theme_preset = theme.preset.get();    // "default" | "ocean" | "amber"
let density_mode = density.mode.get();    // Compact | Comfortable | Spacious
let language = language.current.get();    // Language { code, direction }
```

### Step 2: Resolve System Preferences
```rust
// Resolve "system" theme mode to actual light/dark
let resolved_mode = match theme_mode {
    ThemeMode::Light => "light",
    ThemeMode::Dark => "dark",
    ThemeMode::System => {
        #[cfg(target_arch = "wasm32")]
        {
            if window.matchMedia("(prefers-color-scheme: dark)").matches() {
                "dark"
            } else {
                "light"
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            "light" // SSR default
        }
    }
};
```

### Step 3: Load Preset Colors
```rust
let preset = ThemeRegistry::get_preset(&theme_preset)
    .unwrap_or_else(|| ThemeRegistry::default_preset());

let colors = match resolved_mode {
    "dark" => &preset.dark,
    _ => &preset.light,
};
```

### Step 4: Compute Density Multiplier
```rust
let density_multiplier = match density_mode {
    DensityMode::Compact => "0.75",
    DensityMode::Comfortable => "1.0",
    DensityMode::Spacious => "1.25",
};
```

### Step 5: Generate CSS
```rust
fn compute_theme_variables(
    mode: ThemeMode,
    preset_id: String,
    density: DensityMode,
    lang: Language,
) -> String {
    let resolved_mode = resolve_mode(mode);
    let preset = ThemeRegistry::get_preset(&preset_id).unwrap();
    let colors = match resolved_mode {
        "dark" => &preset.dark,
        _ => &preset.light,
    };
    
    format!(r#"
        :root {{
            /* Theme Colors */
            --color-background: {background};
            --color-foreground: {foreground};
            --color-primary-bg: {primary_bg};
            --color-primary-fg: {primary_fg};
            --color-secondary-bg: {secondary_bg};
            --color-secondary-fg: {secondary_fg};
            --color-accent-bg: {accent_bg};
            --color-accent-fg: {accent_fg};
            --color-destructive-bg: {destructive_bg};
            --color-destructive-fg: {destructive_fg};
            --color-muted-bg: {muted_bg};
            --color-muted-fg: {muted_fg};
            --color-card-bg: {card_bg};
            --color-card-fg: {card_fg};
            --color-popover-bg: {popover_bg};
            --color-popover-fg: {popover_fg};
            --color-border: {border};
            --color-input: {input};
            --color-ring: {ring};
            
            /* Density */
            --density-multiplier: {density};
            
            /* RTL Support (if needed) */
            --text-align-start: {text_align_start};
            --text-align-end: {text_align_end};
        }}
    "#,
        background = colors.background,
        foreground = colors.foreground,
        primary_bg = colors.primary,
        primary_fg = colors.primary_foreground,
        secondary_bg = colors.secondary,
        secondary_fg = colors.secondary_foreground,
        accent_bg = colors.accent,
        accent_fg = colors.accent_foreground,
        destructive_bg = colors.destructive,
        destructive_fg = colors.destructive_foreground,
        muted_bg = colors.muted,
        muted_fg = colors.muted_foreground,
        card_bg = colors.card,
        card_fg = colors.card_foreground,
        popover_bg = colors.popover,
        popover_fg = colors.popover_foreground,
        border = colors.border,
        input = colors.input,
        ring = colors.ring,
        density = match density {
            DensityMode::Compact => "0.75",
            DensityMode::Comfortable => "1.0",
            DensityMode::Spacious => "1.25",
        },
        text_align_start = match lang.direction {
            TextDirection::Rtl => "right",
            TextDirection::Ltr => "left",
        },
        text_align_end = match lang.direction {
            TextDirection::Rtl => "left",
            TextDirection::Ltr => "right",
        },
    )
}
```

---

## Output Example

**Given:**
- Theme: Dark mode, Ocean preset
- Density: Compact
- Language: Arabic (RTL)

**Output CSS:**
```css
:root {
    /* Theme Colors */
    --color-background: hsl(222.2 84% 4.9%);
    --color-foreground: hsl(210 40% 98%);
    --color-primary-bg: hsl(217.2 91.2% 59.8%);
    --color-primary-fg: hsl(222.2 47.4% 11.2%);
    /* ... all other color tokens ... */
    
    /* Density */
    --density-multiplier: 0.75;
    
    /* RTL Support */
    --text-align-start: right;
    --text-align-end: left;
}
```

---

## Integration Point

### App Structure
```rust
<Router>
  <LanguageProvider initial_language>
    <ThemeProvider initial_mode initial_preset>
      <DensityProvider initial_mode>
        <ThemeEngine />  {/* ← SINGLE POINT OF CSS INJECTION */}
        <App />
      </DensityProvider>
    </ThemeProvider>
  </LanguageProvider>
</Router>
```

**Why After Providers?**
- ThemeEngine needs all contexts to be available
- Providers must wrap ThemeEngine for `use_*` hooks to work

---

## Reactivity

### Automatic Updates
```rust
// Memo recomputes whenever ANY context changes
let css_vars = Memo::new(move |_| {
    compute_theme_variables(
        theme.mode.get(),      // ← Tracks theme changes
        theme.preset.get(),    // ← Tracks preset changes
        density.mode.get(),    // ← Tracks density changes
        language.current.get(), // ← Tracks language changes
    )
});
```

**Trigger Examples:**
1. User clicks "Dark Mode" → `theme.mode` changes → CSS updates
2. User selects "Ocean" preset → `theme.preset` changes → CSS updates
3. User switches to "Compact" → `density.mode` changes → CSS updates
4. User changes language to Arabic → `language.current` changes → CSS updates (RTL)

---

## Performance

### Optimization: Memoization
```rust
// ✅ CORRECT - Memo only recomputes when inputs change
let css_vars = Memo::new(move |_| compute_theme_variables(...));

// ❌ WRONG - Recomputes on every render
let css_vars = move || compute_theme_variables(...);
```

### Optimization: Debouncing (Optional)

For rapid theme changes (e.g., scrubbing through presets):
```rust
let css_vars = create_memo_with_prev(move |prev| {
    let new_css = compute_theme_variables(...);
    
    // Only update if actually different
    if prev.as_ref() != Some(&new_css) {
        new_css
    } else {
        prev.unwrap_or_default()
    }
});
```

---

## Prohibited Patterns

### ❌ Multiple Theme Engines

**WRONG - More than one ThemeEngine:**
```rust
// ❌ FORBIDDEN
<ThemeEngine />
<App>
  <ThemeEngine />  {/* ❌ DUPLICATE! */}
</App>
```

**Consequence:** CSS variables conflict, last one wins.

### ❌ Manual CSS Variable Injection

**WRONG - Components writing CSS vars directly:**
```rust
// ❌ FORBIDDEN (in any component except ThemeEngine)
#[component]
pub fn MyComponent() -> impl IntoView {
    view! {
        <style>
            {":root { --color-primary: red; }"}  {/* ❌ NO! */}
        </style>
    }
}
```

**Consequence:** Breaks theme system, causes conflicts.

### ❌ Direct DOM Manipulation of CSS Vars

**WRONG - Using JavaScript to set CSS vars:**
```rust
// ❌ FORBIDDEN
#[cfg(target_arch = "wasm32")]
Effect::new(move |_| {
    let html = document.documentElement();
    html.style().set_property("--color-primary", "red");  // ❌ NO!
});
```

**Consequence:** Out of sync with providers, breaks reactivity.

### ❌ Conditional Theme Engine

**WRONG - Only rendering ThemeEngine sometimes:**
```rust
// ❌ FORBIDDEN
<Show when=some_condition>
  <ThemeEngine />
</Show>
```

**Consequence:** No CSS vars when condition is false.

---

## SSR Considerations

### Server-Side Rendering

**ThemeEngine on Server:**
- Computes CSS based on initial props (from cookies)
- Renders `<style>` tag in HTML
- Client hydrates without re-computing (if values match)

**Example SSR Output:**
```html
<html lang="pt" dir="ltr" class="dark" data-theme="ocean" data-density="compact">
<head>
  <style>
    :root {
      --color-background: hsl(222.2 84% 4.9%);
      /* ... */
    }
  </style>
</head>
<body>...</body>
</html>
```

**Client Hydration:**
1. Anti-flash script applies `data-theme`, `data-density`, `lang`, `dir`
2. SSR HTML includes `<style>` with correct CSS vars
3. Providers hydrate with matching values
4. ThemeEngine renders, computes same CSS
5. No flash, no re-computation if values match

---

## Testing

### Unit Test: CSS Generation
```rust
#[test]
fn test_theme_engine_generates_correct_css() {
    let css = compute_theme_variables(
        ThemeMode::Dark,
        "ocean".to_string(),
        DensityMode::Compact,
        Language::new("en"),
    );
    
    assert!(css.contains("--color-background"));
    assert!(css.contains("--density-multiplier: 0.75"));
    assert!(css.contains("--text-align-start: left"));
}
```

### Integration Test: Reactivity
```rust
#[test]
fn test_theme_engine_reacts_to_context_changes() {
    let theme = ThemeContext::new();
    let density = DensityContext::new();
    
    // Change theme
    theme.mode.set(ThemeMode::Dark);
    
    // ThemeEngine should recompute
    let css = get_computed_css_vars();
    assert!(css.contains("--color-background: hsl(222.2 84% 4.9%)"));
}
```

### Visual Test: No Flash

1. Set theme to dark, density to compact
2. Reload page
3. Verify CSS vars are correct from frame 1
4. **PASS:** No flash, correct colors immediately

---

## Validation Checklist

- [ ] Only one `<ThemeEngine />` in app
- [ ] ThemeEngine is inside all providers
- [ ] ThemeEngine uses `Memo` for performance
- [ ] No other components write CSS variables
- [ ] SSR renders correct CSS in `<style>`
- [ ] Client hydration matches SSR output
- [ ] Theme changes update CSS reactively
- [ ] Density changes update `--density-multiplier`
- [ ] Language changes update RTL variables (if used)

---

## References

- [Canon Rule #21: Canonical Color Tokens](./canon-rule-21-canonical-color-tokens.md)
- [Canon Rule #25: Theme Presets Contract](./canon-rule-25-theme-presets-contract.md)
- [Canon Rule #32: Theme Persistence Contract](./canon-rule-32-theme-persistence-contract.md)
- [Canon Rule #33: Density & Accessibility Mapping](./canon-rule-33-density-accessibility-mapping.md)
- [Canon Rule #37: Provider Taxonomy & Boundaries](./canon-rule-37-provider-taxonomy-boundaries.md)

---

**Enforcement:** Code review + linting  
**Instance Count:** Exactly 1 per app  
**Output:** CSS custom properties in `<style>` tag  
**Reactivity:** Automatic via Memo + contexts
