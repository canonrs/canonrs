# Canon Rule #34: Theme & Density Enforcement (Lint Rules)

**Status:** ✅ Enforced  
**Version:** 1.0.0  
**Date:** 2025-01-02

## Principle
Theme and density rules are **enforced by automated linting** at build time. Code that violates Canon Rules #32 and #33 must not compile or pass CI.

## Prohibited Patterns (AST-level)

### ❌ localStorage for Theme

**Rule:** Theme state MUST use cookies (Canon Rule #32)
```rust
// FORBIDDEN - AST pattern to detect
localStorage.setItem("theme", ...)
localStorage.getItem("theme")

// ALLOWED
document.cookie = "theme-mode=..."
```

**Lint Check:**
```rust
// tools/canon-linter/src/rules/no_localstorage_theme.rs
pub fn check_localStorage_usage(file: &str) -> Vec<Violation> {
    let forbidden = ["localStorage.setItem", "localStorage.getItem"];
    // Scan for forbidden patterns in theme-related files
}
```

### ❌ Fixed Pixel Sizes in Density-Aware Components

**Rule:** Interactive elements MUST scale with density (Canon Rule #33)
```rust
// FORBIDDEN
"h-10"          // Fixed 40px
"h-[40px]"      // Hardcoded pixels
"height: 40px"  // Direct CSS

// ALLOWED
"h-[calc(var(--size-control-md)*var(--density-multiplier))]"
```

**Lint Check:**
```bash
# Grep-based check
grep -r "h-\[.*px\]" src/ui/ && exit 1
grep -r "height:.*px" src/ui/ && exit 1
```

### ❌ Non-Canonical Color Variants

**Rule:** Components MUST NOT use semantic color variants (Canon Rule #21)
```rust
// FORBIDDEN
ButtonVariant::Success
ButtonVariant::Warning
ButtonVariant::Info
AlertVariant::Success

// ALLOWED
ButtonVariant::Primary
ButtonVariant::Destructive
```

**Lint Check:**
```rust
// Scan for enum variants
if code.contains("Variant::Success") 
|| code.contains("Variant::Warning") 
|| code.contains("Variant::Info") {
    return Err("Non-canonical variant");
}
```

### ❌ Hardcoded Colors

**Rule:** Components MUST use color tokens (Canon Rule #21)
```rust
// FORBIDDEN
"bg-red-500"
"text-blue-600"
"border-green-400"
"background: #ff0000"

// ALLOWED
"bg-primary"
"text-destructive"
"border-muted"
```

**Lint Check:**
```bash
# Regex for Tailwind color classes with numbers
grep -rE "bg-(red|blue|green|yellow|purple)-[0-9]+" src/ui/ && exit 1
grep -rE "text-(red|blue|green|yellow|purple)-[0-9]+" src/ui/ && exit 1

# Hex colors in CSS
grep -rE "#[0-9a-fA-F]{3,6}" src/ui/ && exit 1
```

### ❌ Typography Scaling with Density

**Rule:** Font sizes MUST NOT scale with density (Canon Rule #33)
```rust
// FORBIDDEN
"text-[calc(var(--font-size-md)*var(--density-multiplier))]"

// ALLOWED
"text-base"  // Uses var(--font-size-md) without multiplier
```

**Lint Check:**
```bash
grep -r "font-size.*density-multiplier" src/ && exit 1
grep -r "text-\[calc.*density" src/ && exit 1
```

## Allowed Patterns (Whitelist)

### ✅ Cookie-Based Persistence
```rust
// theme_server.rs (app layer)
#[server]
pub async fn set_theme_cookie(mode: String, preset: String) -> Result<(), ServerFnError> {
    response.append_header(SET_COOKIE, ...);
}
```

### ✅ Density-Scaled Spacing
```rust
// Component with density awareness
"h-[calc(var(--size-control-md)*var(--density-multiplier))]"
"px-[calc(var(--space-lg)*var(--density-multiplier))]"
```

### ✅ Canonical Tokens
```rust
"bg-primary"
"text-destructive"
"border-muted"
```

## Linter Implementation

### File: `tools/canon-linter/src/main.rs`
```rust
use std::fs;
use std::path::Path;
use regex::Regex;

#[derive(Debug)]
struct Violation {
    file: String,
    line: usize,
    rule: String,
    message: String,
}

fn main() {
    let mut violations = Vec::new();
    
    // Check all .rs files in src/
    for entry in walkdir::WalkDir::new("src") {
        let entry = entry.unwrap();
        if entry.path().extension().and_then(|s| s.to_str()) == Some("rs") {
            violations.extend(check_file(entry.path()));
        }
    }
    
    if !violations.is_empty() {
        println!("❌ Found {} Canon Rule violations:", violations.len());
        for v in &violations {
            println!("{}:{} [{}] {}", v.file, v.line, v.rule, v.message);
        }
        std::process::exit(1);
    }
    
    println!("✅ All Canon Rules passed!");
}

fn check_file(path: &Path) -> Vec<Violation> {
    let content = fs::read_to_string(path).unwrap();
    let mut violations = Vec::new();
    
    // Rule #32: No localStorage for theme
    if path.to_str().unwrap().contains("theme") {
        for (i, line) in content.lines().enumerate() {
            if line.contains("localStorage") {
                violations.push(Violation {
                    file: path.display().to_string(),
                    line: i + 1,
                    rule: "Canon #32".to_string(),
                    message: "Use cookies, not localStorage for theme".to_string(),
                });
            }
        }
    }
    
    // Rule #33: No fixed px in density-aware components
    if path.to_str().unwrap().contains("src/ui/") {
        let px_regex = Regex::new(r#"(h|w|height|width)-\[?\d+px\]?"#).unwrap();
        for (i, line) in content.lines().enumerate() {
            if px_regex.is_match(line) {
                violations.push(Violation {
                    file: path.display().to_string(),
                    line: i + 1,
                    rule: "Canon #33".to_string(),
                    message: "Use density-scaled sizes, not fixed pixels".to_string(),
                });
            }
        }
    }
    
    // Rule #21: No non-canonical color variants
    let forbidden_variants = ["Success", "Warning", "Info", "Danger"];
    for variant in &forbidden_variants {
        if content.contains(&format!("Variant::{}", variant)) {
            violations.push(Violation {
                file: path.display().to_string(),
                line: 0,
                rule: "Canon #21".to_string(),
                message: format!("Use canonical variants (Primary, Destructive), not {}", variant),
            });
        }
    }
    
    // Rule #21: No hardcoded Tailwind colors
    let color_regex = Regex::new(r#"(bg|text|border)-(red|blue|green|yellow|purple|pink|orange)-\d+"#).unwrap();
    for (i, line) in content.lines().enumerate() {
        if color_regex.is_match(line) {
            violations.push(Violation {
                file: path.display().to_string(),
                line: i + 1,
                rule: "Canon #21".to_string(),
                message: "Use canonical color tokens (bg-primary, text-destructive, etc)".to_string(),
            });
        }
    }
    
    violations
}
```

### File: `tools/canon-linter/Cargo.toml`
```toml
[package]
name = "canon-linter"
version = "0.1.0"
edition = "2021"

[dependencies]
regex = "1"
walkdir = "2"
```

## CI Integration

### GitHub Actions: `.github/workflows/canon-compliance.yml`
```yaml
name: Canon Rules Compliance

on: [push, pull_request]

jobs:
  canon-lint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Install Rust
        uses: actions-rust-lang/setup-rust-toolchain@v1
      
      - name: Run Canon Linter
        run: |
          cd packages-rust/rs-design
          cargo run --bin canon-linter
      
      - name: Check for localStorage in theme files
        run: |
          ! grep -r "localStorage" packages-rust/rs-design/src/providers/
      
      - name: Check for hardcoded colors
        run: |
          ! grep -rE "(bg|text|border)-(red|blue|green)-[0-9]+" packages-rust/rs-design/src/ui/
      
      - name: Check for fixed pixels
        run: |
          ! grep -rE "h-\[[0-9]+px\]" packages-rust/rs-design/src/ui/
```

## Pre-commit Hook

### File: `.git/hooks/pre-commit`
```bash
#!/bin/bash
set -e

echo "🔍 Running Canon Rules checks..."

# Quick grep checks
if grep -r "localStorage" packages-rust/rs-design/src/providers/ 2>/dev/null; then
    echo "❌ Canon #32 violated: localStorage found in theme code"
    exit 1
fi

if grep -rE "(bg|text)-(red|blue|green)-[0-9]+" packages-rust/rs-design/src/ui/ 2>/dev/null; then
    echo "❌ Canon #21 violated: Hardcoded Tailwind colors found"
    exit 1
fi

echo "✅ Canon Rules passed!"
```

## Validation Commands
```bash
# Run linter
cd packages-rust/rs-design
cargo run --bin canon-linter

# Quick checks
./scripts/check-canon.sh

# Full validation
cargo test canon_compliance
```

## Exemptions

### When to Allow Violations

**Documentation/Examples:**
```rust
// canon-exempt: example code
localStorage.setItem("theme", "dark");
```

**Tests:**
```rust
#[cfg(test)]
mod tests {
    // Tests can violate rules
}
```

**Migration Code:**
```rust
// canon-exempt: migration from localStorage to cookies
if let Some(legacy) = localStorage.getItem("theme") {
    set_cookie(legacy);
}
```

## References

- [Canon Rule #21: Canonical Color Tokens](./canon-rule-21-canonical-color-tokens.md)
- [Canon Rule #32: Theme Persistence Contract](./canon-rule-32-theme-persistence-contract.md)
- [Canon Rule #33: Density & Accessibility Mapping](./canon-rule-33-density-accessibility-mapping.md)

---

**Enforcement:** Automated linting + CI  
**Failure Mode:** Build fails on violation  
**Override:** Explicit `// canon-exempt` comment required
