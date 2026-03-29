# Canon Rule #230: WASM Artifact Budget Is a Hard Contract

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Scope:** build, wasm
**Version:** 1.0.0  
**Date:** 2025-02-04

---

## Principle

**Every WASM artifact MUST declare and enforce a maximum size budget at build time, failing compilation if exceeded.**

- Budget is declared explicitly (not implicit)
- Enforcement happens at build time (not deployment)
- Violations block merge/deploy

---

## Problem

Without size budgets:

- WASM bundles grow silently from 2MB → 143MB (71x regression, real incident)
- Performance degradation goes unnoticed until production
- Client devices download bloated bundles
- No early warning system for architectural violations
- Developers unaware of cross-compilation leaks

### Real Incident (2025-02-04)
- SSR code leaked into WASM build
- Bundle grew from 2MB to 143MB
- No build-time detection
- Discovered only during manual testing

---

## Forbidden Pattern

### ❌ Forbidden: No Size Guard
```makefile
# Makefile without budget
build:
	cargo leptos build --release
	# ❌ No size validation
```

**Why this violates the rule:**
- Silent regressions
- No feedback loop
- Production incidents

---

## Canonical Pattern

### ✅ Canonical: Enforced Budget
```makefile
# Makefile with hard limit
MAX_WASM_MB := 10

build:
	cargo leptos build --release
	@$(MAKE) check-wasm-size

check-wasm-size:
	@if [ -f "target/site/pkg/app.wasm" ]; then \
		WASM_SIZE=$$(du -m target/site/pkg/app.wasm | cut -f1); \
		echo "WASM: $${WASM_SIZE}MB / $(MAX_WASM_MB)MB"; \
		if [ $$WASM_SIZE -gt $(MAX_WASM_MB) ]; then \
			echo "❌ CANON VIOLATION: WASM exceeds budget"; \
			exit 1; \
		fi; \
	fi
```

**Why this complies:**
- Explicit budget declaration
- Automatic enforcement
- Fast failure feedback
- Prevents bad merges

---

## Rationale

### Architectural Invariants
1. **Client Performance Contract**: WASM size directly impacts load time
2. **Separation Verification**: Size explosion indicates architecture leak
3. **Resource Constraints**: Mobile/low-bandwidth users

### Why Not Opinion
- 10MB WASM = ~3s download on 3G
- 143MB WASM = unusable on mobile
- Size is objective, measurable metric

### Prevention Class
- SSR code in WASM builds
- Duplicate dependencies
- Debug symbols in release
- Feature leaks

---

## Enforcement

### Build-Time (Required)
```bash
make build  # Fails if budget exceeded
```

### CI/CD (Required)
```yaml
- name: Enforce WASM Budget
  run: |
    cd products/canonrs-site
    make check-wasm-size
```

### Pre-Merge Checklist
- [ ] Build passes with size check
- [ ] WASM < declared budget
- [ ] No warnings ignored

---

## Exceptions

**No exceptions. This rule is absolute.**

Budgets may be adjusted through architectural review, but:
- Must be justified with performance analysis
- Requires explicit approval
- Budget increase ≠ rule suspension

---

## Related Rules

- **Rule #221**: Separate Build Bins by Target
- **Rule #226**: No SSR in WASM Builds
- **Rule #228**: Feature Flags Control Compilation Scope

---

## Version History

- **1.0.0** (2025-02-04) — Initial version post-143MB incident
