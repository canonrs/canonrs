# Canon Rule #196: SSR/CSR Separation for WASM Bloat Prevention

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Scope:** architecture, build, wasm
**Version:** 1.0.0  
**Date:** 2026-02-02

---

## Principle

**Design system code MUST be separated by compilation target: SSR-only code compiles only for server, CSR-only code compiles only for WASM client.**

- UI components, layouts, blocks, primitives = `#[cfg(feature = "ssr")]`
- Behaviors (event handlers, DOM manipulation) = `#[cfg(feature = "hydrate")]`
- Shared types (used by both) = no feature gate
- Design tokens, utils = `#[cfg(feature = "ssr")]` (CSS already has tokens)

---

## Problem

What breaks **without** this rule:

- **WASM bloat**: 22MB+ bundle includes entire SSR rendering engine
- **Build time explosion**: Compiling HTML rendering logic to WASM unnecessarily
- **Runtime overhead**: Loading megabytes of unused code in browser
- **Memory waste**: Client holds full component tree in WASM memory
- **Network cost**: Users download 10-20MB of SSR code they never execute

Real measurement from CanonRS pre-separation:
- WASM size: 22MB (debug), 8-10MB (release)
- After separation: 500KB-2MB (release)
- **Reduction: 80-95%**

---

## Forbidden Pattern

### ❌ Forbidden
```rust
// lib.rs - NO SEPARATION
pub mod ui;           // ❌ Compiles to WASM unnecessarily
pub mod blocks;       // ❌ HTML rendering in client
pub mod primitives;   // ❌ SSR logic in WASM
pub mod behaviors;    // ⚠️ No hydrate guard
pub mod design;       // ❌ 400KB tokens in WASM when CSS has them

// Result: 22MB WASM with full SSR engine
```

Why this violates the rule:
- Every module compiles for both targets
- WASM includes HTML composition, slot logic, SSR utilities
- Design tokens duplicated (CSS already has them)
- No architectural boundary between server and client

---

## Canonical Pattern

### ✅ Canonical
```rust
// lib.rs - CLEAN SEPARATION
#![recursion_limit = "512"]

// SSR-only: HTML rendering, composition, structure
#[cfg(feature = "ssr")]
pub mod blocks;

#[cfg(feature = "ssr")]
pub mod ui;

#[cfg(feature = "ssr")]
pub mod layouts;

#[cfg(feature = "ssr")]
pub mod primitives;

#[cfg(feature = "ssr")]
pub mod components;

#[cfg(feature = "ssr")]
pub mod providers;

#[cfg(feature = "ssr")]
pub mod design;  // Tokens already in CSS

#[cfg(feature = "ssr")]
pub mod utils;

// CSR-only: Event handlers, DOM mutation
#[cfg(feature = "hydrate")]
pub mod behaviors;

// Shared: Types used by both SSR and CSR
pub mod shared;  // TocItem, enums, trait definitions

// Re-exports (conditional)
#[cfg(feature = "ssr")]
pub use ui::button::Button;

#[cfg(feature = "ssr")]
pub use blocks::Card;
```

Why this complies:
- Clear compilation boundary
- WASM contains only client-side event runtime
- SSR contains only server-side rendering logic
- Shared types available to both without duplication
- Design tokens excluded from WASM (CSS has them)

---

## Rationale

Why this rule exists **architecturally**:

### Invariants it protects
- **WASM = event runtime, NOT rendering engine**
- **SSR = HTML generation, NOT shipped to client**
- **Shared = minimal type definitions only**

### Contracts it enforces
- If it renders HTML → SSR only
- If it composes UI → SSR only
- If it reacts to events → CSR only
- If it's a type used by both → Shared

### Bugs it prevents
- 10-20MB WASM bundles from SSR code leakage
- `spawn_local` panics from SSR code running in hydrate context
- Memory bloat from unused component definitions in client
- Build time waste compiling SSR logic to WASM

### Why it is not opinion
WASM size directly impacts:
- Time to Interactive (TTI)
- First Contentful Paint (FCP)
- Mobile network cost
- Browser memory pressure

20MB vs 2MB = 10x difference in load time on 3G network.

---

## Enforcement

How this rule is validated:

**Build-time:**
- `cargo leptos build --release` must produce WASM <3MB (optimized)
- CI fails if WASM exceeds 5MB threshold
- `wasm-opt -Oz` applied in release profile

**Code review:**
- New module in `rs-design/src/` requires explicit `#[cfg]`
- No `pub mod` without feature gate except `shared`
- Re-exports must match module feature gates

**Testing:**
- `cargo build --target wasm32-unknown-unknown --features hydrate` must NOT include SSR modules
- `cargo build --features ssr` must NOT include hydrate-only code

**Measurement:**
```bash
# Check WASM size
ls -lh target/site/pkg/*.wasm

# Analyze bloat sources
twiggy top target/site/pkg/*.wasm

# Verify separation
cargo tree --features hydrate --target wasm32-unknown-unknown | grep "rs-design"
```

---

## Exceptions

**No exceptions. This rule is absolute.**

Every module must be explicitly categorized:
- SSR-only → `#[cfg(feature = "ssr")]`
- CSR-only → `#[cfg(feature = "hydrate")]`
- Both → no feature gate (must justify in code review)

The only valid "both" case: pure type definitions in `shared`.

---

## Related Rules

- **Canon Rule #195**: Interactive components require explicit ID (hydrate needs IDs, SSR doesn't)
- **Canon Rule #XXX** (future): WASM budget enforcement in CI

---

## Version History

- **1.0.0** — Initial version (2026-02-02)
  - Established SSR/CSR separation requirement
  - Documented 80-95% WASM size reduction
  - Defined feature gate patterns
