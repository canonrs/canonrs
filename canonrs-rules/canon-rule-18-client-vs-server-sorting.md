# Canon Rule #18 — Sorting: Client vs Server

**Status:** ENFORCED
**Severity:** MEDIUM
**Scope:** state, architecture
**Version:** 1.0.0
**Date:** 2025-01-16

---
**Short statement (easy to remember):**  
Sort where the data is. Not where the user clicks.

---

## Formal Definition
```
Client-side Sorting = Local sorting (O(n log n))
Server-side Sorting = Backend ORDER BY
```

---

## 🔒 WHAT THIS RULE PROHIBITS

- Client sorting with large datasets  
- Server sorting without optimization  
- Mixing both on same dataset  

---

## Decision Matrix

| Criteria            | Solution        |
|--------------------|-----------------|
| <500 rows          | Client-side     |
| Instant feedback   | Client-side     |
| 10k+ rows          | Server-side     |
| Indexed DB         | Server-side     |

---

## 🎯 WHEN TO USE

### Client-side
```rust
data.sort_by(...)
```

### Server-side
```rust
fetch_sorted(...)
```

---

**Classification:** High severity, Review Blocker  
**Related:** Rule #16, Rule #17
