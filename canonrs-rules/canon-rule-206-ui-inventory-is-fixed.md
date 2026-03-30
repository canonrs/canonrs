# Canon Rule #206: UI Inventory Is Fixed and Canonical

**Status:** ENFORCED
**Severity:** CRITICAL
**Version:** 1.1.0
**Date:** 2026-01-29

**Category:** governance
**Tags:** ui, inventory, design-system, contracts
**Language:** EN

---

**Intro:**
Uncontrolled UI component growth leads to breaking changes and inconsistent APIs. UI inventory must be explicit and versioned.

**Problem:**
ui component set changes without control causing breaking changes and drift

**Solution:**
enforce fixed ui inventory with build time validation and versioning

**Signals:**
- unexpected component
- inventory drift
- build failure

**Search Intent:**
how to enforce fixed ui component inventory

**Keywords:**
ui inventory governance, design system component control, frontend api stability, component registry validation

---

## Principle

**The UI inventory is a fixed, canonical contract.**

- UI count is explicit
- UI set is enumerable
- Any drift is a breaking change

---

## Canonical Statement

As of this rule:

- **Total UI components: 78**
- **Source of truth:** filesystem inventory
- **Location:** `packages-rust/rs-design/src/ui`

This number is **authoritative**.

---

## Invariants

- Every UI component:
  - MUST live under `src/ui/<component>`
  - MUST be discoverable by build-time scan
  - MUST consume **only family tokens (public API)**

- The UI inventory:
  - MUST NOT change silently
  - MUST NOT be inferred implicitly
  - MUST be versioned intentionally

---

## Enforcement

### Build Time Enforcement

- `build.rs` MUST:
  - Discover UI directories
  - Emit a deterministic list
  - Compare against expected count (**78**)

### Failure Conditions

Build **MUST FAIL** if:

- UI count ≠ **78**
- A UI directory is added without version bump
- A UI directory is removed without version bump
- A UI directory bypasses discovery

---

## Forbidden Patterns

### Forbidden

- "We added one more UI, no big deal"
- Dynamic or runtime UI discovery
- UI components outside `src/ui`
- UI count changing without rule update

---

## Allowed Patterns

### Allowed

- Explicit version bump + rule update
- Regenerated inventory with review
- CI-enforced diff on UI list

---

## Rationale

UI components are **public API surface**.

An uncontrolled UI set leads to:

- Undocumented breaking changes
- Token misuse
- Design drift
- Inconsistent consumer expectations

A fixed inventory turns the design system into a **real framework**.

---

## Canon Outcome

✅ UI inventory is explicit
✅ Drift is impossible
✅ Governance is enforced
✅ CI blocks invalid changes

---

**UI count is not metadata.
UI count is contract.**

---

**Version History:**
- 1.0.0 (2026-01-28): Initial - 175 components (incorrect)
- 1.1.0 (2026-01-29): Corrected - 78 components (filesystem reality)