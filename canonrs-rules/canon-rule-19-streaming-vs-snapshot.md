# Canon Rule #19 — Streaming vs Snapshot Data

**Status:** ENFORCED
**Severity:** MEDIUM
**Version:** 1.0.0
**Date:** 2025-01-16

**Category:** state-reactivity
**Tags:** data, streaming, snapshot, architecture
**Language:** EN

---

**Intro:**
Using the wrong data delivery model leads to inefficiency and complexity. Streaming and snapshot approaches must align with data behavior and requirements.

**Problem:**
data delivery model is misused causing inefficiency and complexity

**Solution:**
choose streaming or snapshot based on data update frequency and requirements

**Signals:**
- polling misuse
- latency
- over complexity

**Search Intent:**
when to use streaming vs snapshot data

**Keywords:**
streaming vs snapshot data, real time vs fetch architecture, frontend data strategy pattern, websocket vs rest decision

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