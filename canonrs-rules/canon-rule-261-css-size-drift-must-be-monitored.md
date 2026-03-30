# Canon Rule #261: CSS Bundle Size Drift Must Be Monitored

**Status:** ENFORCED
**Severity:** HIGH
**Version:** 1.0.0
**Date:** 2026-02-13

**Category:** build-tooling
**Tags:** css, build, performance, monitoring
**Language:** EN

---

**Intro:**
Unmonitored css growth leads to performance degradation and architectural decay. Size must be tracked and enforced.

**Problem:**
css bundle grows silently causing performance issues and bloat

**Solution:**
define size baseline and enforce limits via ci checks

**Signals:**
- bundle bloat
- slow load
- unexpected growth

**Search Intent:**
how to monitor css bundle size ci

**Keywords:**
css bundle size monitoring, frontend performance css size, ci size threshold check, design system css bloat

---

## Principle

canonrs.bundle.css MUST have a declared size expectation.

Significant size growth must be detected during CI.

---

## Problem

Silent CSS growth leads to:

- Unbounded token expansion
- Duplicate families
- Forgotten experimental layers
- Performance regressions

---

## Canonical Pattern

Baseline example:

canonrs.bundle.css ≈ X KB

CI must fail if growth exceeds declared threshold (ex: +15%).

---

## Enforcement Example

check-css-size.sh:

SIZE=$(du -k canonrs.bundle.css | cut -f1)
MAX=800

if [ "$SIZE" -gt "$MAX" ]; then
echo "❌ CSS exceeds expected budget"
exit 1
fi

---

## Rationale

Architecture scale must be observable.

Unbounded CSS growth is architectural decay.

---

## Exceptions

Budget may be raised via architectural review.

Silent growth is forbidden.

---

## Version History

- 1.0.0 — Initial definition