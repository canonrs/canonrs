# Canon Rule #39: Settings UI Compliance

**Status:** ✅ Enforced  
**Version:** 1.0.0  
**Date:** 2025-01-02

## Principle
Settings UI components (Theme Toggle, Density Selector, Language Picker) are **Level 1 (Strict)** components. They expose provider state changes via callbacks, do NOT persist themselves, and follow canonical token usage. Apps wire callbacks to server functions for persistence.

## The Three Settings Components

### 1️⃣ ThemeSettingsDropdown

**Responsibility:**
- Display current theme mode (Light/Dark/System)
- Display available theme presets
- Allow user to change mode and preset
- Notify parent via callback when changed

**Does NOT:**
- ❌ Persist to cookies
- ❌ Call server functions
- ❌ Read from localStorage
- ❌ Know about HTTP

**Location:** `packages-rust/rs-design/src/ui/theme_toggle/theme_settings_dropdown.rs`

**Component Signature:**
```rust
#[component]
pub fn ThemeSettingsDropdown(
    #[prop(into, optional)] class: String,
    
    /// Callback when theme changes (app handles persistence)
    #[prop(optional)]
    on_theme_change: Option<Callback<(String, String)>>,
    
    /// Callback when density changes (app handles persistence)
    #[prop(optional)]
    on_density_change: Option<Callback<String>>,
) -> impl IntoView
```

**Usage in App:**
```rust
let on_theme_change = Callback::new(move |(mode, preset): (String, String)| {
    spawn_local(async move {
        let _ = set_theme_cookie(mode, preset).await;
    });
});

let on_density_change = Callback::new(move |mode: String| {
    spawn_local(async move {
        let _ = set_density_cookie(mode).await;
    });
});

view! {
    <ThemeSettingsDropdown 
        on_theme_change=on_theme_change
        on_density_change=on_density_change
    />
}
```

**Internal Implementation:**
```rust
use crate::providers::{use_theme, use_density, DensityMode};
use crate::themes::ThemeRegistry;

#[component]
pub fn ThemeSettingsDropdown(
    #[prop(into, optional)] class: String,
    #[prop(optional)] on_theme_change: Option<Callback<(String, String)>>,
    #[prop(optional)] on_density_change: Option<Callback<String>>,
) -> impl IntoView {
    let theme = use_theme();
    let density = use_density();
    let is_open = RwSignal::new(false);
    let presets = ThemeRegistry::available_presets();
    
    let set_mode = move |new_mode| {
        theme.mode.set(new_mode);
        
        if let Some(callback) = on_theme_change {
            let mode_str = match new_mode {
                ThemeMode::Light => "light",
                ThemeMode::Dark => "dark",
                ThemeMode::System => "system",
            };
            callback.run((mode_str.to_string(), theme.preset.get()));
        }
    };
    
    let set_preset = move |preset_id: String| {
        theme.preset.set(preset_id.clone());
        
        if let Some(callback) = on_theme_change {
            let mode_str = match theme.mode.get() {
                ThemeMode::Light => "light",
                ThemeMode::Dark => "dark",
                ThemeMode::System => "system",
            };
            callback.run((mode_str, preset_id));
        }
    };
    
    let set_density = move |new_density: DensityMode| {
        density.mode.set(new_density);
        
        if let Some(callback) = on_density_change {
            callback.run(new_density.as_str().to_string());
        }
    };
    
    view! {
        <div class=format!("relative {}", class)>
            <Button on:click=move |_| is_open.update(|o| *o = !*o)>
                <ThemeIcon mode=theme.mode />
            </Button>
            
            <Show when=move || is_open.get()>
                <div class="absolute right-0 mt-2 w-64 rounded-md border bg-popover p-4 shadow-lg">
                    // Mode buttons (Light/Dark/System)
                    // Preset selector dropdown
                    // Density buttons (Compact/Comfortable/Spacious)
                </div>
            </Show>
        </div>
    }
}
```

**Canonical Token Usage:**
```rust
// ✅ CORRECT - Uses canonical tokens
"bg-popover"
"text-popover-foreground"
"border-border"
"hover:bg-accent"

// ❌ FORBIDDEN
"bg-gray-800"
"text-blue-500"
```

**Compliance Level:** Strict (100%)

---

### 2️⃣ DensitySelector (Standalone)

**Responsibility:**
- Display current density mode
- Allow user to change density
- Notify parent via callback

**Location:** `packages-rust/rs-design/src/ui/density/density_selector.rs`

**Component Signature:**
```rust
#[component]
pub fn DensitySelector(
    #[prop(into, optional)] class: String,
    #[prop(optional)] on_change: Option<Callback<String>>,
) -> impl IntoView {
    let density = use_density();
    
    let set_density = move |mode: DensityMode| {
        density.mode.set(mode);
        
        if let Some(callback) = on_change {
            callback.run(mode.as_str().to_string());
        }
    };
    
    view! {
        <div class=format!("flex gap-2 {}", class)>
            <Button 
                size=ButtonSize::Sm
                variant=move || if density.mode.get() == DensityMode::Compact {
                    ButtonVariant::Solid
                } else {
                    ButtonVariant::Ghost
                }
                on:click=move |_| set_density(DensityMode::Compact)
            >
                "Compact"
            </Button>
            
            <Button 
                size=ButtonSize::Sm
                variant=move || if density.mode.get() == DensityMode::Comfortable {
                    ButtonVariant::Solid
                } else {
                    ButtonVariant::Ghost
                }
                on:click=move |_| set_density(DensityMode::Comfortable)
            >
                "Comfortable"
            </Button>
            
            <Button 
                size=ButtonSize::Sm
                variant=move || if density.mode.get() == DensityMode::Spacious {
                    ButtonVariant::Solid
                } else {
                    ButtonVariant::Ghost
                }
                on:click=move |_| set_density(DensityMode::Spacious)
            >
                "Spacious"
            </Button>
        </div>
    }
}
```

**Compliance Level:** Strict (100%)

---

### 3️⃣ LanguageToggle

**Responsibility:**
- Display current language
- Show available languages
- Allow user to switch language
- Notify parent via callback

**Location:** `packages-rust/rs-design/src/blocks/language_toggle.rs`

**Component Signature:**
```rust
#[component]
pub fn LanguageToggle(
    #[prop(optional)] on_language_change: Option<Callback<String>>,
) -> impl IntoView {
    let lang_ctx = use_language();
    let current_lang = lang_ctx.current;
    
    let languages = vec![
        ("en", "English"),
        ("pt", "Português"),
        ("es", "Español"),
        ("ar", "العربية"),
    ];
    
    let change_language = move |lang_code: String| {
        let new_lang = Language::new(&lang_code);
        current_lang.set(new_lang);
        
        if let Some(callback) = on_language_change {
            callback.run(lang_code);
        }
    };
    
    view! {
        <div class="flex gap-2">
            {languages.into_iter().map(|(code, name)| {
                let code_owned = code.to_string();
                let is_active = move || current_lang.get().code == code_owned;
                
                view! {
                    <Button
                        variant=ButtonVariant::Ghost
                        size=ButtonSize::Sm
                        on:click=move |_| change_language(code_owned.clone())
                    >
                        {name}
                        {move || {
                            if is_active() {
                                view! { <span class="ml-1">"✓"</span> }.into_any()
                            } else {
                                view! { <span></span> }.into_any()
                            }
                        }}
                    </Button>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}
```

**Compliance Level:** Strict (100%)

---

## Callback Contract

### Theme Callback Signature
```rust
Callback<(String, String)>
// Tuple: (mode, preset)
// mode: "light" | "dark" | "system"
// preset: "default" | "ocean" | "amber" | ...
```

**Example:**
```rust
callback.run(("dark".to_string(), "ocean".to_string()));
```

### Density Callback Signature
```rust
Callback<String>
// mode: "compact" | "comfortable" | "spacious"
```

**Example:**
```rust
callback.run("compact".to_string());
```

### Language Callback Signature
```rust
Callback<String>
// lang_code: "en" | "pt" | "es" | "ar"
```

**Example:**
```rust
callback.run("pt".to_string());
```

---

## App Integration Pattern

### Centralized Settings Page
```rust
#[component]
pub fn SettingsPage() -> impl IntoView {
    // Theme callback
    let on_theme_change = Callback::new(move |(mode, preset): (String, String)| {
        spawn_local(async move {
            let _ = set_theme_cookie(mode, preset).await;
        });
    });
    
    // Density callback
    let on_density_change = Callback::new(move |mode: String| {
        spawn_local(async move {
            let _ = set_density_cookie(mode).await;
        });
    });
    
    // Language callback
    let on_language_change = Callback::new(move |lang: String| {
        spawn_local(async move {
            let _ = set_language_cookie(lang).await;
        });
    });
    
    view! {
        <div class="space-y-6">
            <div class="space-y-2">
                <Label>"Theme"</Label>
                <ThemeSettingsDropdown 
                    on_theme_change=on_theme_change
                    on_density_change=on_density_change
                />
            </div>
            
            <div class="space-y-2">
                <Label>"Density"</Label>
                <DensitySelector on_change=on_density_change />
            </div>
            
            <div class="space-y-2">
                <Label>"Language"</Label>
                <LanguageToggle on_language_change=on_language_change />
            </div>
        </div>
    }
}
```

### Inline Settings (Toolbar)
```rust
#[component]
pub fn AppToolbar() -> impl IntoView {
    let on_theme = Callback::new(|(m, p)| { /* ... */ });
    let on_density = Callback::new(|m| { /* ... */ });
    
    view! {
        <div class="flex items-center gap-2">
            <ThemeSettingsDropdown 
                on_theme_change=on_theme
                on_density_change=on_density
            />
        </div>
    }
}
```

---

## Prohibited Patterns

### ❌ Settings Component with Persistence

**WRONG - Component writing to cookies:**
```rust
// ❌ FORBIDDEN
#[component]
pub fn ThemeToggle() -> impl IntoView {
    let theme = use_theme();
    
    let toggle = move |_| {
        theme.mode.set(ThemeMode::Dark);
        
        // ❌ NO! This belongs in app layer
        document.cookie = "theme-mode=dark";
    };
}
```

### ❌ Settings Component Calling Server Functions

**WRONG - Component calling server directly:**
```rust
// ❌ FORBIDDEN
#[component]
pub fn DensityPicker() -> impl IntoView {
    let set_compact = move |_| {
        spawn_local(async {
            // ❌ NO! App should handle this via callback
            set_density_cookie("compact".to_string()).await;
        });
    };
}
```

### ❌ Settings Without Callbacks

**WRONG - No way for app to persist:**
```rust
// ❌ FORBIDDEN
#[component]
pub fn LanguageSwitcher() -> impl IntoView {
    let lang = use_language();
    
    // ❌ Changes provider but app can't persist!
    let switch = move |_| {
        lang.current.set(Language::new("pt"));
    };
}
```

**Consequence:** Settings change but don't persist across page reloads.

### ❌ Non-Canonical Token Usage

**WRONG - Hardcoded colors:**
```rust
// ❌ FORBIDDEN
view! {
    <button class="bg-blue-500 hover:bg-blue-600">  {/* ❌ */}
        "Light"
    </button>
}

// ✅ CORRECT
view! {
    <Button variant=ButtonVariant::Solid>
        "Light"
    </Button>
}
```

---

## Accessibility Requirements

### Keyboard Navigation
```rust
// ✅ All buttons must be keyboard accessible
<Button on:click=set_mode>
    // Automatically gets tabindex, Enter/Space handlers
</Button>
```

### ARIA Labels
```rust
<Button 
    aria-label="Switch to dark mode"
    on:click=move |_| set_mode(ThemeMode::Dark)
>
    <MoonIcon />
</Button>
```

### Screen Reader Announcements
```rust
// Announce when theme changes
Effect::new(move |_| {
    let mode = theme.mode.get();
    announce_to_screen_reader(&format!("Theme changed to {:?}", mode));
});
```

### Focus Management
```rust
// Dropdown should trap focus when open
let is_open = RwSignal::new(false);

view! {
    <Show when=move || is_open.get()>
        <div 
            role="dialog"
            aria-modal="true"
            on:keydown=move |e| {
                if e.key() == "Escape" {
                    is_open.set(false);
                }
            }
        >
            // Settings UI
        </div>
    </Show>
}
```

---

## Visual States

### Active State
```rust
// Show which option is currently selected
let is_dark = move || theme.mode.get() == ThemeMode::Dark;

view! {
    <Button 
        variant=move || if is_dark() {
            ButtonVariant::Solid  // Active
        } else {
            ButtonVariant::Ghost  // Inactive
        }
    >
        "Dark"
    </Button>
}
```

### Hover State
```rust
// Canonical hover via ButtonVariant (automatic)
<Button variant=ButtonVariant::Ghost>
    // hover:bg-accent applied automatically
</Button>
```

### Disabled State (if applicable)
```rust
<Button 
    disabled=move || some_condition.get()
    variant=ButtonVariant::Solid
>
    "Premium Theme"
</Button>
```

---

## Testing

### Unit Test: Callback Invocation
```rust
#[test]
fn test_theme_toggle_calls_callback() {
    let called = RwSignal::new(false);
    let callback = Callback::new(move |(_mode, _preset)| {
        called.set(true);
    });
    
    // Render component with callback
    let _ = view! {
        <ThemeSettingsDropdown on_theme_change=callback />
    };
    
    // Simulate click on Dark button
    // ...
    
    assert!(called.get());
}
```

### Integration Test: Persistence Flow
```rust
#[test]
fn test_settings_persist_via_callback() {
    let app = spawn_app();
    
    // Click theme toggle
    app.click_button("Dark Mode");
    
    // Wait for server function
    app.wait_for_cookie("theme-mode", "dark");
    
    // Reload page
    app.reload();
    
    // Verify theme persisted
    assert_eq!(app.get_html_class(), "dark");
}
```

### Visual Regression Test
```rust
#[test]
fn test_settings_dropdown_appearance() {
    let screenshot = render_component(ThemeSettingsDropdown);
    assert_matches_snapshot(screenshot, "theme-settings-dropdown");
}
```

---

## Validation Checklist

- [ ] Component uses `use_theme()`, `use_density()`, or `use_language()`
- [ ] Component exposes `on_*_change` callback
- [ ] Component does NOT persist state directly
- [ ] Component uses canonical tokens (no hardcoded colors)
- [ ] Component is keyboard accessible
- [ ] Component has ARIA labels
- [ ] Component shows active state clearly
- [ ] App wires callback to server function
- [ ] Settings persist across page reloads
- [ ] Compliance level: Strict (100%)

---

## References

- [Canon Rule #21: Canonical Color Tokens](./canon-rule-21-canonical-color-tokens.md)
- [Canon Rule #31: Accessibility Contract](./canon-rule-31-accessibility-contract.md)
- [Canon Rule #32: Theme Persistence Contract](./canon-rule-32-theme-persistence-contract.md)
- [Canon Rule #33: Density & Accessibility Mapping](./canon-rule-33-density-accessibility-mapping.md)
- [Canon Rule #36: Component Compliance Levels](./canon-rule-36-component-compliance-levels.md)
- [Canon Rule #37: Provider Taxonomy & Boundaries](./canon-rule-37-provider-taxonomy-boundaries.md)
- [Canon Rule #38: Theme Engine Contract](./canon-rule-38-theme-engine-contract.md)

---

**Enforcement:** Code review + Component audit  
**Compliance:** Strict (Level 1)  
**Callbacks:** Required for all settings changes  
**Persistence:** App layer only, never in components
