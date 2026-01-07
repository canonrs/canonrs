# Canon Record #15 — Pagination vs Virtualization: Architectural Decision

**Data:** 2024-12-30  
**Tipo:** Architectural Decision Record  
**Status:** Approved → Virou Canon Rule #15  
**Scope:** Data Navigation / UX / Performance / Backend API Design  

---

## Contexto

Esta decisão documenta a separação arquitetural entre **Pagination** (UX pattern) e **Virtualization** (performance engine), estabelecendo que são estratégias mutuamente exclusivas.

---

## Decisão

**Pagination** e **Virtualization** não podem coexistir no mesmo componente.

- **Pagination:** Navegação discreta, backend retorna chunks, SEO-friendly
- **Virtualization:** Rendering contínuo, dataset completo no client, performance O(1)

**NUNCA:**
- Adicionar paginação ao VirtualTable
- Adicionar virtualização ao DataTable paginado
- Criar componente híbrido automático

---

## Rationale

### Por que não unificar?

1. **Backend API incompatível:**
   - Pagination: API retorna chunks (`?page=2&size=20`)
   - Virtualization: Dataset completo ou streaming

2. **UX fundamentalmente diferente:**
   - Pagination: "Ir para página 5"
   - Virtualization: Scroll infinito

3. **SEO incompatível:**
   - Pagination: URLs indexáveis (`/products?page=3`)
   - Virtualization: Client-only, sem deep links

4. **Estado diferente:**
   - Pagination: `page: number`, `total_pages: number`
   - Virtualization: `scroll_offset: number`, windowing math

### Impacto da violação

- SEO quebrado (virtualização não indexa)
- Backend confuso (API paginada + dataset completo?)
- UX ambígua (botões de página + scroll infinito?)
- Performance desperdiçada (virtualização sem necessidade)

---

## Implementação

### Pagination
- **Tipo:** UX Pattern (integra com DataTable/Grid)
- **Backend:** API paginada
- **SEO:** ✅ Deep links
- **A11y:** ✅ "Página N de M"

### Virtualization
- **Tipo:** Performance Engine (VirtualTable)
- **Backend:** Dataset completo
- **SEO:** ❌ Client-only
- **A11y:** ⚠️ Scroll infinito

---

## Consequências

### Positivas
✅ Escolha clara e documentada  
✅ Backend API bem definida  
✅ SEO preservado quando necessário  
✅ Performance otimizada quando crítico  
✅ UX previsível  

### Negativas
⚠️ Decisão requer análise de contexto  
⚠️ Não há "solução universal"  

**Benefício supera custo.**

---

## Exemplos

### ✅ Correto
```rust
// E-commerce (SEO crítico, deep links)
<DataTable data=products />
<Pagination current=page total=100 />

// Logs (100k entries, performance)
<VirtualTable rows=logs row_height=36 />
```

### ❌ Errado
```rust
// VirtualTable com paginação (conflito)
<VirtualTable />
<Pagination />

// DataTable com 100k rows (performance ruim)
<DataTable data=huge_dataset />
<Pagination />  // não resolve o problema de rendering
```

---

## Decisão Final

**Componentes separados, estratégias exclusivas, escolha explícita.**

---

## Referências

- Canon Rule #15: `/docs/canon/rules/canon-rule-15-pagination-vs-virtualization.md`
- Canon Rule #14 (DataTable vs VirtualTable): Base conceitual
- VirtualTable: `/packages-rust/rs-design/src/ui/virtual_table/`
