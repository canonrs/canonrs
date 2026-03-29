# CanonRS Rules — Normative System

This directory contains the **complete set of Canon Rules**.

Canon Rules are:
- **Normative** — they define correct behavior
- **Enforceable** — they can be checked by tooling
- **Immutable in intent** — changes require versioning

---

## What is a Canon Rule?

A Canon Rule is a **numbered, versioned statement** that:
1. Defines a specific architectural constraint
2. Explains the violation pattern
3. Provides the canonical solution
4. Justifies why the rule exists

Rules are **not guidelines**. They are **law**.

---

## Numbering System

Rules are numbered sequentially starting from `01`.

- `canon-rule-01-*.md` → Rule #1
- `canon-rule-42-*.md` → Rule #42
- `canon-rule-106-*.md` → Rule #106

Gaps in numbering indicate deprecated or merged rules.

---

## Rule Structure

Each rule file contains:

- **Status** — ENFORCED, PROPOSED, DEPRECATED
- **Severity** — HIGH, MEDIUM, LOW
- **Scope** — Which layer/domain it applies to
- **Version** — Semantic versioning for the rule itself
- **Principle** — The core constraint
- **Problem** — What happens when violated
- **Forbidden Patterns** — Anti-patterns
- **Canonical Pattern** — Correct implementation
- **Justification** — Why this rule exists

---

## How Tools Should Use Rules

### Linters
- Parse rule files to extract constraints
- Match code patterns against forbidden patterns
- Report violations with rule number

### Validators
- Check compliance at build time
- Fail CI if HIGH severity rules are violated
- Generate audit reports

### Documentation
- Link violations to rule numbers
- Provide rule context in error messages
- Enable searchability by rule number

---

## Violation Severity

- **HIGH** — Breaks architectural guarantees, must fix
- **MEDIUM** — Degrades system quality, should fix
- **LOW** — Style/convention issue, may fix

---

## Rule Index

See `INDEX.md` for the complete, auto-generated list of all rules.

---

## Documentation

For conceptual understanding of CanonRS, see:
```
/opt/docker/monorepo/libs/canonRS/docs/
```

This directory contains **only rules**, not explanations.

---

**CanonRS Rules**  
Normative. Enforceable. Immutable.
