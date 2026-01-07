# Input Component Scope & Token Requirements

**Status:** Normative  
**Date:** 2025-12-30

---

## The Problem

Família C (Forms & Validation) includes `input.masking`, but not all inputs need masking. Without formal scope definition, this creates ambiguity:

- Should `Input` (generic) implement masking?
- Should every input component use 100% of Família C?

**Answer:** NO. Scope determines token requirements.

---

## Component Classification

### Generic Input Components

**Examples:** `Input`, `Textarea`  
**Purpose:** General text entry (text, email, password, number, etc.)

**Required Tokens (Família C):**
- ✅ `field.height`
- ✅ `field.padding`
- ✅ `field.border`
- ✅ `field.placeholder`
- ✅ `validation.{error,success,warning}`
- ❌ `input.masking` - NOT REQUIRED

**Rationale:** Generic inputs don't need masking logic. Adding it violates SRP.

---

### Specialized Input Components

**Examples:** `OTPInput`, `MaskedInput`, `PhoneInput`, `CPFInput`, `DateInput`  
**Purpose:** Constrained/formatted text entry

**Required Tokens (Família C):**
- ✅ `field.height`
- ✅ `field.padding`
- ✅ `field.border`
- ✅ `field.placeholder`
- ✅ `validation.{error,success,warning}`
- ✅ `input.masking` - REQUIRED

**Rationale:** These components exist specifically for formatted input.

---

## Token Requirements Matrix

| Token | Generic Input | Specialized Input |
|-------|---------------|-------------------|
| `field.height` | ✅ Required | ✅ Required |
| `field.padding` | ✅ Required | ✅ Required |
| `field.border` | ✅ Required | ✅ Required |
| `field.placeholder` | ✅ Required | ✅ Required |
| `validation.*` | ✅ Required | ✅ Required |
| `input.masking` | ❌ Not Applicable | ✅ Required |

---

## Decision Rules

### When creating an input component, ask:

1. **Does it accept arbitrary text?** → Generic Input (no masking)
2. **Does it enforce format/pattern?** → Specialized Input (with masking)

### Examples:

| Component | Type | Needs Masking? |
|-----------|------|----------------|
| `Input` (text/email/password) | Generic | ❌ No |
| `Textarea` | Generic | ❌ No |
| `OTPInput` (6 digits) | Specialized | ✅ Yes |
| `PhoneInput` (+55 11 99999-9999) | Specialized | ✅ Yes |
| `CPFInput` (999.999.999-99) | Specialized | ✅ Yes |
| `DateInput` (DD/MM/YYYY) | Specialized | ✅ Yes |

---

## Canon Rule Extension

**Original Rule:**  
"Família C tokens must be applied to form components."

**Clarification:**  
"Família C tokens must be applied **according to component scope**:
- Generic inputs: height, padding, border, placeholder, validation
- Specialized inputs: ALL Família C tokens including masking"

---

## Implementation Checklist

### Generic Input (`Input`, `Textarea`)
- [ ] `field.height` via `InputSize` variants
- [ ] `field.padding` via size classes
- [ ] `field.border` via `border-[var(--field-border)]`
- [ ] `field.placeholder` via `placeholder:text-[hsl(var(--field-placeholder))]`
- [ ] `validation.*` via `InputValidation` enum
- [ ] **NO** masking logic

### Specialized Input (`OTPInput`, `PhoneInput`, etc.)
- [ ] All generic input tokens (above)
- [ ] `input.masking` token for mask color/style
- [ ] Masking logic (pattern enforcement)
- [ ] Character restrictions
- [ ] Format validation

---

## Violation Examples

### ❌ WRONG: Generic Input with Masking
```rust
// Input.rs
#[component]
pub fn Input(
    mask: Option<String>, // ❌ Violates scope
    // ...
) { }
```

**Problem:** Generic input shouldn't handle masking. Create `MaskedInput` instead.

---

### ❌ WRONG: Specialized Input without Masking Token
```rust
// OTPInput.rs (specialized)
const BASE_CLASSES: &str = "..."; // ❌ Missing input.masking token
```

**Problem:** Specialized inputs MUST use `input.masking` for mask styling.

---

## Normative Status

- Generic inputs **MUST NOT** implement masking
- Specialized inputs **MUST** implement masking
- Token requirements follow component scope
- Violations block PRs

---

**Author:** Canon Working Group  
**Date:** 2025-12-30  
**Version:** 1.0  
**Status:** Normative
