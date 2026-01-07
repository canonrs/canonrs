# Canon Record #20 — Real-time vs Eventual Consistency

**Data:** 2024-12-30  
**Status:** Approved → Canon Rule #20  
**Scope:** UX / Data Consistency / Infrastructure  

## Decisão
Real-time requer garantias (<100ms, WebSocket).  
Eventual consistency aceita lag (seconds/minutes).

**Critical:** Financial/collaborative = real-time obrigatório  
**Acceptable:** Feeds/analytics = eventual ok

**Anti-pattern:** Polling agressivo não é real-time.
