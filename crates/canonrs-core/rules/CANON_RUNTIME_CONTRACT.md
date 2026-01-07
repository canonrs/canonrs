# Canon Runtime Contract

**Purpose:** Core operational axioms for this monorepo system  
**Target:** New chat sessions, onboarding, quick context  
**Length:** ~350 words

---

## 🎯 System Axioms

### Multi-Runtime Reality
- Code executes in **server**, **browser**, and **hydrate** runtimes
- Functions may execute **multiple times** by design, not by bug
- Effects run **once per runtime**, not once total

### Context Model
- Context is **lexical**, not global
- Resolution follows **component tree hierarchy**
- Providers exist **exactly once** per type and scope

### Architecture Layers
- **Pages orchestrate** (routing, layout, data fetching)
- **Blocks execute** (UI logic, state, interactions)
- **Providers supply** (theme, auth, global state)

### Docker + Rust Reality
- Cache invalidation is **explicit**, not implicit
- MUSL resolves 90% of "impossible" container bugs
- Exit 0 ≠ successful execution (check stderr)
- Binary size matters (strip symbols, optimize)

### SSR Requirements
- IDs must be **deterministic** (no Math.random, Date.now)
- Render order must be **stable** (sort HashMaps)
- Server HTML must **match** client hydration exactly
- Client-only APIs must be **feature-gated**

### Security Model
- BFFs are **mandatory boundaries**
- Core services are **never exposed** publicly
- Auth Core is **isolated** from business logic
- mTLS for **all core-to-core** communication
- Token audience **must be validated**

---

## 🚫 When Things Break

**Hydration mismatch?** → Check Rule #27 (determinism)  
**Effect runs twice?** → Check Rule #26 (per-runtime)  
**Context not found?** → Check Rule #24 (lexical scope)  
**Provider missing?** → Check Rule #23 (exactly once)  
**Works alone, fails in compose?** → Check Rules #18-#21 (Docker cache)  

---

## 📚 Rule Index

Use this to request specific knowledge:

- **#22-#27:** Leptos runtime model
- **#08-#11:** Security (auth, mTLS, keys)
- **#03-#07:** Architecture (BFF, ports, exposure)
- **#12-#21:** Docker + Rust build system

---

**Usage:** Include this in every new chat. Request Rule Capsules on demand.
