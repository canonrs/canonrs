# Canon Rule #258: Mode Drives Build Profiles

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Scope:** cli, build
**Version:** 1.0.0  
**Date:** 2026-02-13  

---

## Principle

**Application mode (ssr | csr | hybrid) defined in `canonrs.toml` is the single source of truth for build profile selection.**

Profiles must never be manually defined inside product `Cargo.toml`.

---

## Deterministic Mapping

| Mode   | Profile           | Purpose |
|--------|-------------------|----------|
| ssr    | canonrs-ssr       | Server-first build |
| csr    | canonrs-csr       | WASM-optimized build |
| hybrid | canonrs-hybrid    | Dual-target build |

---

## Enforcement

- CLI injects correct `[profile.canonrs-*]`
- Products cannot override profiles
- `[profile.*]` inside product Cargo.toml is forbidden

---

## Rationale

Mode is architectural intent.
Profiles are execution strategy.

They must always be aligned.

