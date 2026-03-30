# Canon Rule #239: CLI Sync Operations Must Be Idempotent

**Status:** ENFORCED
**Severity:** HIGH
**Version:** 1.0.0
**Date:** 2026-02-13

**Category:** governance
**Tags:** cli, workspace, determinism, build
**Language:** EN

---

**Intro:**
Non idempotent cli operations cause unstable builds and unnecessary diffs. Outputs must remain deterministic.

**Problem:**
cli operations produce different outputs on repeated runs causing instability

**Solution:**
ensure cli sync produces identical results across executions

**Signals:**
- git diff
- unstable output
- rebuild loop

**Search Intent:**
how to make cli operations idempotent

**Keywords:**
cli idempotent operations, deterministic build output, workspace sync stability, frontend cli consistency

---

## Principle

Running `canonrs sync` multiple times without structural changes must produce zero diffs.

CLI operations must be idempotent.

---

## Problem

Non-idempotent generators cause:

- Git noise
- Infinite reformat loops
- CI instability
- Phantom diffs
- Timestamp pollution
- Rebuild cascades

---

## Forbidden Pattern

```
canonrs sync
git diff → changes

canonrs sync
git diff → different changes again  ❌
```

---

## Canonical Pattern

```
canonrs sync
git diff → clean

canonrs sync
git diff → clean
```

Output must be deterministic:

- Same ordering
- Same formatting
- Same whitespace
- Stable timestamps
- Stable dependency ordering

---

## Enforcement

CI pipeline:

```bash
canonrs sync
if ! git diff --quiet; then
  echo "❌ CLI not idempotent"
  exit 1
fi
```

---

## Exceptions

None.

CLI determinism is mandatory for architectural trust.

---

## Version History

- 1.0.0 — Initial definition