# Canon Rule #19 — Streaming vs Snapshot Data

**Enunciado curto (para lembrar fácil):**  
Streaming é para fluxo contínuo. Snapshot é para estado fixo.

---

## Definição Formal
```
Streaming = Dados em fluxo contínuo (WebSocket, SSE, real-time)
Snapshot  = Dados em momento específico (REST, fetch, point-in-time)
```

---

## 🔒 O QUE ESSA RULE PROÍBE

### ❌ É PROIBIDO

- Usar snapshot polling para simular streaming (anti-pattern)
- Usar streaming para dados estáticos
- Mixing streaming + snapshot no mesmo componente sem estratégia
- Streaming sem backpressure/buffer strategy

---

## ✅ DECISION MATRIX

| Critério                    | Solução        |
|-----------------------------|----------------|
| Dados mudam constantemente  | **Streaming**  |
| Real-time crítico (<1s)     | **Streaming**  |
| Logs/metrics em tempo real  | **Streaming**  |
| Dados estáticos/cacheable   | **Snapshot**   |
| SEO/SSR crítico             | **Snapshot**   |
| Update frequency > 1min     | **Snapshot**   |

---

## 🎯 QUANDO USAR

### Streaming (WebSocket/SSE)
```rust
// Logs em tempo real
let logs_stream = use_websocket("/ws/logs");

view! {
    <VirtualTable 
        rows=logs_stream.into()
        // Append mode, não replace
    />
}
```

**Use cases:**
- Live logs/metrics
- Chat messages
- Stock prices
- Server monitoring

### Snapshot (REST/fetch)
```rust
// Dados ponto-no-tempo
let users = Resource::new(|| (), |_| fetch_users());

view! {
    <DataTable data=users />
}
```

**Use cases:**
- User lists
- Product catalogs
- Configuration data
- Historical reports

---

**Classification:** High severity, Review Blocker  
**Related:** Canon Rule #17 (Scale), Rule #20 (Real-time)
