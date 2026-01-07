# Canon Rule #40: Legacy Exception & Annotation Contract

**Status:** ✅ Mandatory  
**Category:** Governance / Enforcement  
**Automation:** Partial (Lint + Validator)  
**CI Block:** Conditional  
**Version:** 1.0.0  
**Date:** 2025-01-02

---

## 🎯 Objective

Allow **explicit, trackable, and temporary exceptions** to canonical rules without breaking governance, without hiding technical debt, and without enabling anarchy.

⚠️ **Exceptions are not ignored violations**  
✅ **They are documented contracts with deadline, owner, and justification**

---

## 🧱 Fundamental Principle

**Nothing can violate the Canon without leaving a trace.**

If something:
- Uses forbidden token
- Uses arbitrary pixels
- Breaks provider rule
- Maintains unavoidable legacy

👉 **It must be annotated.**

---

## 🧩 Canonical Annotation Format

The annotation must exist at the top of the file, before any code.

### 📌 Mandatory Format
```rust
//! @canon-level: loose
//! @canon-exceptions: [#21, #24]
//! @canon-justification: Legacy component prior to token system
//! @canon-owner: ui-team
//! @canon-target-date: 2025-03-01
//! @canon-migration-status: planned
```

### 🧾 Fields Contract

| Field | Required | Description |
|-------|----------|-------------|
| `@canon-level` | ✅ | `strict`, `standard`, or `loose` |
| `@canon-exceptions` | ✅ | List of broken rules |
| `@canon-justification` | ✅ | Why this exists |
| `@canon-owner` | ✅ | Team or responsible person |
| `@canon-target-date` | ⚠️ | Elimination deadline |
| `@canon-migration-status` | ⚠️ | `planned` \| `in-progress` \| `blocked` |

---

## 🎚️ Canon Levels (Real Impact)

### `strict`

❌ **No exceptions allowed**  
- CI blocks
- Used for:
  - New components
  - Providers
  - Critical infrastructure

### `standard`

⚠️ **Exceptions allowed with annotation**  
- CI warns
- Project default

### `loose`

🟡 **Legacy tolerated**  
- CI doesn't block
- Validator tracks debt

---

## 🔍 Linter Behavior (Canon Rule #40)

### 1️⃣ Without Annotation → Error
```
❌ Canon #40 violation
File uses non-canonical tokens but has no @canon-exceptions annotation
```

➡️ **CI fails**

### 2️⃣ With Valid Annotation → Allowed
```
⚠️ Canon Exception (#21, #24)
Owner: ui-team
Target date: 2025-03-01
```

➡️ **CI continues**  
➡️ **Violation becomes tracked exception**

### 3️⃣ Invalid Annotation → Error

Examples:
- Non-existent rule (#99)
- Missing field
- Wrong format

➡️ **CI fails**

---

## 🤖 Automatic Lint Rules

### Token Validator Logic
```rust
if violation_found {
    if has_canon_annotation(file) {
        mark_as_exception()
    } else {
        fail()
    }
}
```

### What Linter Validates Automatically

✔️ Annotation presence  
✔️ Correct syntax  
✔️ Rule exists (1–40)  
✔️ Valid `canon-level`  
✔️ File matches configured level  
✔️ No exception in forbidden provider  

---

## 🧮 Impact on Compliance Score

Rule #40 doesn't "zero out" compliance, it **classifies debt**.

### Example:

| Situation | Impact |
|-----------|--------|
| Violation without annotation | ❌ -5% |
| Annotated exception | ⚠️ 0% |
| Canonical | ✅ +0% |

👉 **Result:**  
Compliance = real quality + transparency

---

## 📊 Reports

In reports (`canonrs report`):
```
Total files: 182
Canonical: 130
Annotated legacy: 52
Untracked violations: 0

Compliance: 100% (tracked)
```

**This is gold for enterprise.**

---

## 🧠 Why This Rule is Canon's Differentiator

### Without Rule #40:
- Design systems become dogma
- Teams bypass rules
- Lints get disabled

### With Rule #40:
- Technical debt stays visible
- Compliance becomes metric
- Evolution is gradual
- CI isn't the enemy

👉 **This rule transforms Canon from "tool" to healthy political system.**

---

## 📝 Exception Categories

### 1. Legacy Migration

**When:** Pre-canon code being gradually upgraded
```rust
//! @canon-level: loose
//! @canon-exceptions: [#21, #24]
//! @canon-justification: Component predates token system
//! @canon-migration-status: planned
//! @canon-target-date: 2025-Q2
```

**Allowed violations:** Any (temporary)  
**Requires:** Migration plan + target date  
**Review frequency:** Monthly

---

### 2. Third-Party Integration

**When:** External library constraints
```rust
//! @canon-level: standard
//! @canon-exceptions: [#24]
//! @canon-justification: Stripe widget requires exact 44px height
//! @canon-vendor: Stripe
//! @canon-vendor-docs: https://stripe.com/docs/...
```

**Allowed violations:** Size/color matching external requirements  
**Requires:** Vendor documentation reference  
**Review frequency:** On vendor update

---

### 3. Brand Requirements

**When:** White-label or brand-specific overrides
```rust
//! @canon-level: standard
//! @canon-exceptions: [#21]
//! @canon-justification: ACME Corp brand colors (contract requirement)
//! @canon-customer: ACME Corp
//! @canon-contract-expires: 2025-12-31
```

**Allowed violations:** Customer-specific colors  
**Requires:** Contract reference + expiration  
**Review frequency:** On contract renewal

---

### 4. Performance Critical

**When:** Optimization requires non-canonical approach
```rust
//! @canon-level: standard
//! @canon-exceptions: [#35]
//! @canon-justification: Inline calc for 60fps animation
//! @canon-benchmark: benchmarks/animation-perf.md
```

**Allowed violations:** Performance-justified only  
**Requires:** Benchmark data  
**Review frequency:** Quarterly

---

## 🔧 Implementation Example

### Before (Violation)
```rust
// switch.rs
pub fn Switch() -> impl IntoView {
    view! {
        <button class="border-gray-300 p-[2px]">
            // ❌ Canon #21, #24 violations
        </button>
    }
}
```

**Validator output:**
```
❌ Canon #21: border-gray-300
❌ Canon #24: p-[2px]
```

### After (Annotated Exception)
```rust
//! @canon-level: loose
//! @canon-exceptions: [#21, #24]
//! @canon-justification: Legacy switch component
//! @canon-owner: ui-team
//! @canon-target-date: 2025-03-01
//! @canon-migration-status: planned

pub fn Switch() -> impl IntoView {
    view! {
        <button class="border-gray-300 p-[2px]">
            // ⚠️ Tracked exception
        </button>
    }
}
```

**Validator output:**
```
⚠️ Exception tracked (#21, #24)
Owner: ui-team | Target: 2025-03-01
```

---

## 🚨 Forbidden Patterns

### ❌ Exception Without Justification
```rust
//! @canon-level: loose
//! @canon-exceptions: [#21]
// ❌ NO JUSTIFICATION - INVALID
```

**Linter error:**
```
❌ Canon #40: Missing @canon-justification
```

---

### ❌ Exception for Non-Existent Rule
```rust
//! @canon-level: loose
//! @canon-exceptions: [#99]  // ❌ Rule doesn't exist
```

**Linter error:**
```
❌ Canon #40: Rule #99 does not exist
```

---

### ❌ Exception in Forbidden Area
```rust
// ❌ FORBIDDEN: Exception in provider
// packages-rust/rs-design/src/providers/theme_provider.rs

//! @canon-level: loose
//! @canon-exceptions: [#32]
// ❌ Providers cannot have exceptions for localStorage
```

**Linter error:**
```
❌ Canon #37: Providers cannot use exceptions for critical rules
```

---

## 📈 Monthly Exception Review

### Automated Report
```bash
canonrs exceptions --expired
```

**Output:**
```
⚠️ EXPIRED EXCEPTIONS:

ui/switch.rs
  Owner: ui-team
  Target: 2025-01-01 (EXPIRED)
  Exceptions: #21, #24

ui/separator.rs
  Owner: ui-team
  Target: 2025-01-15 (15 days overdue)
  Exceptions: #21

Total expired: 2
```

---

## 🎯 Success Metrics

### Healthy Exception Management:

- ✅ 90%+ of exceptions have target dates
- ✅ <10% of codebase has exceptions
- ✅ Average exception lifetime <6 months
- ✅ Zero expired exceptions >3 months old

### Warning Signs:

- ⚠️ Exceptions growing faster than removals
- ⚠️ Same files excepted repeatedly
- ⚠️ Expired exceptions ignored
- ⚠️ Generic justifications ("legacy", "TODO")

---

## 🧪 Testing

### Valid Annotation Test
```rust
#[test]
fn test_valid_exception_annotation() {
    let content = r#"
//! @canon-level: loose
//! @canon-exceptions: [#21]
//! @canon-justification: Legacy component
//! @canon-owner: ui-team
    "#;
    
    assert!(is_valid_annotation(content));
}
```

### Expired Exception Test
```rust
#[test]
fn test_expired_exception_warning() {
    let annotation = parse_annotation(file);
    let target = parse_date(&annotation.target_date);
    
    if target < now() {
        warn!("Exception expired: {}", file);
    }
}
```

---

## 🏁 Conclusion

### ✔️ Yes, it fits  
### ✔️ Yes, it's necessary  
### ✔️ You already implemented the technical base  
### ✔️ Only formal documentation was missing  

**Canon Rule #40 is the peace treaty between ideal and reality.**

---

## 📚 References

- [Canon Rule #35: Token Usage Validation](./canon-rule-35-token-usage-validation.md)
- [Canon Rule #36: Component Compliance Levels](./canon-rule-36-component-compliance-levels.md)
- [Canon Rule #37: Provider Taxonomy & Boundaries](./canon-rule-37-provider-taxonomy-boundaries.md)

---

**Enforcement:** Automated via validators  
**Exception Limit:** Max 10% of codebase  
**Review Cycle:** Monthly  
**Auto-expire:** 6 months without renewal
