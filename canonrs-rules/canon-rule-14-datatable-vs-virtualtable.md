# Canon Rule #14 — DataTable vs VirtualTable

**Status:** ENFORCED
**Severity:** MEDIUM
**Scope:** state, ui, components
**Version:** 1.0.0
**Date:** 2025-01-16

---
**Short statement (easy to remember):**  
Semantics does not scale. Performance does not provide semantics.

---

## Formal Definition
```
DataTable    = Semantic table + rich UX + accessibility (Type 3)
VirtualTable = Virtual rendering engine + performance (Type 4)
```

**DataTable** is a UI component for human-scale data.  
**VirtualTable** is a rendering system for machine-scale data.

---

## 🔒 WHAT THIS RULE PROHIBITS (binding)

### ❌ FORBIDDEN

- Using VirtualTable for small tables (<1k rows)  
- Adding virtualization to DataTable  
- Creating "DataTable with `virtual=true` flag"  
- Using VirtualTable in SSR-critical contexts  
- Adding complex inline actions to VirtualTable  
- Mixing HTML semantics (`<table>`) with virtualization  

👉 **The choice must be EXPLICIT and JUSTIFIED.**

---

## ✅ WHAT THE RULE REQUIRES

Every decision between DataTable and VirtualTable **MUST** consider:

| Criteria                    | Required            |
|-----------------------------|----------------------|
| < 1,000 rows?               | **DataTable**        |
| SSR / SEO critical?         | **DataTable**        |
| Rich accessibility?         | **DataTable**        |
| Inline actions (edit/delete)? | **DataTable**      |
| 10k+ rows?                  | **VirtualTable**     |
| Logs / metrics / traces?    | **VirtualTable**     |
| Streaming / real-time?      | **VirtualTable**     |
| Performance > UX?           | **VirtualTable**     |

---

## 🧠 WHY THIS IS A RULE (not just a guideline)

Because it directly affects:

| Area            | Impact                          |
|-----------------|----------------------------------|
| Architecture    | Type 3 vs Type 4                 |
| SSR             | Full vs Client-only              |
| Performance     | O(n) vs O(1) DOM nodes           |
| Accessibility   | HTML semantics vs limited ARIA   |
| SEO             | Indexable vs Non-indexable       |
| Bundle size     | Medium vs High                   |
| Complexity      | Medium vs High                   |

**This is not a UX trade-off. It is an architectural decision.**

---

## 🏷️ RULE CLASSIFICATION

| Field       | Value                          |
|-------------|--------------------------------|
| Rule ID     | Canon Rule #14                 |
| Category    | Component Choice / Performance |
| Type        | Architectural Rule             |
| Severity    | **High**                       |
| Scope       | UI / Data / Performance / SSR  |
| Violation   | **Review Blocker**             |

---

## 📊 DataTable vs VirtualTable: Comparison

| Aspect         | DataTable           | VirtualTable        |
|-----------------|---------------------|---------------------|
| **Render**      | All rows            | Only viewport       |
| **DOM nodes**   | O(n)                | O(1)                |
| **SSR**         | ✅ Full             | ❌ Client-only      |
| **A11y**        | ✅ High             | ⚠️ Limited         |
| **Scroll**      | Normal              | Windowed            |
| **Bundle**      | ~5KB                | ~8KB                |
| **Complexity**  | Medium              | High                |
| **Max rows**    | ~1k                 | 1M+                 |
| **Tokens**      | Canonical + C       | Canonical + D       |

---

## 🧪 HOW THIS RULE IS APPLIED IN PRACTICE

### Code Review

**Checklist:**

- [ ] PR uses VirtualTable with <1k rows?  
- [ ] PR uses DataTable with 10k+ rows?  
- [ ] PR uses VirtualTable in SSR-critical?  
- [ ] PR documents the decision?  

**If fails → PR NOT APPROVED**

---

## 🧱 ARCHITECTURAL DIFFERENCE

### DataTable (Type 3)
```rust
<DataTable<User>
    data=users.get()
    columns=vec![/* ... */]
    get_id=|u| u.id
    actions=|u| view! { <EditButton /> }
 />
```

### VirtualTable (Type 4)
```rust
<VirtualTable
    rows=logs.into()
    columns=vec![/* ... */]
    row_height=36.0
    viewport_height=600.0
/>
```

---

## 🎯 CANONICAL USE CASES

### Use DataTable

- Admin panels  
- Backoffice CRUD  
- Forms  
- SEO-critical tables  

### Use VirtualTable

- Logs  
- Metrics  
- Traces  
- Big datasets  

---

## Anti-Patterns

### ❌ Virtualized DataTable
```rust
<DataTable virtual=true />
```

### ❌ VirtualTable with actions
```rust
<VirtualTable>
  actions=|row| view! {
    <EditDialog />
  }
</VirtualTable>
```

---

## 🏁 FINAL VERDICT

- ✅ Canon Rule #14  
- ✅ Architectural decision  
- ✅ Blocks wrong PRs  
- ✅ Protects SSR, Performance, A11y  

---

**Mantra:** *Semantics does not scale. Performance does not provide semantics.*
