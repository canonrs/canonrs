# Canon Rule #20 — Real-time UI vs Eventual Consistency

**Status:** ENFORCED
**Severity:** MEDIUM
**Version:** 1.0.0
**Date:** 2025-01-16

**Category:** state-reactivity
**Tags:** realtime, consistency, state, architecture
**Language:** EN

---

**Intro:**
Confusing real-time guarantees with eventual consistency leads to incorrect system design and poor UX expectations. Each model has distinct constraints.

**Problem:**
real time and eventual consistency are misused causing incorrect ux and system design

**Solution:**
choose real time or eventual consistency explicitly based on guarantees required

**Signals:**
- polling misuse
- latency issue
- over engineering

**Search Intent:**
when to use realtime vs eventual consistency

**Keywords:**
realtime vs eventual consistency, frontend data sync strategy, websocket vs polling decision, state consistency architecture

---

**Short statement (easy to remember):**  
Real-time is guarantee. Eventual is acceptance.

---

## Formal Definition
```
Real-time UI          = Immediate update, guaranteed sync (<100ms)
Eventual Consistency  = Asynchronous update, accepts lag (seconds/minutes)
```

---

## 🔒 WHAT THIS RULE PROHIBITS

### ❌ FORBIDDEN

- Promising real-time without infrastructure (WebSocket/SSE)  
- Using eventual consistency for critical UX (financial transactions)  
- Aggressive polling (anti-pattern, not real-time)  
- Unnecessary real-time (over-engineering)  

---

## Decision Matrix

| Criteria                      | Solution                  |
|-------------------------------|--------------------------|
| Financial/critical operations | **Real-time**            |
| Collaboration (docs, chat)    | **Real-time**            |
| Live dashboards/monitoring    | **Real-time**            |
| Social media feeds            | **Eventual** (acceptable)|
| Analytics/reports             | **Eventual** (acceptable)|
| Non-critical notifications    | **Eventual** (acceptable)|

---

## 🎯 PATTERNS

### Real-time (WebSocket)
```rust
// Collaborative editing
let doc_state = use_websocket("/ws/doc/{id}");

// Immediate sync
Effect::new(move || {
    doc_state.get(); // always updated
});
```

**Guarantees:**
- <100ms latency  
- Conflict resolution  
- Guaranteed delivery  

### Eventual Consistency (Polling/Background sync)
```rust
// Social feed
let feed = Resource::new(
    || interval(60_000), // 1min refresh
    |_| fetch_feed()
);

// User accepts lag
```

**Accepts:**
- Seconds/minutes delay  
- Temporary inconsistency  
- Background reconciliation  

---

## Anti-Pattern: Aggressive Polling
```rust
// FORBIDDEN - not real-time, wasteful
setInterval(() => fetch(), 100);
```

**Why it's wrong:**
- Does not guarantee <100ms (network latency)  
- Overloads backend/client  
- Not true real-time  

**Correct:** Use WebSocket OR accept eventual consistency.

---

## Trade-Offs

| Aspect       | Real-time        | Eventual         |
|---------------|------------------|------------------|
| **Latency**   | <100ms           | Seconds/minutes  |
| **Infra**     | WebSocket/SSE    | REST/polling     |
| **Cost**      | High             | Low              |
| **Complexity**| High             | Low              |
| **UX**        | Full sync        | Accepts lag      |

---

**Classification:** Critical severity, Review Blocker  
**Related:** Canon Rule #19 (Streaming), Rule #17 (Scale)