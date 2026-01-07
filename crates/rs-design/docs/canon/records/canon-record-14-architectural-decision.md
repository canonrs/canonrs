# Canon Record #14 — DataTable vs VirtualTable: Architectural Decision

**Data:** 2024-12-30  
**Tipo:** Architectural Decision Record  
**Status:** Approved → Virou Canon Rule #14  
**Scope:** Component Choice / Performance / SSR / Data Visualization  

---

## Contexto

Esta decisão documenta a separação arquitetural entre `DataTable` (Type 3) e `VirtualTable` (Type 4), estabelecendo critérios obrigatórios para escolha entre ambos em contextos de dados human-scale vs machine-scale.

---

## Decisão

**DataTable** e **VirtualTable** são componentes fundamentalmente diferentes:

- **DataTable:** Componente semântico com UX rica (sorting, filtering, actions, SSR)
- **VirtualTable:** Engine de performance para datasets grandes (virtualização, O(1) rendering)

**NUNCA:**
- Adicionar virtualização ao DataTable
- Usar VirtualTable para listas pequenas
- Criar componente híbrido automático

---

## Rationale

### Por que não unificar?

1. **SSR incompatível:** VirtualTable depende de scroll APIs (client-only)
2. **Semântica quebra:** Virtualização requer divs, não `<table>`
3. **Acessibilidade diferente:** DataTable = ARIA completo, VirtualTable = limitado
4. **Complexidade explode:** Flags condicionais criam "god component"
5. **Tokens diferentes:** DataTable usa Família C, VirtualTable usa Família D

### Impacto da violação

- SSR quebrado em produção
- SEO perdido
- Acessibilidade degradada
- Performance ruim (DataTable com 50k rows)
- Bundle size desnecessário (VirtualTable para 10 rows)

---

## Implementação

### DataTable
- **Tipo:** 3 (Complex Interactive Component)
- **SSR:** ✅ Total
- **Bundle:** ~5KB
- **Max rows:** ~1k

### VirtualTable
- **Tipo:** 4 (System Component / Performance Engine)
- **SSR:** ❌ Client-only
- **Bundle:** ~8KB
- **Max rows:** 1M+

---

## Consequências

### Positivas
✅ Escolha explícita e documentada  
✅ SSR protegido  
✅ Performance garantida  
✅ Acessibilidade preservada  
✅ Sem over-engineering  

### Negativas
⚠️ Dois componentes para aprender  
⚠️ Decisão requer análise de contexto  

**Benefício supera custo.**

---

## Exemplos

### ✅ Correto
```rust
// Admin de usuários (200 rows, ações inline)
<DataTable<User>
    data=users
    columns=vec![/* ... */]
    actions=|u| view! { <EditButton /> }
/>

// Logs (100k entries, scroll infinito)
<VirtualTable
    rows=logs
    columns=vec![/* ... */]
    row_height=36.0
/>
```

### ❌ Errado
```rust
// DataTable com 50k rows (performance ruim)
<DataTable data=huge_dataset />

// VirtualTable com 10 rows (over-engineering)
<VirtualTable rows=small_list />
```

---

## Referências

- Canon Rule #14: `/docs/canon/rules/canon-rule-14-datatable-vs-virtualtable.md`
- Canon Rule #12 (Select vs Combobox): Padrão análogo
- Implementações: `/packages-rust/rs-design/src/ui/{data_table,virtual_table}/`

---

**Decisão final:** Componentes separados, escolha explícita, review obrigatório.
