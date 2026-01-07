# Canon Rule #14 — DataTable vs VirtualTable

**Enunciado curto (para lembrar fácil):**  
Semântica não escala. Performance não semantiza.

---

## Definição Formal
```
DataTable    = Tabela semântica + UX rica + acessibilidade (Type 3)
VirtualTable = Engine de renderização virtual + performance (Type 4)
```

**DataTable** é um componente de UI para dados human-scale.  
**VirtualTable** é um sistema de renderização para dados machine-scale.

---

## 🔒 O QUE ESSA RULE PROÍBE (binding)

### ❌ É PROIBIDO

- Usar VirtualTable para tabelas pequenas (<1k rows)
- Adicionar virtualização ao DataTable
- Criar "DataTable com flag `virtual=true`"
- Usar VirtualTable em contextos SSR-críticos
- Adicionar ações inline complexas ao VirtualTable
- Misturar semântica HTML (`<table>`) com virtualização

👉 **A escolha deve ser EXPLÍCITA e JUSTIFICADA.**

---

## ✅ O QUE A RULE EXIGE

Toda decisão entre DataTable e VirtualTable **DEVE** considerar:

| Critério                    | Obrigatório            |
|-----------------------------|------------------------|
| < 1.000 rows?               | **DataTable**          |
| SSR / SEO crítico?          | **DataTable**          |
| Acessibilidade rica?        | **DataTable**          |
| Ações inline (edit/delete)? | **DataTable**          |
| 10k+ rows?                  | **VirtualTable**       |
| Logs / métricas / traces?   | **VirtualTable**       |
| Streaming / tempo real?     | **VirtualTable**       |
| Performance > UX?           | **VirtualTable**       |

---

## 🧠 POR QUE ISSO É UMA RULE (e não só guideline)

Porque ela afeta diretamente:

| Área            | Impacto                          |
|-----------------|----------------------------------|
| Arquitetura     | Type 3 vs Type 4                 |
| SSR             | Total vs Client-only             |
| Performance     | O(n) vs O(1) DOM nodes           |
| Acessibilidade  | Semântica HTML vs ARIA limitado  |
| SEO             | Indexável vs Não-indexável       |
| Bundle size     | Médio vs Alto                    |
| Complexidade    | Média vs Alta                    |

**Isso não é trade-off de UX. É escolha arquitetural.**

---

## 🏷️ CLASSIFICAÇÃO DA RULE

| Campo       | Valor                          |
|-------------|--------------------------------|
| Rule ID     | Canon Rule #14                 |
| Categoria   | Component Choice / Performance |
| Tipo        | Architectural Rule             |
| Severidade  | **High**                       |
| Scope       | UI / Data / Performance / SSR  |
| Violação    | **Review Blocker**             |

---

## 📊 DataTable vs VirtualTable: Comparação

| Aspecto         | DataTable           | VirtualTable        |
|-----------------|---------------------|---------------------|
| **Render**      | Todas as rows       | Só viewport         |
| **DOM nodes**   | O(n)                | O(1)                |
| **SSR**         | ✅ Total            | ❌ Client-only      |
| **A11y**        | ✅ Alta             | ⚠️ Limitada         |
| **Scroll**      | Normal              | Windowed            |
| **Bundle**      | ~5KB                | ~8KB                |
| **Complexidade**| Média               | Alta                |
| **Max rows**    | ~1k                 | 1M+                 |
| **Tokens**      | 100% canônicos + C  | Canônicos + D       |

---

## 🧪 COMO ESSA RULE É APLICADA NA PRÁTICA

### Em Code Review

**Checklist obrigatório:**

- [ ] PR usa VirtualTable com <1k rows?
- [ ] PR usa DataTable com 10k+ rows?
- [ ] PR usa VirtualTable em SSR-crítico?
- [ ] PR documenta a escolha DataTable vs VirtualTable?

**Se falhar → PR NÃO APROVA**

### Em Auditoria

Query para detectar violações:
```sql
-- DataTable com muitas rows (performance issue)
SELECT file, component, row_count
FROM component_usage
WHERE component = 'DataTable'
  AND row_count > 1000;

-- VirtualTable em SSR (architectural violation)
SELECT file, component, ssr_context
FROM component_usage
WHERE component = 'VirtualTable'
  AND ssr_context = 'critical';
```

---

## 🧱 DIFERENÇA ARQUITETURAL

### DataTable (Type 3)
```rust
// Semântico, SSR-safe, acessível
<DataTable<User>
    data=users.get()
    columns=vec![/* ... */]
    get_id=|u| u.id
    actions=|u| view! { <EditButton /> }  // ✅ OK
/>
```

**Características:**
- Renderiza todas as rows no DOM
- `<table>`, `<thead>`, `<tbody>` semânticos
- ARIA completo
- SSR total
- Ações inline ricas
- Família C (Forms) + B (Selection)

### VirtualTable (Type 4)
```rust
// Performance engine, client-only
<VirtualTable
    rows=logs.into()  // 100k+ rows
    columns=vec![/* ... */]
    row_height=36.0
    viewport_height=600.0
/>
```

**Características:**
- Renderiza apenas viewport (O(1) nodes)
- Sem `<table>` semântico (divs + flex)
- ARIA limitado
- Client-only (scroll APIs)
- Sem ações inline complexas
- Família D (Data/Visualization)

---

## 🎯 CASOS DE USO CANÔNICOS

### ✅ Use DataTable

- **Admin panels:** Lista de usuários, pedidos, configs
- **Backoffice:** CRUDs, auditoria, billing
- **Forms:** Seleção de itens, listas relacionadas
- **SEO-critical:** Tabelas indexáveis

**Exemplo:** Tabela de produtos no admin (200 items, edit/delete inline)

### ✅ Use VirtualTable

- **Logs:** Application logs, server logs, audit trails
- **Métricas:** Time-series data, analytics
- **Traces:** Distributed tracing, spans
- **Datasets:** Big data visualization, CSVs grandes

**Exemplo:** Visualizador de logs (100k entries, scroll infinito)

---

## 🚫 ANTI-PATTERNS

### ❌ DataTable virtualizado
```rust
// PROIBIDO
<DataTable virtual=true />  // Não existe!
```

### ❌ VirtualTable com ações ricas
```rust
// PROIBIDO - VirtualTable não suporta isso bem
<VirtualTable>
  actions=|row| view! {
    <EditDialog />
    <DeleteConfirm />
  }
</VirtualTable>
```

### ❌ Escolha por heurística automática
```rust
// PROIBIDO - escolha deve ser explícita
fn auto_table(rows: usize) {
    if rows > 1000 {
        VirtualTable::new()  // ❌
    } else {
        DataTable::new()
    }
}
```

---

## 📚 Tokens Aplicados

### DataTable
- **Canônicos:** spacing, typography, color, state, motion
- **Família C:** field.*, validation.*
- **Família B:** selection.*, list.*

### VirtualTable
- **Canônicos:** spacing, typography, color, motion
- **Família D:** chart.grid, data visualization
- **NÃO usa:** Família C (forms), Família B (selection rica)

---

## 🏁 VEREDITO FINAL

- ✅ É a **Canon Rule #14**
- ✅ É **arquitetural**, não de UX
- ✅ Ela **bloqueia PR errado**
- ✅ Ela protege **SSR, Performance, A11y**
- ✅ Ela evita **component creep** e **over-engineering**

---

## Referências

- Canon Record #14: `/docs/canon/records/canon-record-14-architectural-decision.md`
- Implementação DataTable: `/packages-rust/rs-design/src/ui/data_table/`
- Implementação VirtualTable: `/packages-rust/rs-design/src/ui/virtual_table/`
- Canon Rule #12 (Select vs Combobox): `/docs/canon/rules/canon-rule-12-select-vs-combobox.md`

---

**Mantra:** *Semântica não escala. Performance não semantiza.*
