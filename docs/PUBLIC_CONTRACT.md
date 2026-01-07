# Public Contract

**Version:** 0.1.0-alpha  

**Status:** 🟡 Public Alpha — Trusted for experimentation, not production
**Last Updated:** 2026-01-07  
**Purpose:** Define stability guarantees for external consumers

---

## 🎯 What This Document Does

This contract separates **what you can trust** from **what will change**.  
It exists to prevent breaking your code without warning.

---

## ✅ Stable (Will Not Break)

### Core Concepts
- **Multi-runtime execution model** (server/browser/hydrate)
- **Lexical context resolution** (provider hierarchy)
- **Deterministic SSR** (no random IDs, stable sort)

### Workspace Structure
```
rs-design/          → Design system (stable API surface)
canonrs-contract/   → Types, traits (semver-locked)
canonrs-runtime/    → Runtime detection (internal, but stable behavior)
```

### Design System API (`rs-design`)
- `CanonProvider` signature
- `ThemeProvider` context shape
- Block component props (color, spacing, variant)
- SSR determinism guarantees

**Breaking changes:** Will be versioned (0.1 → 0.2 → 1.0)

---

## ⚠️ Experimental (May Change)

### Adapters (`canonrs-*-adapter`)
- Internal implementation details
- Optimization strategies
- Cache behavior

### Examples
- File structure may reorganize
- Dependencies may update
- Not part of public API

### Documentation Structure
- Rule numbers may shift
- Capsule format may evolve
- Content is stable, organization isn't

---

## 🚫 Internal (Never Depend On)

### Private Modules
- Anything under `src/internal/`
- Helper functions not re-exported
- Build scripts, dev tools

### Implementation Details
- How contexts are stored
- Exact hydration sequence
- Docker layer optimization

---

## 📦 Semver Policy

**Target for 1.0:**  
Once Leptos SSR stabilizes on stable Rust (not nightly), we'll lock the public API and commit to strict semver.

**Before 1.0:**
- `0.x.y` → `y` can break things (with changelog)
- Major refactors will bump `x`
- Bug fixes bump `y`

**After 1.0:**
- Standard semver (breaking.feature.fix)
- 6-month deprecation warnings
- Migration guides for all breaks

---

## 🔄 Deprecation Process

1. Mark as `#[deprecated]` with alternative
2. Announce in CHANGELOG + GitHub Discussions
3. Keep for 2 minor versions minimum
4. Remove in next major

---

## 🎓 What "Stable" Means



**Stable ≠ Perfect.**  
It means:
- We won't remove it without warning
- Behavior is documented and tested
- API shape is frozen (but internals may optimize)


**Important:** Canon Rules define **mental invariants**, not public APIs.  
They teach how to think about the system, not what to import.
**Stable ≠ Perfect.**  
It means:
- We won't remove it without warning
- Behavior is documented and tested
- API shape is frozen (but internals may optimize)

---

## 📞 Breaking the Contract

If we must break something "stable":
1. GitHub Issue with **[BREAKING]** label
2. RFC period (14 days minimum)
3. Migration path documented
4. Major version bump

---

## 🧭 Current Maturity

| Component | Status | Notes |
|-----------|--------|-------|
| `rs-design` blocks | Alpha | API stable, internals may optimize |
| `CanonProvider` | Alpha | Shape stable, behavior may improve |
| Multi-runtime model | Stable | This is the foundation |
| Leptos version | Pinned | 0.7.x required, will upgrade with major bump |
| Docker setup | Internal | You control your own deployment |

---

## ❓ When to Trust This

**Trust this contract when:**
- Building production apps on CanonRS
- Writing adapters for other frameworks
- Creating derived design systems

**Don't trust this contract when:**
- Contributing to internals (read ARCHITECTURE.md)
- Optimizing build (read Canon Rules)
- Debugging runtime (read CANON_RUNTIME_CONTRACT.md)

---

## 📌 Relationship to Other Docs
```
PUBLIC_CONTRACT.md  → What won't break (for users)
ARCHITECTURE.md     → How it works (for contributors)
CANON_RUNTIME_CONTRACT.md → Mental model (for debugging)
docs/canonrs/index.md → How to think (for everyone)
```

---

**TL;DR:**  
`rs-design` API is stable. Examples are not. Internals will optimize. Breaking changes get warnings. Trust the runtime model, not the file paths.
