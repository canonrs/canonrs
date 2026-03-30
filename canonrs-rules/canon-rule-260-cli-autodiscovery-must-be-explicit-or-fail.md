# Canon Rule #260: CLI Autodiscovery Must Be Explicit or Fail

**Status:** ENFORCED  
**Severity:** HIGH  
**Version:** 1.0.0  
**Date:** 2026-02-13  

**Category:** governance
**Tags:** cli, config, discovery, determinism
**Language:** EN

---

**Intro:**
Implicit fallback paths hide configuration errors and create unpredictable behavior. Discovery must be explicit.

**Problem:**
cli uses silent fallback paths causing hidden misconfiguration

**Solution:**
enforce explicit discovery with hard failure when root is not found

**Signals:**
- misconfiguration
- unexpected path
- partial build

**Search Intent:**
how to enforce explicit cli discovery

**Keywords:**
cli root discovery rules, explicit config resolution, fail fast cli pattern, workspace root detection rust

---

## Principle

**CanonRS CLI must never rely on silent fallback paths.**

Root discovery must follow this strict order:

1. `$CANONRS_ROOT` environment variable
2. Directory traversal up to defined depth
3. Hard failure with explicit instruction

---

## Forbidden

❌ Hardcoded filesystem fallbacks  
❌ Implicit default paths  
❌ Silent misconfiguration  

---

## Required Failure Behavior

If root cannot be found:

- CLI must stop execution
- Clear remediation steps must be printed
- No partial build may occur

---

This prevents invisible configuration drift and ensures explicit operational intent.
