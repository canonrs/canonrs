# Canon Rule #323: Island Props Must Be Serializable

**Status:** ENFORCED
**Severity:** CRITICAL
**Version:** 1.0.0
**Date:** 2026-04-04

**Category:** islands-architecture
**Tags:** islands, props, serde, leptos
**Language:** EN

---

**Intro:**
All props passed to a Leptos island must implement Serialize and Deserialize.

**Problem:**
custom types without serde cause compile errors in island props

**Solution:**
derive serde traits or use primitive-compatible types

**Signals:**
- island compile error
- trait bound not satisfied
- failed to parse path

**Search Intent:**
leptos island props serialize deserialize

**Keywords:**
leptos island props, serde island, island compile error, wasm props serialization

---

## Principle

Islands are hydrated from serialized props embedded in the HTML. Every prop type must be serializable.

---

## Problem

Custom enums like DisabledState that do not derive Serialize cause compile errors when used as island props.

---

## Patterns

### Forbidden Pattern

Passing a custom enum without serde as an island prop.

### Canonical Pattern

Use Option<bool>, Option<f64>, Option<String> in island props. Unwrap with defaults inside the function body.

---

## Contract

### Enforcement

- island props must use only serializable types: bool, f64, i64, String, Option<T>
- custom enums used as island props must derive Serialize and Deserialize
- prop(default = value) is not supported in islands - use Option<T> with unwrap_or

### Exceptions

None.

---

## Version History

- 1.0.0 - Initial definition
