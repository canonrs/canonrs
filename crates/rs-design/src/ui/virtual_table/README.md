# VirtualTable

Engine de renderização virtualizada para datasets grandes (10k-1M+ rows).

## ⚠️ CRITICAL: Type Classification

- **Type:** 4 (System Component / Performance Engine)
- **SSR:** ❌ Client-only (depende de scroll/viewport APIs)
- **Bundle:** ~3KB (só engine, sem libs)
- **Uso:** Machine-scale data (logs, metrics, traces, datasets)

## 🚫 O QUE NÃO É

❌ **NÃO** é um `<table>` semântico  
❌ **NÃO** tem `<thead>/<tbody>`  
❌ **NÃO** tem ARIA rica  
❌ **NÃO** substitui DataTable  
❌ **NÃO** usa para listas pequenas (<1k rows)

## ✅ O QUE É

Um **rendering system** que virtualiza o DOM:
- Renderiza apenas viewport visível (O(1) nodes)
- Calcula range com overscan
- Scroll infinito nativo
- Performance extrema

## 🔒 DESIGN CONSTRAINTS

### ❗ Variable Row Height is NOT Supported

VirtualTable **requires fixed row height by design**. This is not a limitation—it's the foundation of O(1) performance.

**Why:**
- Math engine depends on deterministic `rowHeight`
- Variable heights require O(n) measurement
- Breaks virtualization guarantees

**If you need variable heights → use DataTable**

### 📌 Header is NOT Virtualized

The header remains static and is **intentionally excluded** from virtualization.

**Rationale:**
- Header = UX semântico (labels, sorting, actions)
- Rows = performance engine
- Prevents scroll-sync hell
- Avoids alignment bugs

### ⚠️ Limited Accessibility

**VirtualTable is NOT screen-reader optimized by design.**

This is a conscious trade-off:
- Semantic HTML (`<table>`) requires all rows in DOM → breaks virtualization
- ARIA roles for virtual scrolling are complex and unreliable
- Focus management is limited

**When accessibility is mandatory → use DataTable**

---

## Canon Rule #14

**VirtualTable vs DataTable**  
Veja: `/docs/canon/rules/canon-rule-14-datatable-vs-virtualtable.md`

**Regra:** Semântica não escala. Performance não semantiza.

## Quando Usar

✅ **Use VirtualTable quando:**
- 10k+ rows
- Logs, traces, metrics, eventos
- Streaming/dados em tempo real
- Desktop-first
- Performance > UX rica
- Acessibilidade não é crítica

❌ **NÃO use VirtualTable quando:**
- <1k rows → use DataTable
- SSR crítico → use DataTable
- Ações inline complexas → use DataTable
- SEO importa → use DataTable
- Screen reader support obrigatório → use DataTable

## Tokens Aplicados

### Canônicos
- `space.sm` → padding células (0.5rem)
- `font.family.mono` → fonte de dados
- `font.size.sm` → 0.875rem
- `font.weight.semibold` → header
- `color.bg.surface` → background
- `color.bg.muted` → header
- `color.border.default` → bordas
- `radius.md` → cantos

### Família D (Data/Visualization)
- `chart.grid` → linhas da tabela
- Não usa Família C (Forms)
- Não usa Família B (Selection rica)

## Uso
```rust
use rs_design::{VirtualTable, VirtualRow, VirtualColumn, ColumnAlign};

#[component]
fn LogsViewer() -> impl IntoView {
    // Simular 100k logs
    let logs = RwSignal::new(
        (0..100_000).map(|i| VirtualRow {
            index: i,
            data: vec![
                format!("2024-12-30 {}:{}:{}", i/3600, (i/60)%60, i%60),
                "INFO".to_string(),
                format!("Log entry #{}", i),
            ],
        }).collect::<Vec<_>>()
    );

    let columns = vec![
        VirtualColumn {
            key: "timestamp".to_string(),
            width: Some(180),
            flex: None,
            align: ColumnAlign::Left,
        },
        VirtualColumn {
            key: "level".to_string(),
            width: Some(80),
            flex: None,
            align: ColumnAlign::Center,
        },
        VirtualColumn {
            key: "message".to_string(),
            width: None,
            flex: Some(1.0),
            align: ColumnAlign::Left,
        },
    ];

    view! {
        <VirtualTable
            rows=logs.into()
            columns=columns
            row_height=36.0
            viewport_height=600.0
            overscan=10
        />
    }
}
```

## Engine Interno
```
scrollTop
  ↓
visible_start = floor(scrollTop / rowHeight)
visible_end   = visible_start + viewportRows + overscan
  ↓
render rows[visible_start..visible_end]
  ↓
translateY(visible_start * rowHeight)
```

**Garantias matemáticas:**
- DOM nodes: O(1) - constante, independente de N
- Scroll: O(1) - sem layout thrashing
- Memory: O(viewport) - apenas rows visíveis

## Performance

| Métrica       | DataTable | VirtualTable |
|---------------|-----------|--------------|
| DOM nodes     | O(n)      | O(1)         |
| Scroll        | Normal    | Windowed     |
| Max rows      | ~1k       | 1M+          |
| Render time   | Linear    | Constante    |
| Memory        | O(n)      | O(viewport)  |
| A11y          | Alta      | Limitada     |

## Limitações (By Design)

### Hard Constraints
- ❌ **Sem SSR** - Client-only por natureza
- ❌ **Row height fixo** - Variável quebra virtualização
- ❌ **Sem acessibilidade rica** - Trade-off consciente
- ❌ **Sem ações inline complexas** - Use DataTable

### Soft Constraints
- ⚠️ Desktop-first (mobile precisa de ajustes)
- ⚠️ Horizontal scroll não virtualizado (futuro)
- ⚠️ Tree/nested rows não suportado (futuro)

## Extensões Futuras

- [ ] Column resizing
- [ ] Column reordering
- [ ] Horizontal scroll virtual
- [ ] Tree/nested rows (se viável sem quebrar O(1))
- [ ] Infinite loading with streaming

---

## Architecture Notes

### Why Header is Separate

**Header = UX semântico**
- Labels, sorting, filtering
- Fixed position
- Interaction layer

**Rows = Performance engine**
- Pure data rendering
- Virtualized viewport
- O(1) guarantee

**Keeping them separate:**
- ✅ Prevents scroll-sync bugs
- ✅ Simplifies alignment
- ✅ Clear separation of concerns

### Why No Variable Heights

Variable row heights require:
1. Measure all rows → O(n) scan
2. Store height cache → O(n) memory
3. Binary search on scroll → O(log n) lookup
4. Re-measure on resize → O(n) reflow

**This destroys the O(1) guarantee.**

VirtualTable chooses deterministic performance over layout flexibility.

**If you need variable heights, you need a different engine entirely.**

---

**Mantra:** *Fixed constraints enable infinite scale.*
