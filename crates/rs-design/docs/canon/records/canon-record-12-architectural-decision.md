# Canon Record #12 — Select vs Combobox: Architectural Decision

**Data:** 2024-12-30  
**Tipo:** Architectural Decision Record  
**Status:** Approved → Virou Canon Rule #12  
**Scope:** Component Choice / SSR / Performance / UX  

---

## Contexto

Esta decisão documenta a separação arquitetural entre `Select` (Type 1) e `Combobox` (Type 3), estabelecendo critérios obrigatórios para escolha entre ambos.

---

# Canon Record #12: Select vs Combobox

**Decisão:** Select e Combobox são componentes distintos e não intercambiáveis  
**Data:** 2025-12-30  
**Status:** DEFINITIVO  

## Regra Fundamental

Select = HTML nativo + tokens canônicos (42 tokens, Família C)
Combobox = Overlay + busca + interativo (64 tokens, Família C+B+A)

NUNCA unificar os dois.

## Quando Usar

**Select:** < 50 itens, SSR crítico, mobile-first, sem busca
**Combobox:** > 50 itens, busca obrigatória, UX rica, desktop-first

## Diferenças Técnicas

| Aspecto | Select | Combobox |
|---------|--------|----------|
| Type | 1 (Pure) | 3 (Complex) |
| Famílias | C | C + B + A |
| Tokens | 42 | 64 |
| SSR | 100% | Parcial |
| Bundle | ~2KB | ~8KB |

Ver arquivo completo para detalhes técnicos.
