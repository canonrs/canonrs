# Canon Rule #13 — Specialization vs Substitution

**Status:** ENFORCED
**Severity:** HIGH
**Scope:** components, architecture
**Version:** 1.0.0
**Date:** 2025-01-16

---
**Short statement (easy to remember):**  
A specialized component never replaces its base component. It extends semantics, not rewrites it.

---

## Formal Definition
```
Base Component        = Generic, foundational component
Specialized Component = Semantic extension of the base, specific case
```

**Specialization** is about **extending semantics**.  
**Substitution** is about **rewriting behavior**.

**Rule #13 forbids substitution disguised as specialization.**

---

## 🔒 WHAT THIS RULE PROHIBITS (binding)

### ❌ FORBIDDEN

#### 1. Using specialized as default
```rust
// ❌ WRONG - MaskedInput as default
fn UserForm() {
    view! {
        <MaskedInput value=name />
    }
}

// ✅ CORRECT
fn UserForm() {
    view! {
        <Input value=name />
        <MaskedInput value=cpf mask_type=MaskType::CPF />
    }
}
```

#### 2. Magic flags turning base into specialized
```rust
// ❌ FORBIDDEN
<Input mask="cpf" />
<Select searchable=true />
<Button loading=true spinner />

// ✅ CORRECT
<MaskedInput mask_type=MaskType::CPF />
<Combobox />
<LoadingButton />
```

#### 3. "Smart" components deciding automatically
```rust
// ❌ FORBIDDEN
<SelectOrCombo options=list />

// ✅ CORRECT
if list.len() < 50 {
    <Select options=list />
} else {
    <Combobox options=list />
}
```

#### 4. Silent substitution
```rust
// ❌ FORBIDDEN
type Select = Combobox;

// ✅ CORRECT
<Select />
<Combobox />
```

---

## ✅ WHAT THE RULE REQUIRES

### 1. Base Component ALWAYS exists

Every specialized component **must have a generic base component**:

| Base         | Exists as? | Purpose                    |
|--------------|------------|----------------------------|
| `Input`      | ✅ Type 1  | Generic field              |
| `Select`     | ✅ Type 1  | Native selection           |
| `Button`     | ✅ Type 1  | Generic action             |
| `DataTable`  | ✅ Type 3  | Human-scale semantic table |

### 2. Specialization is EXPLICIT and SEPARATE

| Base         | Specialized      | Relation                         |
|--------------|------------------|----------------------------------|
| `Input`      | `MaskedInput`    | Extension (required format)      |
| `Select`     | `Combobox`       | Extension (search + overlay)     |
| `Button`     | `IconButton`     | Extension (visual variant)       |
| `DataTable`  | `VirtualTable`   | **Change of nature** (Rule #17)  |

### 3. Specialized DEPENDS semantically on Base

**MaskedInput:**
- ✅ Reuses Input tokens  
- ✅ Reuses Field wrapper  
- ✅ Reuses validation patterns  
- ✅ Adds: deterministic mask  

**Combobox:**
- ✅ Reuses "selection" concept  
- ✅ Reuses option rendering  
- ✅ Adds: search, overlay, virtualization  

---

## 🧠 WHY THIS IS A RULE

This rule solves **component creep**:

### Without Rule #13:
```rust
<Input 
    mask="cpf"
    autocomplete=true
    debounce=300
    validation="email"
    prefix_icon="user"
    clearable=true
/>
```

**Problem:** Input becomes a God Component.

### With Rule #13:
```rust
<Input />
<MaskedInput />
<AutocompleteInput />
<DebouncedInput />
<EmailInput />
<IconInput />
```

**Solution:** Small, focused, composable components.

---

## 🏷️ RULE CLASSIFICATION

| Field       | Value                              |
|-------------|------------------------------------|
| Rule ID     | Canon Rule #13                     |
| Category    | Component Design / Specialization  |
| Type        | Architectural Rule                 |
| Severity    | **High**                           |
| Scope       | All Components / DX / Maintenance  |
| Violation   | **Review Blocker**                 |

---

## 🧬 CANONICAL MATRIX: Base → Specialized

| Base Component | Specialized Component | Extension Type        | Related Rule |
|----------------|----------------------|------------------------|--------------|
| `Input`        | `MaskedInput`        | Required format        | #13          |
| `Input`        | `EmailInput`         | Specific validation    | #13          |
| `Select`       | `Combobox`           | Search + overlay       | #12, #13     |
| `Button`       | `IconButton`         | Visual variant         | #13          |
| `Button`       | `LoadingButton`      | Specialized state      | #13          |
| `DataTable`    | `VirtualTable`       | **Change of nature**   | #14, #17     |
| `Grid`         | -                    | Base (no specialization)| -           |

**Note:** DataTable → VirtualTable is NOT specialization, it is **scale change** (Rule #17).

---

## 🎯 HOW RULE #13 FITS IN THE CANON
```
Rule #12 (Select vs Combobox)
    ↓
Rule #13 (Specialization vs Substitution)
    ↓
Rule #17 (Human vs Machine Scale)
```

---

## 📊 Specialization vs Substitution

| Aspect              | Specialization (✅)      | Substitution (❌)       |
|----------------------|--------------------------|-------------------------|
| Relation to base     | Extends                  | Replaces                |
| Tokens               | Reuses                   | Redefines               |
| API                  | Adds props               | Changes base behavior   |
| Semantics            | Preserves + adds         | Rewrites                |
| Usage                | Specific case            | "Improved version"      |
| Example              | MaskedInput extends Input| Input with mask flag    |

---

## 🧪 HOW THIS RULE IS APPLIED IN PRACTICE

### Code Review Checklist

- [ ] PR adds flag to base component that changes behavior?  
- [ ] PR uses specialized where base would suffice?  
- [ ] PR creates auto-deciding component?  
- [ ] Feature should be separate component?  

**If yes → PR NOT APPROVED**

---

## 🚫 ANTI-PATTERNS

### ❌ God Component
```rust
<Input
    type="text"
    mask="cpf"
    autocomplete=true
    debounce=300
    currency=true
    validation="email"
/>
```

### ❌ Magic Flags
```rust
<Select searchable=true />
```

### ❌ Silent Substitution
```rust
pub type Input = MaskedInput;
```

---

## ✅ CORRECT PATTERNS

### Pattern 1: Clear Extension
```rust
#[component]
pub fn Input(
    value: Signal<String>,
    on_change: Callback<String>,
) -> impl IntoView { /* ... */ }

#[component]
pub fn MaskedInput(
    value: Signal<String>,
    on_change: Callback<String>,
    mask_type: MaskType,
) -> impl IntoView {
}
```

### Pattern 2: Composition
```rust
#[component]
pub fn EmailInput(
    value: Signal<String>,
    on_change: Callback<String>,
) -> impl IntoView {
    view! {
        <Field label="Email">
            <Input
                value=value
                on_change=on_change
                type="email"
            />
        </Field>
    }
}
```

### Pattern 3: Explicit Decision
```rust
#[component]
fn DocumentForm() -> impl IntoView {
    view! {
        <Input value=name />
        <MaskedInput value=cpf mask_type=MaskType::CPF />
        <EmailInput value=email />
    }
}
```

---

## 🎓 Design Principles

### 1. Specialization Adds Semantics
```
Input       → generic field
MaskedInput → field + required format
EmailInput  → field + email validation
```

### 2. Base is Always Available

### 3. No Magic Flags

---

## 🏁 FINAL VERDICT

- ✅ Canon Rule #13  
- ✅ Specialization principle  
- ✅ Prevents God Components  
- ✅ Forces explicit types  
- ✅ Blocks bad PRs  

---

## References

- Canon Rule #12  
- Canon Rule #17  

---

**Mantra:** *Specialization extends semantics. Substitution rewrites it. Never confuse them.*
