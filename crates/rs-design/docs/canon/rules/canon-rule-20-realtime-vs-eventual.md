# Canon Rule #20 — Real-time UI vs Eventual Consistency

**Enunciado curto (para lembrar fácil):**  
Real-time é garantia. Eventual é aceitação.

---

## Definição Formal
```
Real-time UI          = Atualização imediata, garantia de sincronia (<100ms)
Eventual Consistency  = Atualização assíncrona, aceita lag (seconds/minutes)
```

---

## 🔒 O QUE ESSA RULE PROÍBE

### ❌ É PROIBIDO

- Prometer real-time sem infraestrutura (WebSocket/SSE)
- Usar eventual consistency para UX crítica (financial transactions)
- Polling agressivo (anti-pattern, não é real-time)
- Real-time desnecessário (over-engineering)

---

## ✅ DECISION MATRIX

| Critério                      | Solução                  |
|-------------------------------|--------------------------|
| Financial/critical operations | **Real-time**            |
| Collaboration (docs, chat)    | **Real-time**            |
| Live dashboards/monitoring    | **Real-time**            |
| Social media feeds            | **Eventual** (acceptable)|
| Analytics/reports             | **Eventual** (acceptable)|
| Non-critical notifications    | **Eventual** (acceptable)|

---

## 🎯 PADRÕES

### Real-time (WebSocket)
```rust
// Collaborative editing
let doc_state = use_websocket("/ws/doc/{id}");

// Immediate sync
Effect::new(move || {
    doc_state.get(); // sempre atualizado
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

## 🚫 ANTI-PATTERN: Aggressive Polling
```rust
// PROIBIDO - não é real-time, é desperdício
setInterval(() => fetch(), 100); // ❌
```

**Por que é errado:**
- Não garante <100ms (network latency)
- Sobrecarga backend/client
- Não é verdadeiro real-time

**Correto:** Use WebSocket OU aceite eventual consistency.

---

## 🧠 TRADE-OFFS

| Aspecto       | Real-time        | Eventual         |
|---------------|------------------|------------------|
| **Latency**   | <100ms           | Seconds/minutes  |
| **Infra**     | WebSocket/SSE    | REST/polling     |
| **Cost**      | Alto (conexões)  | Baixo            |
| **Complexity**| Alta             | Baixa            |
| **UX**        | Sincronia total  | Aceita lag       |

---

**Classification:** Critical severity, Review Blocker  
**Related:** Canon Rule #19 (Streaming), Rule #17 (Scale)
