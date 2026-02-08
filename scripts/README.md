# CanonRS Scripts

## 🎯 Core Scripts

### `generate-families.sh`
**Role:** Generate CSS from Rust family tokens

**What it does:**
- Runs tokens-engine to generate complete CSS cascade
- Generates 10 family CSS files in `canonrs-ui/styles/.generated/`

**When to run:** After modifying family tokens
```bash
./scripts/core/generate-families.sh
```

---

### `bundle-css.sh`
**Role:** Bundle canonrs.css into single file

**What it does:**
- Resolves all @import recursively
- Creates `canonrs-ui/styles/canonrs.bundle.css`
- Single file ready for production

**When to run:** Before product build
```bash
./scripts/core/bundle-css.sh
```

---

### `validate-token-usage.sh`
**Role:** Token governance (CI gate)

**What it does:**
1. Validates all CSS tokens exist
2. Ensures UI/blocks use only public family tokens
3. Blocks hardcoded literals

**When to run:** CI pipeline
```bash
cargo build
./scripts/core/validate-token-usage.sh
```

---

## 🚀 Complete Workflow

### Development
```bash
# 1. Edit family tokens
vim canonrs-ssr/src/design/tokens/families/family_a_overlay.rs

# 2. Regenerate CSS
./scripts/core/generate-families.sh

# 3. Bundle
./scripts/core/bundle-css.sh

# 4. Validate
cargo build
./scripts/core/validate-token-usage.sh
```

### Product Build
```bash
# products/canonrs-site/build.sh
cd ../../packages-rust/rs-canonrs
./scripts/core/bundle-css.sh
cp canonrs-ui/styles/canonrs.bundle.css ../../products/canonrs-site/public/canonrs.css
cd ../../products/canonrs-site
cargo leptos build
```

---

## 📐 Architecture
```
Rust Families → tokens-engine → .generated/*.css → 
canonrs.css → bundle-css.sh → canonrs.bundle.css → 
product build step → public/canonrs.css
```
