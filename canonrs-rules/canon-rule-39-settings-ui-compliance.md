# Canon Rule #39: Settings UI Compliance

**Status:** ENFORCED

**Severity:** MEDIUM
**Scope:** state, ui, theming
**Version:** 1.0.0
**Date:** 2025-01-16


## Principle
Settings UI components (Theme Toggle, Density Selector, Language Picker) are **Level 1 (Strict)** components. They expose provider state changes via callbacks, do NOT persist themselves, and follow canonical token usage. Apps wire callbacks to server functions for persistence.

## The Three Settings Components

### Themesettingsdropdown

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






























































