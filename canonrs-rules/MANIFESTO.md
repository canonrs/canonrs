# CanonRS Manifesto
## A Canonical System for UI, Runtime, and SSR

---

## 1. What CanonRS Is

CanonRS is **not** a design system.  
CanonRS is **not** a UI framework.  
CanonRS is **not** a collection of best practices.

CanonRS is a **closed normative system** that defines **how UI, runtime behavior, CSS, SSR, and hydration actually work** — and enforces those realities as architecture.

Where most systems describe *preferences*, CanonRS describes *causality*.

If a CanonRS rule is violated, the system does not become “less elegant” —  
**it becomes incorrect**.

---

## 2. The Core Premise

> **Reality precedes preference. Architecture precedes style.**

CanonRS is built on a single uncompromising sequence:

Physical Reality → Architectural Model → Enforcement

yaml
Copiar código

This order is non-negotiable.

Any attempt to reverse it — by hiding runtime behavior in components, styling primitives, or treating hydration as enhancement — leads to silent failure, undefined behavior, and architectural rot.

---

## 3. The Canonical Separation of Concerns

CanonRS enforces **hard boundaries** that cannot be blurred.

### 3.1 Primitives

Primitives are **structural only**.

- Semantic HTML
- `data-*` attributes
- Zero CSS
- Zero browser APIs
- Zero behavior

Primitives do not adapt.
Primitives do not compensate.
Primitives do not “fix” environments.

If primitives know about CSS or the browser, the system is already broken.

---

### 3.2 UI Layer

UI exists to provide **ergonomics and resilience**.

- Neutralizes hostile CSS
- Applies spacing, alignment, density
- Owns visual semantics
- Interprets intent declared by primitives

UI may adapt.
UI may defend.
UI may compensate.

But UI **never violates primitive purity**.

---

### 3.3 Runtime (Shell)

Runtime behavior is **infrastructure**, not component logic.

- Clipboard
- Drag & drop
- Keyboard shortcuts
- Global event delegation
- Hydration safety

Runtime code lives in the **shell**, executes **before hydration**, and is owned by deployment — not by Rust components.

Components declare intent.  
The shell executes effects.

---

## 4. SSR Is Document Generation

CanonRS rejects the illusion that SSR is “CSR with a head start”.

SSR is **HTML document generation**.

- `<html>`, `<head>`, `<body>` are mandatory
- Scripts must be ordered deterministically
- Providers must exist before render
- Hydration replaces the DOM — it does not enhance it

Any architecture that assumes node identity survives hydration is incorrect by definition.

---

## 5. Hydration Is Replacement, Not Enhancement

Hydration **destroys and replaces** DOM nodes.

Therefore:

- Pre-hydration listeners are invalid
- Imperative bindings are discarded
- Event delegation is mandatory
- Node identity is not stable

CanonRS encodes this reality as law — not advice.

---

## 6. Ownership Is Architectural, Not Visual

Every visual signal must have **exactly one owner**.

- Focus ring
- Underline
- Selection marker
- Active border

If two layers draw the same idea, the system is invalid.

Visual ambiguity is not a styling issue.  
It is a semantic ownership violation.

---

## 7. Tooling Is Not Neutral

CanonRS treats toolchains as part of architecture.

- WASM debug builds are incompatible with interactive development
- Workspace-level feature resolution is mandatory
- Floating nightly is a requirement, not a preference
- Build orchestration is workspace-scoped

Tooling constraints are encoded as rules to eliminate false debugging paths.

---

## 8. Debugging Is Normative

CanonRS eliminates empirical debugging.

Instead of:
> “It doesn’t work, let’s try something”

CanonRS enables:
> “This symptom implies violation of Rule X”

Examples:
- Works in prod, fails in dev → hydration order violation
- Script loads but behavior is dead → runtime ordering violation
- CSS breaks layout → token or ownership violation
- UI fixes primitive bugs → layering violation

Bugs become **architectural proofs**, not mysteries.

---

## 9. CanonRS Is a Closed System

CanonRS is internally consistent.

Rules are not independent:
- Structural rules derive from physical constraints
- Enforcement rules exist to protect foundational ones
- Nothing exists in isolation

You cannot partially adopt CanonRS.

You either accept the causal chain —  
or you are using something else.

---

## 10. Why CanonRS Is Hard to Copy

The value of CanonRS is **not the text of the rules**.

It is:
- The causal model
- The rigidity of boundaries
- The refusal to compromise
- The conversion of bugs into violations

Anyone can copy rules.
Very few can maintain a **coherent closed system under pressure**.

---

## 11. Final Declaration

CanonRS does not optimize for speed of adoption.
It optimizes for **correctness under scale**.

It favors:
- Determinism over convenience
- Architecture over ergonomics
- Explicit ownership over implicit magic
- Reality over abstraction

CanonRS exists for teams who prefer **systems that cannot lie**.

---

**CanonRS**
*A Canonical Reality System*
