# Canon Rule #17 — Human-scale vs Machine-scale Components

**Status:** ENFORCED
**Severity:** HIGH
**Version:** 1.0.0
**Date:** 2025-01-16

**Category:** component-architecture
**Tags:** scale, components, performance, architecture
**Language:** EN

---

**Intro:**
Choosing components without considering scale leads to performance issues or unnecessary complexity. Human-scale and machine-scale components have fundamentally different constraints.

**Problem:**
components are used outside their intended scale causing performance or complexity issues

**Solution:**
choose components explicitly based on data scale and cognitive constraints

**Signals:**
- slow rendering
- over engineering
- wrong component

**Search Intent:**
when to use human scale vs

**Keywords:**
human vs machine scale components, frontend scale architecture, ui performance scaling pattern, component selection based on scale

---

**Short statement (easy to remember):**  
Build for humans or build for machines. Never pretend one is the other.

---

## 🧠 THE META-RULE

This is the **conceptual generalization** that underpins all previous Canon Rules:

| Canon Rule | Human-scale | Machine-scale |
|------------|-------------|---------------|
| **#12** | Select | Combobox |
| **#14** | DataTable | VirtualTable |
| **#15** | Pagination | Virtualization |
| **#16** | Client Filtering | Server Filtering |
| **#18** | Client Sorting | Server Sorting |
| **#19** | Snapshot | Streaming |
| **#20** | Eventual Consistency | Real-time |

**Universal pattern:**  
When you cross from human-scale → machine-scale, **the component fundamentally changes**.

---

## Formal Definition
```
Human-scale  = Components for quantities humans process (~1-1000)
Machine-scale = Components for quantities machines process (10k-1M+)
```

**Human-scale** is about **human cognition, rich UX, semantics**.  
**Machine-scale** is about **technical performance, density, scalability**.

---

## 🔒 WHAT THIS RULE PROHIBITS

### ❌ FORBIDDEN

- Using human-scale components for machine-scale data  
- Adding flags `virtual=true`, `big_data=true` to human-scale components  
- Creating "smart components" that auto-select scale  
- Using machine-scale components for human-scale data (over-engineering)  
- Scaling human components beyond cognitive limits  
- Downscaling machine components losing performance  

👉 **Scale is an architectural decision, not configuration.**

---

## ✅ WHAT THE RULE REQUIRES

Every component decision **MUST** consider data scale:

| Aspect                    | Human-scale      | Machine-scale    |
|----------------------------|------------------|------------------|
| **Quantity**               | 1-1000           | 10k-1M+          |
| **Cognition**              | Human processes  | Machine processes |
| **UX**                     | Rich, semantic   | Performance-driven |
| **HTML semantics**         | Full             | Limited/Div-based |
| **Accessibility**          | Full             | Partial          |
| **SSR**                    | Preferred        | Optional         |
| **Performance target**     | UX > performance | Performance > UX |

---

## The Threshold Rule

**When crossing the threshold, the component changes nature.**

### Canonical Thresholds
```
Human-scale:    1 -----------------> ~1000
Machine-scale:  10k ----------------> 1M+
```

**Gray zone (1k-10k):**  
Conscious decision area based on:
- UX vs performance  
- SSR needs  
- Accessibility requirements  

---

## 📊 Fundamental Divide

| Aspect              | Human-scale                | Machine-scale              |
|----------------------|----------------------------|----------------------------|
| Target               | Human cognition            | Technical performance      |
| Quantity             | 1-1000                     | 10k-1M+                    |
| UX                   | Rich                       | Efficient                  |
| Semantics            | Native HTML                | Optimized divs             |
| Accessibility        | Full                       | Limited                    |
| SSR                  | ✅ Preferred               | ⚠️ Optional/impossible     |
| Performance          | O(n) acceptable            | O(1) required              |
| Navigation           | Discrete                   | Continuous                 |
| Type                 | Type 1-3                   | Type 4                     |

---

## 🚫 ANTI-PATTERNS

### ❌ Scaling human component
```rust
<DataTable data=fifty_thousand_rows />
```

### ❌ Using machine component for small data
```rust
<VirtualTable rows=ten_items />
```

### ❌ Universal component
```rust
<Table 
    data=rows
    virtual={rows.len() > 1000}
    paginated={rows.len() > 100}
/>
```

---

## ✅ CORRECT PATTERNS
```rust
<DataTable<User>
    data=users
/>

<VirtualTable
    rows=logs
/>
```

---

## 🏷️ RULE CLASSIFICATION

| Field       | Value                              |
|-------------|------------------------------------|
| Rule ID     | Canon Rule #17                     |
| Category    | Meta-Rule / Design Philosophy      |
| Type        | Foundational Architectural Rule    |
| Severity    | **Critical**                       |
| Scope       | System-wide                        |
| Violation   | **Review Blocker**                 |

---

## 🧬 DNA OF SCALE

**Human-scale:**
```
Semantics → Cognition → UX → SSR → Accessibility
```

**Machine-scale:**
```
Performance → Efficiency → Scaling → Client-only → Trade-offs
```

---

## 🏁 FINAL VERDICT

- ✅ Canon Rule #17  
- ✅ Meta-rule  
- ✅ Explains all others  
- ✅ Blocks bad architecture  

---

**Mantra:** *Build for humans or build for machines.*