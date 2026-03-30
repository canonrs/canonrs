# Canon Rule #12 — Select vs Combobox

**Status:** ENFORCED
**Severity:** MEDIUM
**Scope:** components, ui
**Version:** 1.0.0
**Date:** 2025-01-16

---
**Short statement (easy to remember):**  
Select and Combobox are semantically different components and **MUST NOT be used as substitutes for each other**.

---

## Formal Definition
```
Select   = Native HTML control (Type 1)
Combobox = Interactive component with overlay and search (Type 3)
```

---

## 🔒 WHAT THIS RULE PROHIBITS (binding)

### ❌ FORBIDDEN

- Using Combobox where Select is sufficient  
- Adding search, overlay or JS to Select  
- Creating "smart Select", "advanced Select", "SelectSearch"  
- Creating hybrid component `SelectOrCombobox`  
- Automatic choice based on heuristic (`if items > x`)  

👉 **The choice MUST be EXPLICIT and JUSTIFIED.**

---

## ✅ WHAT THE RULE REQUIRES

Every decision between Select and Combobox **MUST** consider:

| Criteria              | Required             |
|-----------------------|----------------------|
| SSR critical?         | **Select**           |
| Small list (<50)?     | **Select**           |
| Mobile-first?         | **Select**           |
| Search required?      | **Combobox**         |
| Overlay required?     | **Combobox**         |
| Rich UX > performance?| **Combobox**         |

---

## 🧠 WHY THIS IS A RULE (not just a guideline)

Because it directly affects:

| Area          | Impact                      |
|---------------|------------------------------|
| Architecture  | Type 1 vs Type 3             |
| SSR           | Native vs Client-only        |
| Performance   | O(1) vs O(n)                 |
| Accessibility | Native HTML vs manual ARIA   |
| Mobile UX     | OS picker vs overlay         |
| Bundle size   | ~2KB vs ~8KB                 |
| Governance    | Avoids emotional decisions   |

**This is not aesthetic preference. It is structural decision.**

---

## 🏷️ RULE CLASSIFICATION

| Field       | Value                          |
|-------------|--------------------------------|
| Rule ID     | Canon Rule #12                 |
| Category    | Component Choice               |
| Type        | Architectural Rule             |
| Severity    | **High**                       |
| Scope       | UI / Forms / UX / SSR          |
| Violation   | **Review Blocker**             |

---

## 🧪 HOW THIS RULE IS APPLIED IN PRACTICE

### In Code Review

**Mandatory checklist:**

- [ ] PR uses Combobox with <20 options?  
- [ ] PR uses Combobox in SSR-critical form?  
- [ ] PR documents the choice between Select vs Combobox?  

**If it fails → PR NOT APPROVED**

### In Auditing

You can run queries like:
```sql
SELECT file, component
FROM component_usage
WHERE component = 'Combobox'
  AND list_size < 10;
```

👉 This is **platform-level**, not app-level.

---

## 🧱 DOCUMENTATION STRUCTURE
```
docs/canon/
├── rules/
│   └── canon-rule-12-select-vs-combobox.md
└── records/
    └── canon-record-12-architectural-decision.md
```

- **Rule:** permanent standard  
- **Record:** decision history  

---

## 🏁 FINAL VERDICT

- ✅ It is **Canon Rule #12**  
- ✅ It is **architectural**, not stylistic  
- ✅ It **blocks wrong PRs**  
- ✅ It protects **SSR, DX, UX and performance**  
- ✅ It prevents future **component creep**  

---

## References

- Canon Record #12: `/docs/canon/records/canon-record-12-architectural-decision.md`
- Implementation: `/packages-rust/rs-design/src/ui/combobox/README.md`
- Original discussion: `/docs/canon/12-select-vs-combobox.md`
