# Canon Rule #16 — Client-side vs Server-side Filtering

**Status:** ENFORCED
**Severity:** MEDIUM
**Scope:** state, architecture
**Version:** 1.0.0
**Date:** 2025-01-16

---
**Short statement (easy to remember):**  
Filtering is about where the data is, not where the user sees it.

---

## Formal Definition
```
Client-side = Local filtering (O(n))
Server-side = Backend query filtering
```

---

## 🔒 WHAT THIS RULE PROHIBITS

- Client filtering with large datasets  
- Server filtering without debounce  
- Automatic flags (`client_filter=true`)  
- Mixing both without strategy  

---

## ✅ WHAT THE RULE REQUIRES

| Criteria                    | Solution        |
|----------------------------|-----------------|
| Dataset in client?         | Client-side     |
| <500 rows?                 | Client-side     |
| 10k+ rows?                 | Server-side     |
| Sensitive data?            | Server-side     |
| SEO needed?                | Server-side     |

---

## 📊 Comparison

| Aspect        | Client-side | Server-side |
|---------------|-------------|-------------|
| Performance   | O(n)        | O(log n)    |
| Latency       | 0ms         | Network     |
| Scale         | Limited     | Unlimited   |
| Security      | Low         | High        |

---

## Anti-Patterns

```rust
let filtered = all_data.filter(...)
```

```rust
Resource::new(search, fetch)
```

---

## 🏁 FINAL VERDICT

- ✅ Canon Rule #16  
- ✅ Architectural  
- ✅ Protects performance and security  

---

**Mantra:** *Filtering depends on where the data lives.*
