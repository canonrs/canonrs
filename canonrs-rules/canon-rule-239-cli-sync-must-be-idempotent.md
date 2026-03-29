# Canon Rule #239: CLI Sync Operations Must Be Idempotent

**Status:** ENFORCED
**Severity:** HIGH
**Scope:** cli, workspace
**Version:** 1.0.0
**Date:** 2026-02-13

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
