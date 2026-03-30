# Canon Rule #19 — Streaming vs Snapshot Data

**Status:** ENFORCED
**Severity:** MEDIUM
**Scope:** state, ssr
**Version:** 1.0.0
**Date:** 2025-01-16

---
**Short statement (easy to remember):**  
Streaming is continuous flow. Snapshot is fixed state.

---

## Formal Definition
```
Streaming = Continuous data (WebSocket, SSE)
Snapshot  = Point-in-time data (REST)
```

---

## 🔒 WHAT THIS RULE PROHIBITS

- Polling as fake streaming  
- Streaming for static data  
- Mixing both without strategy  

---

## Decision Matrix

| Criteria        | Solution     |
|----------------|--------------|
| Real-time      | Streaming    |
| Static data    | Snapshot     |
| SEO            | Snapshot     |

---

## 🎯 WHEN TO USE

### Streaming
```rust
use_websocket(...)
```

### Snapshot
```rust
fetch_data(...)
```

---

**Classification:** High severity, Review Blocker  
**Related:** Rule #17, Rule #20
