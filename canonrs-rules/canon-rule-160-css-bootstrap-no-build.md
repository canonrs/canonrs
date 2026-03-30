# Canon Rule #160: First App Must Not Require CSS Build

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Version:** 1.0.0  
**Date:** 2026-01-27

**Category:** build-tooling
**Tags:** css, build, onboarding, architecture
**Language:** EN

---

**Intro:**
Requiring CSS build steps in initial setup increases friction and couples users to toolchains prematurely. This breaks onboarding simplicity and predictability.

**Problem:**
first app requires css build step causing onboarding friction

**Solution:**
provide prebuilt css so first app runs without any build step

**Signals:**
- build required
- onboarding friction
- toolchain dependency

**Search Intent:**
how to avoid css build step

**Keywords:**
css bootstrap no build, zero build frontend setup, onboarding css strategy, prebuilt css architecture

---

## Principle

**The first CanonRS application must run with prebuilt CSS and no CSS build step.**

- Objective
- Verifiable
- One clear boundary

---

## Problem

Requiring CSS build tools in the first app breaks onboarding.

- Immediate friction for new users
- Toolchain coupling (Node, Tailwind, PostCSS)
- False perception of framework complexity
- Non-deterministic first experience

---

## Forbidden Pattern

### ❌ Forbidden

```bash
npm install
npm run build:css
```

Why this violates the rule.

---

## Canonical Pattern

### ✅ Canonical

```html
<link rel="stylesheet" href="/canonrs-bootstrap.css" />
```

Why this complies with the rule.

---

## Rationale

The first app validates architecture, not tooling.

- Protects onboarding invariants
- Enforces zero-build bootstrap
- Prevents early toolchain lock-in
- Not an opinion: measurable by absence of build steps

---

## Enforcement

- Documentation review
- CI check for CSS build scripts in first-app guides
- Manual validation

---

## Exceptions

> **No exceptions. This rule is absolute.**

---

## Version History

- **1.0.0** — Initial version