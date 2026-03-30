# Canon Rule #15 — Pagination vs Virtualization

**Status:** ENFORCED
**Severity:** MEDIUM
**Scope:** layout, ui
**Version:** 1.0.0
**Date:** 2025-01-16

---
**Short statement (easy to remember):**  
Pagination is UX. Virtualization is an engine. Never mix them.

---

## Formal Definition
```
Pagination     = UX navigation in chunks (human-driven)
Virtualization = Rendering performance engine (machine-driven)
```

---

## 🔒 WHAT THIS RULE PROHIBITS

- Pagination inside VirtualTable  
- Virtualization inside paginated components  
- Hybrid flags (`paginated=true` + `virtual=true`)  
- Mixing both strategies  

---

## ✅ WHAT THE RULE REQUIRES

| Criteria                      | Solution          |
|-------------------------------|-------------------|
| Navigate to page X?           | Pagination        |
| Deep links needed?            | Pagination        |
| SEO important?                | Pagination        |
| Dataset in client?            | Virtualization    |
| Infinite scroll?              | Virtualization    |
| Performance critical?         | Virtualization    |

---

## 📊 Comparison

| Aspect        | Pagination | Virtualization |
|---------------|------------|----------------|
| UX            | Pages      | Scroll         |
| Backend       | Chunked    | Full dataset   |
| SEO           | ✅         | ❌             |
| Performance   | Chunked    | O(1) window    |

---

## Anti-Patterns

```rust
<VirtualTable pagination=true />
```

```rust
<DataTable virtual=true />
```

---

## 🏁 FINAL VERDICT

- ✅ Canon Rule #15  
- ✅ Architectural  
- ✅ Blocks wrong PRs  

---

**Mantra:** *Pagination is UX. Virtualization is engine.*
