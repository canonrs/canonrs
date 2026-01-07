# Canon Record #16 — Client-side vs Server-side Filtering: Architectural Decision

**Data:** 2024-12-30  
**Tipo:** Architectural Decision Record  
**Status:** Approved → Virou Canon Rule #16  
**Scope:** Data Architecture / Performance / Security / Bandwidth  

---

## Contexto

Esta decisão documenta a separação entre **Client-side Filtering** (dados já carregados) e **Server-side Filtering** (backend retorna filtered), estabelecendo critérios baseados em onde os dados estão.

---

## Decisão

**Client-side** e **Server-side filtering** são estratégias diferentes baseadas em:
1. Tamanho do dataset
2. Localização dos dados
3. Requisitos de performance/bandwidth/segurança

**Regra:**
- Dataset < 500 rows já no client → **Client-side**
- Dataset > 10k rows → **Server-side obrigatório**
- 500-10k rows → **Análise de contexto**

**NUNCA:**
- Client-side filtering com 10k+ rows
- Server-side filtering sem debounce
- Decisão automática sem análise

---

## Rationale

### Por que importa onde filtrar?

1. **Performance:**
   - Client: O(n) scan, mas instantâneo se n pequeno
   - Server: O(log n) database, mas network latency

2. **Bandwidth:**
   - Client: carrega dataset completo
   - Server: apenas dados necessários

3. **Segurança:**
   - Client: expõe dataset completo
   - Server: controle de acesso/redação

4. **Escalabilidade:**
   - Client: limitado a memória (~500 rows)
   - Server: ilimitado (database scale)

---

## Implementação

### Client-side
- Memo/Signal com `.filter()`
- Feedback instantâneo
- Dataset < 500 rows

### Server-side
- Resource + debounce (300ms)
- API query otimizada
- Dataset > 10k rows

---

## Consequências

### Positivas
✅ Performance otimizada  
✅ Bandwidth controlado  
✅ Segurança preservada  
✅ Escalabilidade garantida  
✅ UX apropriada (instant vs loading)  

### Negativas
⚠️ Decisão requer análise  
⚠️ Server-side mais complexo (debounce, API)  

**Benefício supera custo.**

---

## Decisão Final

**Escolha baseada em onde os dados estão e tamanho do dataset.**

---

## Referências

- Canon Rule #16: `/docs/canon/rules/canon-rule-16-client-vs-server-filtering.md`
- Canon Rule #15 (Pagination vs Virtualization): Relacionado
