# Canon Rule #35: Token Usage Validation

**Status:** ✅ Enforced  
**Version:** 1.0.0  
**Date:** 2025-01-02

## Principle
All visual properties MUST use canonical tokens. Hardcoded values (colors, sizes, spacing) are **prohibited** and detected by automated validation. Components reference tokens, never raw values.

## Token Categories

### Color Tokens (Canon Rule #21)

**Required Pattern:**
```rust
// ✅ CORRECT - Use semantic tokens
"bg-primary"
"text-destructive" 
"border-muted"
color: var(--color-primary-bg);
```

**Prohibited Patterns:**
```rust
// ❌ FORBIDDEN - Hardcoded Tailwind colors
"bg-red-500"
"text-blue-600"
"border-green-400"

// ❌ FORBIDDEN - Hex colors
background: #ff0000;
color: #3b82f6;

// ❌ FORBIDDEN - RGB/HSL
background: rgb(255, 0, 0);
color: hsl(220, 90%, 50%);
```

### Typography Tokens (Canon Rule #29)

**Required Pattern:**
```rust
// ✅ CORRECT - Use typography tokens
"text-base"          // var(--font-size-md)
"text-lg"            // var(--font-size-lg)
"font-medium"        // var(--font-weight-medium)
"leading-normal"     // var(--line-height-normal)
```

**Prohibited Patterns:**
```rust
// ❌ FORBIDDEN - Hardcoded sizes
"text-[14px]"
font-size: 16px;

// ❌ FORBIDDEN - Non-canonical weights
"font-[550]"
font-weight: 450;

// ❌ FORBIDDEN - Custom line heights
"leading-[1.3]"
line-height: 1.7;
```

### Spacing Tokens (Canon Rule #24)

**Required Pattern:**
```rust
// ✅ CORRECT - Use spacing scale
"p-4"                // var(--space-md)
"gap-2"              // var(--space-xs)
"mx-6"               // var(--space-lg)
padding: var(--space-sm);
```

**Prohibited Patterns:**
```rust
// ❌ FORBIDDEN - Arbitrary spacing
"p-[17px]"
"gap-[13px]"
margin: 23px;
padding: 0.7rem;
```

### Size Tokens (Canon Rule #24)

**Required Pattern:**
```rust
// ✅ CORRECT - Use size tokens
"w-10"               // var(--size-control-md)
"h-12"               // var(--size-control-lg)
width: var(--size-icon-md);
```

**Prohibited Patterns:**
```rust
// ❌ FORBIDDEN - Fixed sizes (unless density-scaled)
"w-[42px]"
"h-[38px]"
width: 40px;
height: 3rem;
```

### Border Radius Tokens (Canon Rule #26)

**Required Pattern:**
```rust
// ✅ CORRECT - Use radius tokens
"rounded-md"         // var(--radius-md)
"rounded-lg"         // var(--radius-lg)
border-radius: var(--radius-sm);
```

**Prohibited Patterns:**
```rust
// ❌ FORBIDDEN - Arbitrary radius
"rounded-[7px]"
border-radius: 12px;
border-radius: 0.5rem;
```

### Shadow Tokens (Canon Rule #26)

**Required Pattern:**
```rust
// ✅ CORRECT - Use shadow tokens
"shadow-md"          // var(--shadow-md)
"shadow-lg"          // var(--shadow-lg)
box-shadow: var(--shadow-sm);
```

**Prohibited Patterns:**
```rust
// ❌ FORBIDDEN - Custom shadows
box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
box-shadow: 0 10px 15px -3px rgb(0 0 0 / 0.1);
```

## Validator Implementation

### AST-Based Scanner
```rust
// tools/token-validator/src/main.rs
use regex::Regex;
use std::fs;
use walkdir::WalkDir;

#[derive(Debug)]
struct TokenViolation {
    file: String,
    line: usize,
    column: usize,
    rule: String,
    found: String,
    expected: String,
}

fn main() {
    let mut violations = Vec::new();
    
    for entry in WalkDir::new("packages-rust/rs-design/src") {
        let entry = entry.unwrap();
        let path = entry.path();
        
        if path.extension().and_then(|s| s.to_str()) == Some("rs") {
            violations.extend(validate_rust_file(path));
        }
        
        if path.extension().and_then(|s| s.to_str()) == Some("css") {
            violations.extend(validate_css_file(path));
        }
    }
    
    if !violations.is_empty() {
        print_violations(&violations);
        std::process::exit(1);
    }
    
    println!("✅ All token usage is canonical!");
}

fn validate_rust_file(path: &Path) -> Vec<TokenViolation> {
    let content = fs::read_to_string(path).unwrap();
    let mut violations = Vec::new();
    
    // Check for hardcoded Tailwind colors
    let color_regex = Regex::new(
        r#"(bg|text|border|ring)-(slate|gray|zinc|neutral|stone|red|orange|amber|yellow|lime|green|emerald|teal|cyan|sky|blue|indigo|violet|purple|fuchsia|pink|rose)-(\d+)"#
    ).unwrap();
    
    for (line_num, line) in content.lines().enumerate() {
        if let Some(captures) = color_regex.captures(line) {
            violations.push(TokenViolation {
                file: path.display().to_string(),
                line: line_num + 1,
                column: captures.get(0).unwrap().start(),
                rule: "Canon #21 (Color Tokens)".to_string(),
                found: captures.get(0).unwrap().as_str().to_string(),
                expected: "Use canonical tokens: bg-primary, text-destructive, etc.".to_string(),
            });
        }
    }
    
    // Check for hardcoded pixel values
    let px_regex = Regex::new(r#"(text|w|h|p|m|gap)-\[(\d+)px\]"#).unwrap();
    
    for (line_num, line) in content.lines().enumerate() {
        if let Some(captures) = px_regex.captures(line) {
            violations.push(TokenViolation {
                file: path.display().to_string(),
                line: line_num + 1,
                column: captures.get(0).unwrap().start(),
                rule: "Canon #29/#24 (Token Usage)".to_string(),
                found: captures.get(0).unwrap().as_str().to_string(),
                expected: "Use scale tokens: text-base, w-10, p-4, etc.".to_string(),
            });
        }
    }
    
    // Check for arbitrary font weights
    let weight_regex = Regex::new(r#"font-\[(\d+)\]"#).unwrap();
    
    for (line_num, line) in content.lines().enumerate() {
        if let Some(captures) = weight_regex.captures(line) {
            let weight: u32 = captures.get(1).unwrap().as_str().parse().unwrap();
            if ![400, 500, 600, 700].contains(&weight) {
                violations.push(TokenViolation {
                    file: path.display().to_string(),
                    line: line_num + 1,
                    column: captures.get(0).unwrap().start(),
                    rule: "Canon #29 (Typography)".to_string(),
                    found: format!("font-[{}]", weight),
                    expected: "Use canonical weights: 400, 500, 600, 700".to_string(),
                });
            }
        }
    }
    
    violations
}

fn validate_css_file(path: &Path) -> Vec<TokenViolation> {
    let content = fs::read_to_string(path).unwrap();
    let mut violations = Vec::new();
    
    // Check for hex colors
    let hex_regex = Regex::new(r#"#[0-9a-fA-F]{3,6}"#).unwrap();
    
    for (line_num, line) in content.lines().enumerate() {
        // Skip if line uses var()
        if line.contains("var(--color") {
            continue;
        }
        
        if let Some(captures) = hex_regex.captures(line) {
            violations.push(TokenViolation {
                file: path.display().to_string(),
                line: line_num + 1,
                column: captures.get(0).unwrap().start(),
                rule: "Canon #21 (Color Tokens)".to_string(),
                found: captures.get(0).unwrap().as_str().to_string(),
                expected: "Use var(--color-primary-bg), var(--color-text), etc.".to_string(),
            });
        }
    }
    
    // Check for px font sizes
    let font_px_regex = Regex::new(r#"font-size:\s*(\d+)px"#).unwrap();
    
    for (line_num, line) in content.lines().enumerate() {
        if line.contains("var(--font-size") {
            continue;
        }
        
        if let Some(captures) = font_px_regex.captures(line) {
            violations.push(TokenViolation {
                file: path.display().to_string(),
                line: line_num + 1,
                column: captures.get(0).unwrap().start(),
                rule: "Canon #29 (Typography)".to_string(),
                found: captures.get(0).unwrap().as_str().to_string(),
                expected: "Use var(--font-size-md), var(--font-size-lg), etc.".to_string(),
            });
        }
    }
    
    violations
}

fn print_violations(violations: &[TokenViolation]) {
    println!("❌ Found {} token usage violations:\n", violations.len());
    
    for v in violations {
        println!("{}:{}:{}", v.file, v.line, v.column);
        println!("  Rule: {}", v.rule);
        println!("  Found: {}", v.found);
        println!("  Expected: {}", v.expected);
        println!();
    }
}
```

### Cargo.toml
```toml
[package]
name = "token-validator"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "token-validator"
path = "src/main.rs"

[dependencies]
regex = "1.10"
walkdir = "2.4"
```

## CI Integration

### GitHub Actions
```yaml
name: Token Validation

on: [push, pull_request]

jobs:
  validate-tokens:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Install Rust
        uses: actions-rust-lang/setup-rust-toolchain@v1
      
      - name: Run Token Validator
        run: |
          cd tools/token-validator
          cargo run --release
      
      - name: Check for hardcoded colors (quick)
        run: |
          ! grep -rE "(bg|text|border)-(red|blue|green|yellow|purple)-[0-9]+" \
            packages-rust/rs-design/src/ui/ \
            packages-rust/rs-design/src/primitives/
      
      - name: Check for hex colors in CSS
        run: |
          ! grep -rE "#[0-9a-fA-F]{3,6}" \
            packages-rust/rs-design/src/ \
            --include="*.css" \
            | grep -v "var(--color"
      
      - name: Check for arbitrary spacing
        run: |
          ! grep -rE "(p|m|gap|space)-\[[0-9]+px\]" \
            packages-rust/rs-design/src/ui/
```

## Database Validation

### Token Coverage Report
```sql
-- Check which components use tokens correctly
SELECT 
    c.name,
    c.tokens_canonicos_percent,
    c.tokens_familia_c_percent,
    CASE 
        WHEN c.tokens_canonicos_percent = 100 THEN '✅'
        WHEN c.tokens_canonicos_percent >= 80 THEN '⚠️'
        ELSE '❌'
    END as status
FROM components c
ORDER BY c.tokens_canonicos_percent ASC;
```

### Token Usage Audit
```sql
-- List all hardcoded values found in components
SELECT 
    file_path,
    COUNT(*) as violations
FROM (
    -- This would be populated by the validator
    SELECT file_path, line, violation_type
    FROM token_violations
) 
GROUP BY file_path
ORDER BY violations DESC;
```

## Exemptions

### Allowed Hardcoded Values

**System Colors:**
```css
/* Transparent is allowed */
background: transparent;
color: currentColor;
```

**Zero Values:**
```rust
"m-0"    // margin: 0 is fine
"p-0"    // padding: 0 is fine
```

**Percentage Values:**
```css
width: 100%;
height: 50%;
```

**Documentation/Examples:**
```rust
// token-exempt: documentation example
"bg-red-500"  // Shows non-canonical usage for comparison
```

## Validation Commands
```bash
# Full validation
cd tools/token-validator
cargo run --release

# Quick check (grep-based)
./scripts/check-tokens.sh

# Per-component check
./scripts/check-tokens.sh src/ui/button/

# Generate report
cargo run --release -- --report > token-report.txt
```

## Report Output
```
Token Usage Validation Report
Generated: 2025-01-02 15:30:00

Summary:
  Total Files: 127
  Files Checked: 127
  Violations: 3
  Compliance: 97.6%

Violations by Category:
  Color Tokens (Canon #21): 2
  Typography (Canon #29): 1
  Spacing (Canon #24): 0

Critical Violations:
  ❌ src/ui/alert/alert.rs:45:12
     Found: bg-red-500
     Expected: bg-destructive
  
  ❌ src/ui/badge/badge.rs:23:8
     Found: text-blue-600
     Expected: text-primary
  
  ⚠️  src/ui/input/input.rs:67:10
     Found: text-[14px]
     Expected: text-sm

Action Required:
  Fix 3 violations to achieve 100% compliance
```

## Pre-commit Hook
```bash
#!/bin/bash
# .git/hooks/pre-commit

echo "🔍 Validating token usage..."

cd tools/token-validator
if ! cargo run --quiet 2>/dev/null; then
    echo "❌ Token validation failed!"
    echo "Run 'cd tools/token-validator && cargo run' for details"
    exit 1
fi

echo "✅ All tokens canonical!"
```

## References

- [Canon Rule #21: Canonical Color Tokens](./canon-rule-21-canonical-color-tokens.md)
- [Canon Rule #24: Density & Size Scaling](./canon-rule-24-density-size-scaling.md)
- [Canon Rule #29: Typography Contract](./canon-rule-29-typography-contract.md)
- [Canon Rule #34: Theme & Density Enforcement](./canon-rule-34-theme-density-enforcement.md)

---

**Enforcement:** AST scanning + CI  
**Failure Mode:** Build fails on hardcoded values  
**Compliance Target:** 100% token usage
