# Canon Rule #229: Design System CSS Must Be Consumed as an Independent Artifact

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Scope:** css, build, design-system
**Version:** 1.0.0  
**Date:** 2026-02-03

---

## Principle

**The CanonRS design system CSS MUST be consumed by products as an independent, prebuilt artifact, never as an input to utility or product-specific build pipelines.**

- One-directional dependency
- No build-time coupling
- No path leakage
- No implicit regeneration

---

## Problem

When design system CSS is merged into, imported by, or rebuilt inside product pipelines:

- Products become coupled to internal CanonRS paths
- Design system updates force product rebuilds
- Utility frameworks (Tailwind) gain authority over design tokens
- Build graphs become cyclic or implicit
- Versioning and rollback become impossible

**Observable symptoms**:
- Tailwind `@import` referencing `rs-canonrs/`
- Products breaking when CanonRS directory layout changes
- CSS changes requiring full product rebuilds
- Inability to ship CanonRS updates independently

---

## Forbidden Pattern

### ❌ Forbidden

```css
/* Tailwind input.css */
@import "../../../packages-rust/rs-canonrs/styles/canonrs.css";
```

```js
// tailwind.config.js
content: [
  "../../packages-rust/rs-canonrs/**/*"
]
```

```html
<!-- Product shell -->
<link rel="stylesheet" href="../../rs-canonrs/styles/canonrs.css">
```

**Why this violates the rule**:
- Product now depends on CanonRS internal layout
- Tailwind becomes a transitive owner of design tokens
- Breaks artifact isolation and versioning
- Violates Canon Rule #158 (immutable design system contracts)

---

## Canonical Pattern

### ✅ Canonical

```text
CanonRS (build once)
└── styles/canonrs.css   ← versioned artifact

Product
├── public/
│   ├── canonrs.css      ← copied or published artifact
│   └── output.css       ← utilities (Tailwind)
```

```html
<!-- Product HTML shell -->
<link rel="stylesheet" href="/canonrs.css">   <!-- Design system -->
<link rel="stylesheet" href="/output.css">    <!-- Utilities -->
```

**Why this complies**:
- CanonRS is built independently
- Products consume a stable artifact
- Tailwind remains utilities-only
- No path coupling, no rebuild cascade

---

## Rationale

### Architectural invariants protected

1. **Artifact isolation**
   - Design system is a distributable unit
   - Products do not rebuild it

2. **Dependency direction**
   ```
   Design System → Product
   Utilities      → Product
   ```
   Never the reverse.

3. **Version governance**
   - CanonRS can be versioned, cached, rolled back
   - Products opt-in to upgrades explicitly

4. **Scalability**
   - 1 design system
   - N products
   - N utility strategies (Tailwind, Uno, none)

### Bugs prevented

- Accidental token overrides
- CSS order instability
- Hidden rebuild dependencies
- CI-only failures (“works locally”)
- Design drift across products

### Why this is not opinion

This rule enforces **build graph correctness**.
Mixing design system CSS into product pipelines creates implicit dependencies that tooling cannot reason about or cache safely.

This is a structural invariant, not a stylistic choice.

---

## Enforcement

### CI checks

```bash
#!/bin/bash
# forbid-design-system-imports.sh

# Tailwind must not import canonrs.css
if grep -R "canonrs.css" products/*/style | grep "@import"; then
  echo "❌ CanonRS CSS must not be imported into Tailwind"
  exit 1
fi

# No product may reference rs-canonrs paths
if grep -R "rs-canonrs/styles" products/; then
  echo "❌ Products must not reference CanonRS internal paths"
  exit 1
fi
```

### Review checklist

- [ ] canonrs.css served as standalone asset
- [ ] No Tailwind `@import` of design system
- [ ] No relative paths to rs-canonrs in products
- [ ] CSS order: CanonRS → Utilities

---

## Exceptions

**No exceptions. This rule is absolute.**

If a product requires customization:
- Override via utilities
- Extend via new CanonRS families
- Never inline or recompile the design system

---

## Version History

- **1.0.0** — Initial version
