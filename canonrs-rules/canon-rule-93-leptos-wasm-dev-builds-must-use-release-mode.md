# Canon Rule #93: Leptos WASM Dev Builds Must Use Release Mode

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Scope:** leptos, wasm, build
**Version:** 1.0.0  
**Date:** 2026-01-14

---

## Principle

**Interactive Leptos applications that compile to WASM MUST run dev/watch in `--release` mode.**

Debug-mode WASM builds are prohibitively large, cause browser-side compilation stalls, and make the application unusable during development.

This is a toolchain constraint, not an application bug.

---

## The Problem

When using default dev/watch mode:

- WASM is built with full debug symbols
- Binary size explodes to hundreds of megabytes
- Browser must download, validate, compile, and instantiate the module
- Main thread is blocked for tens of seconds to minutes
- App appears frozen after refresh with no errors

### Real Symptoms

- Page loads, then freezes for ~30–60s
- No console errors
- Hydration logs appear only after long delay
- Hot reload WebSocket disconnects
- Developers misdiagnose SSR, routing, or JS issues

---

## Anti-Pattern (FORBIDDEN)

```bash
# ❌ FORBIDDEN for medium/large apps
cargo leptos watch
```

This command produces debug WASM and must not be used for interactive development in CanonRS-scale apps.

---

## Canonical Pattern (REQUIRED)

```bash
# ✅ REQUIRED
cargo leptos watch --release
```

Release mode:
- Strips debug symbols
- Applies LLVM optimizations
- Produces small, browser-friendly WASM
- Enables instant refresh and hydration

---

## Expected Results

| Mode | Typical WASM Size | Browser Behavior |
|----|-------------------|------------------|
| Debug | 100–300 MB | Freeze / Stall |
| Release | 1–8 MB | Instant load |

Any WASM larger than ~10 MB in dev indicates a rule violation.

---

## Approved Alternatives

If `--release` cannot be used temporarily:

```bash
# Acceptable fallback (not ideal)
RUSTFLAGS="-C debuginfo=0 -C opt-level=2" cargo leptos watch
```

This is a mitigation, not a replacement for `--release`.

---

## Enforcement

- All CanonRS Leptos apps MUST document `--release` as the default dev command
- Scripts (`dev.sh`, `Makefile`, `Justfile`) MUST use release mode
- Debug WASM usage is allowed ONLY for:
  - minimal demos
  - reproduction of compiler bugs
  - non-interactive examples

---

## Canonical Justification

> **WASM debug builds are incompatible with interactive development.**  
> The cost of browser-side compilation exceeds acceptable latency, creating false negatives and wasted debugging effort.

This rule exists to:
- Eliminate phantom performance bugs
- Align dev experience with production reality
- Encode toolchain limitations as governance
- Prevent repeated misdiagnosis

---

## Related Canon Rules

- Canon Rule #87 — Leptos SSR Script Placement
- Canon Rule #90 — Hydration Is DOM Replacement
- Canon Rule #92 — SSR Debugging Heuristic

---

## Version History

- **1.0.0** (2026-01-14): Initial rule based on Workbench WASM stall diagnosis
