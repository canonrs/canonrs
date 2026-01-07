# Canon Rule #37: Provider Taxonomy & Boundaries

**Status:** ✅ Enforced  
**Version:** 1.0.0  
**Date:** 2025-01-02

## Principle
The design system defines **exactly three canonical providers**: Theme, Density, and Language. Each provider is **orthogonal**, **stateless** (on the design system layer), and **SSR-safe**. Persistence, business logic, and HTTP concerns belong exclusively to the app layer.

## The Three Canonical Providers

### 1️⃣ ThemeProvider

**Responsibility:**
- Apply `data-theme` attribute to `<html>`
- Apply `class="light|dark"` based on resolved mode
- Expose `ThemeContext { mode, preset }`

**Does NOT:**
- ❌ Persist to cookies/localStorage
- ❌ Call server functions
- ❌ Make HTTP requests
- ❌ Know about user preferences

**Location:** `packages-rust/rs-design/src/providers/theme_provider.rs`

**Props:**
```rust
#[component]
pub fn ThemeProvider(
    #[prop(optional)] initial_mode: Option<String>,
    #[prop(optional)] initial_preset: Option<String>,
    children: Children,
) -> impl IntoView
```

**Context:**
```rust
#[derive(Clone, Copy)]
pub struct ThemeContext {
    pub mode: RwSignal<ThemeMode>,
    pub preset: RwSignal<String>,
}

pub fn use_theme() -> ThemeContext;
```

**DOM Effects:**
```html
<html class="dark" data-theme="ocean">
```

---

### 2️⃣ DensityProvider

**Responsibility:**
- Apply `data-density` attribute to `<html>`
- Expose `DensityContext { mode }`
- Define density modes (compact/comfortable/spacious)

**Does NOT:**
- ❌ Persist to cookies/localStorage
- ❌ Calculate multipliers in runtime (CSS handles this)
- ❌ Override based on device (CSS media queries handle this)

**Location:** `packages-rust/rs-design/src/providers/density_provider.rs`

**Props:**
```rust
#[component]
pub fn DensityProvider(
    #[prop(optional)] initial_mode: Option<String>,
    children: Children,
) -> impl IntoView
```

**Context:**
```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DensityMode {
    Compact,      // 0.75x
    Comfortable,  // 1.0x (default)
    Spacious,     // 1.25x
}

#[derive(Clone, Copy)]
pub struct DensityContext {
    pub mode: RwSignal<DensityMode>,
}

pub fn use_density() -> DensityContext;
```

**DOM Effects:**
```html
<html data-density="comfortable">
```

**CSS Contract:**
```css
:root {
  --density-multiplier: 1.0;
}

[data-density="compact"] {
  --density-multiplier: 0.75;
}

[data-density="spacious"] {
  --density-multiplier: 1.25;
}

/* Mobile override (Canon Rule #33) */
@media (pointer: coarse) {
  [data-density="compact"] {
    --density-multiplier: 1.1; /* Enforce 44px minimum */
  }
}
```

---

### 3️⃣ LanguageProvider

**Responsibility:**
- Apply `lang` attribute to `<html>`
- Apply `dir="ltr|rtl"` based on language
- Expose `LanguageContext { current }`
- Model languages with name, code, direction

**Does NOT:**
- ❌ Persist to cookies/localStorage
- ❌ Load translations (i18n layer does this)
- ❌ Detect browser language (app does this)

**Location:** `packages-rust/rs-design/src/providers/language_provider.rs`

**Props:**
```rust
#[component]
pub fn LanguageProvider(
    #[prop(optional)] initial_language: Option<String>,
    children: Children,
) -> impl IntoView
```

**Context:**
```rust
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Language {
    pub code: String,          // "en", "pt", "ar"
    pub name: String,          // "English", "Português", "العربية"
    pub direction: TextDirection,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextDirection {
    Ltr,  // Left-to-Right
    Rtl,  // Right-to-Left
}

#[derive(Clone, Copy)]
pub struct LanguageContext {
    pub current: RwSignal<Language>,
}

pub fn use_language() -> LanguageContext;
```

**DOM Effects:**
```html
<html lang="pt" dir="ltr">
<html lang="ar" dir="rtl">
```

---

## Provider Hierarchy

**Correct Order:**
```rust
<Router>
  <LanguageProvider initial_language>     // 1. Affects layout (dir)
    <ThemeProvider initial_mode initial_preset>  // 2. Affects tokens
      <DensityProvider initial_mode>      // 3. Affects sizes
        <ThemeEngine />                   // 4. Consumes all
        <App />
      </DensityProvider>
    </ThemeProvider>
  </LanguageProvider>
</Router>
```

**Why This Order?**
1. **Language first**: `dir="rtl"` affects layout fundamentally
2. **Theme second**: Color tokens depend on theme
3. **Density third**: Spacing/sizing depends on tokens
4. **ThemeEngine**: Computes final CSS variables from all contexts

---

## App Layer Responsibilities

### Server Functions (apps/*/src/theme_server.rs)

**Theme:**
```rust
#[server]
pub async fn set_theme_cookie(mode: String, preset: String) -> Result<(), ServerFnError>;

#[server]
pub async fn get_theme_from_cookie() -> Result<(String, String), ServerFnError>;
```

**Density:**
```rust
#[server]
pub async fn set_density_cookie(mode: String) -> Result<(), ServerFnError>;

#[server]
pub async fn get_density_from_cookie() -> Result<String, ServerFnError>;
```

**Language:**
```rust
#[server]
pub async fn set_language_cookie(lang: String) -> Result<(), ServerFnError>;

#[server]
pub async fn get_language_from_cookie() -> Result<String, ServerFnError>;
```

### SSR Hydration (apps/*/src/app.rs)
```rust
fn get_initial_theme() -> (String, String) {
    #[cfg(feature = "ssr")]
    {
        // Read from HTTP headers
        if let Some(headers) = use_context::<axum::http::HeaderMap>() {
            // Parse cookies...
        }
    }
    
    #[cfg(not(feature = "ssr"))]
    {
        ("system".to_string(), "default".to_string())
    }
}

fn get_initial_density() -> String { /* ... */ }
fn get_initial_language() -> String { /* ... */ }
```

### Anti-Flash Script (apps/*/src/app.rs)
```rust
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <head>
            <script>
                {r#"
                (function() {
                    const getCookie = (name) => { /* ... */ };
                    
                    const html = document.documentElement;
                    
                    // Theme
                    const mode = getCookie('theme-mode') || 'system';
                    const preset = getCookie('theme-preset') || 'default';
                    html.classList.add(resolveMode(mode));
                    html.setAttribute('data-theme', preset);
                    
                    // Density
                    const density = getCookie('density-mode') || 'comfortable';
                    html.setAttribute('data-density', density);
                    
                    // Language
                    const lang = getCookie('language') || 'en';
                    html.setAttribute('lang', lang);
                    html.setAttribute('dir', lang === 'ar' ? 'rtl' : 'ltr');
                })();
                "#}
            </script>
            <link rel="stylesheet" href="/app.css"/>
        </head>
    }
}
```

---

## Prohibited Provider Patterns

### ❌ Provider Inflation

**WRONG - Creating unnecessary providers:**
```rust
// ❌ FORBIDDEN
PreferencesProvider      // Too generic
UISettingsProvider       // Redundant with Theme/Density
AppearanceProvider       // Same as Theme
UserConfigProvider       // Business logic, not design
LayoutProvider           // Use CSS, not provider
ColorSchemeProvider      // Same as Theme
```

**Consequence:** Provider hell, unclear boundaries, circular dependencies.

### ❌ God Provider

**WRONG - Merging all providers:**
```rust
// ❌ FORBIDDEN
#[component]
pub fn DesignSystemProvider(
    theme: String,
    density: String,
    language: String,
    // ... 20 other props
) -> impl IntoView
```

**Consequence:** Cannot change theme without re-rendering density, etc.

### ❌ Persistence in Design System

**WRONG - Design system writing to cookies:**
```rust
// ❌ FORBIDDEN (in rs-design)
#[component]
pub fn ThemeProvider(...) -> impl IntoView {
    Effect::new(move |_| {
        // ❌ NO! This belongs in app layer
        document.cookie = "theme-mode=dark";
    });
}
```

**Consequence:** Design system knows about HTTP, breaks reusability.

### ❌ Business Logic in Providers

**WRONG - Provider calling APIs:**
```rust
// ❌ FORBIDDEN
#[component]
pub fn ThemeProvider(...) -> impl IntoView {
    let theme = Resource::new(|| (), |_| async {
        // ❌ NO! This is business logic
        fetch("/api/user/preferences").await
    });
}
```

**Consequence:** Design system coupled to backend.

---

## Provider Boundaries

| Concern | Design System | App Layer |
|---------|---------------|-----------|
| DOM attributes (`data-*`, `class`) | ✅ | ❌ |
| Context exposure | ✅ | ❌ |
| Type definitions (enums, structs) | ✅ | ❌ |
| Cookie reading (SSR) | ❌ | ✅ |
| Cookie writing (client) | ❌ | ✅ |
| Server functions | ❌ | ✅ |
| HTTP headers | ❌ | ✅ |
| Business logic | ❌ | ✅ |
| User preferences API | ❌ | ✅ |

---

## Migration from Legacy Patterns

### Before (Anti-Pattern)
```rust
// ❌ OLD: Provider with persistence
#[component]
pub fn ThemeProvider(children: Children) -> impl IntoView {
    let mode = RwSignal::new(ThemeMode::System);
    
    // ❌ Reading from localStorage in design system
    Effect::new(move |_| {
        if let Ok(Some(storage)) = window.local_storage() {
            if let Ok(Some(saved)) = storage.get_item("theme") {
                mode.set(parse_mode(&saved));
            }
        }
    });
    
    provide_context(ThemeContext { mode });
    view! { {children()} }
}
```

### After (Canon-Compliant)

**Design System:**
```rust
// ✅ NEW: Pure provider
#[component]
pub fn ThemeProvider(
    #[prop(optional)] initial_mode: Option<String>,
    children: Children,
) -> impl IntoView {
    let mode = RwSignal::new(
        initial_mode.map(|m| parse_mode(&m)).unwrap_or(ThemeMode::System)
    );
    
    // Apply to DOM only
    #[cfg(target_arch = "wasm32")]
    Effect::new(move |_| {
        let html = document.documentElement();
        html.set_attribute("data-theme", mode.get().as_str());
    });
    
    provide_context(ThemeContext { mode });
    view! { {children()} }
}
```

**App Layer:**
```rust
// ✅ NEW: App handles persistence
#[component]
pub fn App() -> impl IntoView {
    let (initial_mode, initial_preset) = get_theme_from_cookie_ssr();
    
    view! {
        <ThemeProvider initial_mode initial_preset>
            <App />
        </ThemeProvider>
    }
}
```

---

## Testing

### Provider Isolation Test
```rust
#[test]
fn test_providers_are_orthogonal() {
    // Change theme should not affect density
    let theme_ctx = ThemeContext::new();
    let density_ctx = DensityContext::new();
    
    theme_ctx.mode.set(ThemeMode::Dark);
    
    assert_eq!(density_ctx.mode.get(), DensityMode::Comfortable);
}
```

### SSR Hydration Test
```bash
# Verify cookies are read in SSR
curl -H "Cookie: theme-mode=dark; density-mode=compact; language=pt" \
     http://localhost:3000 | grep 'class="dark"'

# Should return HTML with:
# <html lang="pt" dir="ltr" class="dark" data-theme="default" data-density="compact">
```

### Anti-Flash Test

1. Set theme to dark
2. Hard refresh (Ctrl+Shift+R)
3. Watch first frame
4. **PASS:** No white flash, dark from frame 1

---

## Validation Checklist

- [ ] Only 3 providers exist (Theme, Density, Language)
- [ ] No provider reads/writes cookies directly
- [ ] No provider calls server functions
- [ ] Each provider applies exactly 1-2 DOM attributes
- [ ] Providers are composed in correct order
- [ ] SSR reads cookies in app layer
- [ ] Anti-flash script in app `shell()`
- [ ] No provider inflation (no "Settings", "Preferences", etc.)

---

## References

- [Canon Rule #32: Theme Persistence Contract](./canon-rule-32-theme-persistence-contract.md)
- [Canon Rule #33: Density & Accessibility Mapping](./canon-rule-33-density-accessibility-mapping.md)
- [Canon Rule #36: Component Compliance Levels](./canon-rule-36-component-compliance-levels.md)

---

**Enforcement:** Code review + Architecture linting  
**Providers:** Exactly 3 (Theme, Density, Language)  
**Boundaries:** Design System = DOM only | App = Persistence + HTTP
