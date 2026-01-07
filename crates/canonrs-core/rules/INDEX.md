# Canon Rules Index

This document serves as the master index for all Canon Rules in the monorepo.

---

## Rules

| # | Title | Status | Enforcement | Script |
|---|-------|--------|-------------|--------|
| 01 | Rust Rules Docker Build Cache Invalidation | ACTIVE | MANDATORY | - |
| 02 | Compose One Per Product | ACTIVE | MANDATORY | - |
| 03 | BFF Mandatory Boundary | ACTIVE | MANDATORY | `check-bff-boundary.sh` |
| 04 | Port Allocation Strategy | ACTIVE | MANDATORY | `validate-ports.sh` |
| 05 | Core Services Port Non-Exposure | ACTIVE | MANDATORY | `validate-core-exposure.sh` |
| 06 | Traefik Single Public Ingress | ACTIVE | MANDATORY | `validate-traefik-ingress.sh` |
| 07 | Environment Variable Classification | ACTIVE | MANDATORY | `validate-env-classification.sh` |
| 08 | Authentication Core Hardening | ACTIVE | MANDATORY | `validate-auth-core.sh` |
| 09 | Token Audience & Cross-Service Trust | ACTIVE | MANDATORY | `validate-token-audience.sh` |
| 10 | mTLS Core-to-Core Communication | ACTIVE | MANDATORY | `validate-mtls-core.sh` |
| 11 | Key Rotation & kid Enforcement | ACTIVE | MANDATORY | `validate-key-rotation.sh` |
| 12 | Cargo Workspace ≠ Target Directory | ACTIVE | MANDATORY | - |
| 13 | Binary Size Validation for Rust | ACTIVE | MANDATORY | - |
| 14 | Exit 0 ≠ Crash | ACTIVE | MANDATORY | - |
| 15 | Scratch Images Require Self-Contained Binaries | ACTIVE | MANDATORY | - |
| 16 | Healthcheck Is Not Part of Application Logic | ACTIVE | RECOMMENDED | - |
| 17 | Cargo.lock v4 = Toolchain Incompatibility | ACTIVE | MANDATORY | - |
| 18 | Docker Cache Invalidates Debugging | ACTIVE | MANDATORY | - |
| 19 | Docker CLI vs Compose - Never Mix | ACTIVE | MANDATORY | - |
| 20 | MUSL Resolves 90% of "Impossible" Container Bugs | ACTIVE | MANDATORY | - |
| 21 | Docker Build Cache Invalidation (Detailed) | ACTIVE | MANDATORY | - |

---

## Rule Numbering Policy

- Rules are numbered sequentially starting from #01
- Once assigned, a rule number is permanent (even if deprecated)
- New rules take the next available number
- Deprecated rules keep their number with status changed to `DEPRECATED`

---

## Rule Lifecycle

**DRAFT** → Under development, not enforced  
**ACTIVE** → Enforced in CI/CD  
**DEPRECATED** → No longer enforced, kept for reference

---

## Rule Categories

### 🏗️ Architecture & Structure
- #02 (Compose One Per Product)
- #03 (BFF Mandatory Boundary)
- #04 (Port Allocation Strategy)
- #05 (Core Services Port Non-Exposure)
- #06 (Traefik Single Public Ingress)

### 🔐 Security & Authentication
- #08 (Authentication Core Hardening)
- #09 (Token Audience & Cross-Service Trust)
- #10 (mTLS Core-to-Core Communication)
- #11 (Key Rotation & kid Enforcement)

### 🦀 Rust Development
- #01 (Docker Build Cache for Rust)
- #12 (Cargo Workspace Target)
- #13 (Binary Size Validation)
- #14 (Exit 0 Understanding)
- #17 (Cargo.lock v4 Toolchain)
- #20 (MUSL Static Linking)
- #21 (Cache Invalidation Detailed)

### 🐋 Docker & Containers
- #15 (Scratch Images)
- #16 (Healthcheck Design)
- #18 (Cache During Debug)
- #19 (Docker CLI vs Compose)

### 🔧 Operations
- #07 (Environment Variable Classification)

---

## Rule Dependencies
```
#03 (BFF) ← #08 (Auth Core)
#04 (Ports) ← #05 (Non-Exposure)
#06 (Traefik) ← #05 (Non-Exposure)
#08 (Auth Core) ← #09 (Token Audience)
#09 (Token Audience) ← #10 (mTLS)
#10 (mTLS) ← #11 (Key Rotation)
#12 (Workspace) ← #13 (Binary Size)
#13 (Binary Size) ← #20 (MUSL)
#14 (Exit 0) ← #13 (Binary Size)
#15 (Scratch) ← #20 (MUSL)
#18 (Cache Debug) ← #01, #21 (Cache Invalidation)
#20 (MUSL) ← #15 (Scratch)
```

---

## 🎯 Quick Reference by Symptom

### "Container exits immediately with exit 0"
→ **#14** (Exit 0), **#13** (Binary Size), **#20** (MUSL), **#18** (Cache)

### "Binary works on host, fails in container"
→ **#20** (MUSL), **#15** (Scratch), **#13** (Binary Size)

### "Healthcheck always fails"
→ **#15** (Scratch), **#16** (Healthcheck Logic)

### "Can't find binary after cargo build"
→ **#12** (Workspace Target)

### "Cargo.lock version error"
→ **#17** (Toolchain)

### "Changes to code don't appear in container"
→ **#18** (Cache), **#01** (Cache Invalidation), **#21** (Detailed Cache)

### "Docker compose conflicts with manual containers"
→ **#19** (CLI vs Compose)

### "Binary too small (< 1MB) but has dependencies"
→ **#13** (Binary Size), **#20** (MUSL)

---

## Enforcement

All `ACTIVE` rules with scripts are executed in CI/CD pipeline:
- `.github/workflows/canon-rules-validation.yml`
- Runs on every push to `main` and `develop`
- Runs on all pull requests

**Manual Validation:**
```bash
# Validate all rules with scripts
./core-services/_rules/scripts/validate-all.sh

# Validate specific rule
./core-services/_rules/scripts/validate-mtls-core.sh
```

---

## Adding New Rules

1. Create `XX-rule-name.md` in `core-services/_rules/`
2. Follow template structure from existing rules
3. Create validation script in `core-services/_rules/scripts/` (if applicable)
4. Update this INDEX.md with new entry
5. Update rule dependencies if applicable
6. Update `.github/workflows/canon-rules-validation.yml` if script added
7. Submit PR with rationale and real-world example

---

## Rule Template Structure
```markdown
# Canon Rule #XX: Title

**Status:** `ACTIVE|DRAFT|DEPRECATED`  
**Enforcement:** `MANDATORY|RECOMMENDED|ADVISORY`  
**Scope:** [Description of what this applies to]

---

## 🎯 Objective
[What problem does this solve?]

## 📋 The Problem
[Detailed description with examples]

## ✅ Solution
[How to implement correctly]

## 🚫 Anti-Patterns
[What NOT to do]

## 🔍 Verification
[How to validate compliance]

## 🔗 Related Rules
[References to other rules]

---

**Last Updated:** YYYY-MM-DD  
**Version:** X.Y
```

---

**Last Updated:** 2025-01-07  
**Total Rules:** 21  
**Version:** 2.0

| 27 | Hydration Determinism (IDs, Time, Random, Order) | ACTIVE | MANDATORY | - |
