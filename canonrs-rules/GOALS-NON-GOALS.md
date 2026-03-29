# CanonRS — Goals & Non-Goals

This document defines **exactly what CanonRS exists to do** — and, more importantly,  
**what it explicitly refuses to do**.

CanonRS is opinionated by design.  
Ambiguity is treated as architectural failure.

---

## 1. Core Goals (What CanonRS Is For)

### 1.1 Enforce Architectural Truth

CanonRS exists to enforce **how UI, runtime behavior, CSS, SSR, and hydration actually work**.

It turns:
- Implicit behavior into explicit law
- Emergent bugs into deterministic violations
- “It works locally” into an invalid excuse

**Correctness is not optional.**

---

### 1.2 Eliminate Ghost Bugs

CanonRS is designed to eliminate entire classes of bugs:

- Hydration mismatches
- Event listeners disappearing after hydration
- CSS cascade conflicts
- Runtime code executing in the wrong phase
- Toolchain inconsistencies between dev and prod
- Components compensating for architectural mistakes

If a bug exists, it must map to a violated rule.

---

### 1.3 Enforce Strict Ownership Boundaries

CanonRS enforces **single ownership** across all layers:

- Primitives own semantics
- UI owns visual interpretation
- Runtime owns effects
- CSS tokens own values
- Shell owns browser APIs

No layer is allowed to “help” another.

Help creates dependency.
Dependency creates rot.

---

### 1.4 Treat SSR as Document Generation

CanonRS formalizes SSR as:

- Deterministic HTML document generation
- Explicit script ordering
- Explicit provider trees
- Predictable hydration replacement

SSR is not a performance hack.
It is a correctness requirement.

---

### 1.5 Make Debugging Normative

CanonRS converts debugging from exploration into deduction.

Symptoms imply causes.
Causes imply violated rules.

This enables:
- Faster diagnosis
- Shared mental models across teams
- Less tribal knowledge
- Fewer “hero debuggers”

---

### 1.6 Survive Scale and Refactors

CanonRS optimizes for:
- Large teams
- Long-lived codebases
- Aggressive refactoring
- Toolchain upgrades
- Staff turnover

If a system only works while its authors are present, it is invalid.

---

## 2. Secondary Goals (Allowed but Not Central)

These goals are acceptable **only if they do not conflict with core goals**.

- Design system integration
- Multiple themes and brands
- Product-specific aesthetics
- Component libraries built on top
- Developer ergonomics (within constraints)
- Educational documentation

These are *permitted*, not *guaranteed*.

---

## 3. Explicit Non-Goals (What CanonRS Refuses to Be)

### 3.1 CanonRS Is Not a Design System

CanonRS does not aim to:
- Provide pretty components
- Define brand identity
- Solve visual design
- Optimize for aesthetics

Design systems may exist **on top of CanonRS**, but CanonRS does not exist to serve them.

---

### 3.2 CanonRS Is Not Flexible by Default

CanonRS explicitly rejects:
- “Just override it”
- “Temporary exceptions”
- “We’ll clean this up later”
- Silent escape hatches

Flexibility is earned through understanding, not granted by default.

---

### 3.3 CanonRS Does Not Optimize for Onboarding Speed

CanonRS does not aim to be:
- Easy on day one
- Instantly productive
- Intuitive without study

Learning CanonRS requires unlearning common industry myths.

Initial friction is intentional.

---

### 3.4 CanonRS Does Not Abstract Reality Away

CanonRS will not:
- Hide hydration mechanics
- Mask runtime ordering
- Obscure CSS cascade
- Virtualize browser behavior

Reality is exposed, not abstracted.

---

### 3.5 CanonRS Is Not Framework-Agnostic

CanonRS is grounded in:
- Real HTML semantics
- Real browser behavior
- Real WASM constraints
- Real SSR mechanics

It does not aim to be portable across incompatible paradigms.

---

### 3.6 CanonRS Does Not Protect Against Poor Judgment

CanonRS cannot prevent:
- Ignoring the rules
- Writing bad business logic
- Poor product decisions
- Bad UX choices

CanonRS protects architecture, not taste.

---

## 4. What Happens If You Ignore the Non-Goals

Treating CanonRS as something it is not results in:

- Pressure to weaken rules
- Requests for “just this one exception”
- Boundary erosion
- Gradual architectural collapse
- Loss of determinism
- Loss of trust in the system

CanonRS fails loudly when misused — by design.

---

## 5. Final Statement

CanonRS exists to define **what is allowed to exist** in a UI system.

It is not concerned with popularity.
It is not concerned with trends.
It is not concerned with comfort.

It is concerned with **truth, ownership, and causality**.

If those are not your priorities,
CanonRS is not for you.

---

**CanonRS**
*Clear goals. Explicit refusals. No ambiguity.*
