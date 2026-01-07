# Grid

Layout component para visualização de múltiplos itens independentes (cards, tiles, dashboards).

## 🏷️ Type Classification

- **Type:** 2 (Layout Component - Pure)
- **SSR:** ✅ Total (sem estado, só children)
- **Bundle:** ~0.2KB (CSS Grid wrapper)
- **Uso:** Dashboards, cards, galerias, marketplaces, kanban boards

---

## 🧠 O QUE É

Grid é um **layout system** orientado a cards/células para dados human-scale.

**Grid ≠ Tabela**  
Grid renderiza **itens independentes**, não linhas tabulares.

---

## 🚫 O QUE NÃO É

❌ **NÃO** é DataTable  
❌ **NÃO** usa `<table>`, `<thead>`, `<tbody>`  
❌ **NÃO** tem virtualização  
❌ **NÃO** conhece dados (só children)  
❌ **NÃO** faz sorting/filtering  
❌ **NÃO** substitui VirtualTable  

---

## ✅ Quando Usar

✅ **Use Grid quando:**
- UI card-based (cards, tiles, summaries)
- Dashboards
- Marketplace / catálogo de produtos
- Boards (kanban, status boards)
- Galerias de imagens/vídeos
- < 500 itens
- Responsividade importa
- UX visual > densidade tabular

❌ **NÃO use Grid quando:**
- Precisa ler linhas e colunas → use **DataTable**
- Precisa sorting por coluna → use **DataTable**
- Precisa de 10k+ items → use **VirtualTable**
- Precisa semântica `<table>` → use **DataTable**

---

## 📊 Grid vs DataTable vs VirtualTable

| Aspecto       | Grid          | DataTable     | VirtualTable  |
|---------------|---------------|---------------|---------------|
| **Estrutura** | Cards/Tiles   | Linhas/Colunas| Rows virtuais |
| **Semântica** | Divs          | `<table>`     | Divs          |
| **SSR**       | ✅ Total      | ✅ Total      | ❌ Client     |
| **Max items** | ~500          | ~1k           | 1M+           |
| **Sorting**   | ❌            | ✅            | Limited       |
| **Layout**    | Responsive    | Tabular       | Fixed         |
| **UX**        | Visual/Cards  | Densidade     | Performance   |

---

## Tokens Aplicados

### Canônicos
- `space.xs` → gap-xs (0.25rem)
- `space.sm` → gap-sm (0.5rem)
- `space.md` → gap-md (1rem)
- `space.lg` → gap-lg (1.5rem)
- `space.xl` → gap-xl (2rem)

### Layout (futuros tokens)
- `layout.grid.columns.*` → cols config
- `layout.grid.breakpoints` → responsive breakpoints

### NÃO Usa
- ❌ Família C (Forms)
- ❌ Família B (Selection)
- ❌ Família D (Data engine)

**Grid é layout puro, não input, não data engine.**

---

## Uso

### Exemplo 1: Dashboard com cards
```rust
use rs_design::{Grid, GridCols, GridGap};

#[component]
fn Dashboard() -> impl IntoView {
    let metrics = vec![
        ("Users", "1,234"),
        ("Revenue", "$45,678"),
        ("Orders", "890"),
        ("Growth", "+12%"),
    ];

    view! {
        <Grid cols=GridCols::Responsive gap=GridGap::Md>
            {metrics.into_iter().map(|(label, value)| {
                view! {
                    <div class="p-6 bg-surface border border-border rounded-md shadow-sm">
                        <h3 class="text-sm text-fg-muted">{label}</h3>
                        <p class="text-3xl font-bold mt-2">{value}</p>
                    </div>
                }
            }).collect::<Vec<_>>()}
        </Grid>
    }
}
```

### Exemplo 2: Marketplace de produtos
```rust
#[component]
fn ProductGrid() -> impl IntoView {
    let products = use_context::<RwSignal<Vec<Product>>>().unwrap();

    view! {
        <Grid cols=GridCols::Fixed(3) gap=GridGap::Lg>
            {move || products.get().iter().map(|product| {
                view! {
                    <ProductCard product=product.clone() />
                }
            }).collect::<Vec<_>>()}
        </Grid>
    }
}

#[component]
fn ProductCard(product: Product) -> impl IntoView {
    view! {
        <div class="border border-border rounded-lg overflow-hidden hover:shadow-lg transition">
            <img src=product.image class="w-full h-48 object-cover" />
            <div class="p-4">
                <h3 class="font-semibold">{product.name}</h3>
                <p class="text-2xl font-bold mt-2">{product.price}</p>
                <button class="mt-4 w-full btn-primary">"Add to Cart"</button>
            </div>
        </div>
    }
}
```

### Exemplo 3: Auto-fit responsivo
```rust
view! {
    <Grid 
        cols=GridCols::Auto { min: 250 }
        gap=GridGap::Md
    >
        {items.into_iter().map(|item| {
            view! { <ItemCard item=item /> }
        }).collect::<Vec<_>>()}
    </Grid>
}
```

---

## GridCols Options

### `Fixed(n)`
Colunas fixas em todas as telas.
```rust
GridCols::Fixed(3)  // sempre 3 colunas
```

### `Responsive`
Adapta automaticamente:
- Mobile (< 640px): 1 coluna
- Tablet (640-768px): 2 colunas
- Desktop (768-1024px): 3 colunas
- Wide (> 1024px): 4 colunas
```rust
GridCols::Responsive
```

### `Auto { min }`
Auto-fit com largura mínima.
```rust
GridCols::Auto { min: 200 }  // min 200px por item
```

---

## GridGap Options
```rust
GridGap::None  // 0
GridGap::Xs    // 0.25rem (space.xs)
GridGap::Sm    // 0.5rem  (space.sm)
GridGap::Md    // 1rem    (space.md) ← padrão
GridGap::Lg    // 1.5rem  (space.lg)
GridGap::Xl    // 2rem    (space.xl)
```

---

## Limites Recomendados

| Métrica       | Limite        |
|---------------|---------------|
| Items         | ~300-500      |
| DOM nodes     | Linear OK     |
| Mobile        | ✅ Ótimo      |
| Desktop       | ✅ Ótimo      |
| SSR           | ✅ Total      |

**Se passou de 500 items:** avaliar **VirtualTable** ou paginação.

---

## Anti-Patterns

### ❌ Grid como tabela disfarçada
```rust
// ERRADO - isso é DataTable mal feito
<Grid>
  Nome | Email | Status | Ações
</Grid>
```

### ✅ Grid correto (cards independentes)
```rust
// CORRETO - cards visuais
<Grid>
  <UserCard />
  <UserCard />
  <UserCard />
</Grid>
```

---

## Design Principles

1. **Grid NÃO conhece dados** - Só layout e children
2. **Grid NÃO faz virtualização** - Use VirtualTable se precisar
3. **Grid NÃO faz sorting/filtering** - Isso é responsabilidade do parent
4. **Grid é stateless** - Pure layout component

---

## Analogias Corretas

- **Grid** ≈ Dashboard / Cards / Kanban
- **DataTable** ≈ Excel / Spreadsheet
- **VirtualTable** ≈ Pandas / Big Data Engine

---

## SSR Safety

✅ **100% SSR-Safe** porque:
- Sem estado
- Sem browser APIs
- Só children + CSS Grid
- Render idêntico server/client

---

## Future Enhancements

- [ ] Masonry layout variant
- [ ] Drag & drop reordering (Type 3 upgrade)
- [ ] Collapse/expand groups (Type 3 upgrade)
- [ ] Virtual scrolling (seria outro componente)

---

**Definição Canon:**  
*Grid é um componente de layout para visualização de múltiplos itens independentes, orientado a cards, responsivo, SSR-safe, sem semântica tabular e sem virtualização.*
