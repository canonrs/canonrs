# Tokens Flow - CSS Generation Pipeline

## 1. Execute
```bash
cd /opt/docker/monorepo/packages-rust/rs-canonrs
cargo run --bin tokens-engine
```

---

## 2. Input Directories

### A. Rust Token Families
```
canonrs-tokens/src/design/tokens/families/
├── family_a_overlay.rs
├── family_b_selection.rs
├── family_c_forms.rs
├── family_d_navigation.rs
├── family_e_feedback.rs
├── family_f_data.rs
├── family_g_composite.rs
├── family_h_layout.rs
├── family_i_animation.rs
├── family_s_state.rs
└── family_z_layers.rs
```

### B. Theme Definitions
```
canonrs-ui/themes-engine/ingest/css/
├── canonrs-theme.css
├── amber-minimal.css
└── clean-slate.css
```

### C. Base Tokens
```
canonrs-ui/styles/tokens/
├── base/
│   ├── globals.css
│   ├── core.css
│   ├── ui.css
│   ├── layout.css
│   └── blocks.css
└── semantic.css
```

### D. Theme Files
```
canonrs-ui/styles/themes/
├── light/ui.css
└── dark/ui.css
```

### E. Variants
```
canonrs-ui/styles/variants/
├── density-comfortable.css
├── density-compact.css
├── density-spacious.css
├── size-lg.css
├── size-md.css
└── size-sm.css
```

### F. UI Components
```
canonrs-ui/styles/ui/
├── accordion_ui.css
├── alert_ui.css
├── button_ui.css
└── ... (82+ components)
```

---

## 3. Processing Steps

### Step 1: Generate Families (theme_generator.rs)
- Reads Rust token structs
- Generates CSS variables
- Parses theme definitions
- Maps semantic tokens

**Generates:**
```
canonrs-ui/styles/.generated/
├── family-a-overlay.css       (4.8KB)
├── family-b-selection.css     (7.6KB)
├── family-c-forms.css         (13KB)
├── family-d-navigation.css    (5.2KB)
├── family-e-feedback.css      (8.9KB)
├── family-f-data.css          (7.8KB)
├── family-g-composite.css     (1.5KB)
├── family-h-layout.css        (5.9KB)
├── family-i-animation.css     (635B)
├── family-s-state.css         (457B)
├── family-z-layers.css        (327B)
└── themes.css                 (17KB)
```

### Step 2: Generate Entry (entry_generator.rs)
- Scans all CSS files
- Creates @import structure
- Orders dependencies correctly

**Generates:**
```
canonrs-ui/styles/
└── canonrs.css                (3.7KB)
```

### Step 3: Bundle (bundler.rs)
- Resolves all @imports recursively
- Concatenates files
- Creates single-file output

**Generates:**
```
canonrs-ui/styles/
└── canonrs.bundle.css         (315KB)
```

---

## 4. Output Verification

### Generated Files

**Directory 1: `.generated/`**
```bash
ls -lh canonrs-ui/styles/.generated/
# Should show:
# - 11 family-*.css files
# - 1 themes.css file
# Total: 12 files
```

**Directory 2: `styles/`**
```bash
ls -lh canonrs-ui/styles/canonrs*.css
# Should show:
# - canonrs.css (~4KB)
# - canonrs.bundle.css (~315KB)
```

---

## 5. Usage in Apps

### Development (fast reload)
```html
<link rel="stylesheet" href="/canonrs.css">
```

### Production (single file)
```html
<link rel="stylesheet" href="/canonrs.bundle.css">
```

### Leptos
```rust
use leptos_meta::Stylesheet;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Stylesheet href="/canonrs.css"/>
        // ...
    }
}
```

---

## 6. Workflow Summary
```
Rust Tokens → tokens-engine → .generated/*.css
                            ↓
Theme Ingest → parse → themes.css
                            ↓
All Sources → entry_generator → canonrs.css
                            ↓
Resolve @imports → bundler → canonrs.bundle.css
```

---

## 7. Quick Validation
```bash
# Check families generated
test -f canonrs-ui/styles/.generated/family-a-overlay.css && echo "✅ Families OK"

# Check entry created
test -f canonrs-ui/styles/canonrs.css && echo "✅ Entry OK"

# Check bundle created
test -f canonrs-ui/styles/canonrs.bundle.css && echo "✅ Bundle OK"

# Check bundle size
wc -l canonrs-ui/styles/canonrs.bundle.css
# Should be 8000+ lines
```
