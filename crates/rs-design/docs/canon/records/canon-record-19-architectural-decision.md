# Canon Record #19 — Streaming vs Snapshot

**Data:** 2024-12-30  
**Status:** Approved → Canon Rule #19  
**Scope:** Data Architecture / Real-time  

## Decisão
Streaming e Snapshot são padrões arquiteturais distintos:
- Streaming: fluxo contínuo (WebSocket/SSE)
- Snapshot: ponto-no-tempo (REST)
- Não simular streaming com polling

**Use cases claros:** Logs/chat = streaming, Lists/catalogs = snapshot
