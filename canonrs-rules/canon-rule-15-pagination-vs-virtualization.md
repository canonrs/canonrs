# Canon Rule #15 — Pagination vs Virtualization


**Status:** ENFORCED
**Severity:** MEDIUM
**Scope:** layout, ui
**Version:** 1.0.0
**Date:** 2025-01-16

---
**Enunciado curto (para lembrar fácil):**  
Pagination é UX. Virtualization é engine. Nunca misture.

---

## Definição Formal
```
Pagination     = UX pattern de navegação em chunks (human-driven)
Virtualization = Performance engine de rendering (machine-driven)
```

**Pagination** é sobre controle humano da navegação.  
**Virtualization** é sobre performance técnica do rendering.

---

## 🔒 O QUE ESSA RULE PROÍBE (binding)

### ❌ É PROIBIDO

- Adicionar paginação ao VirtualTable
- Adicionar virtualização ao componente paginado
- Criar "DataTable com flag `paginated=true` e `virtual=true`"
- Usar paginação quando virtualização resolve
- Usar virtualização quando paginação é melhor UX
- Misturar ambos no mesmo componente

👉 **São estratégias mutuamente exclusivas.**

---

## ✅ O QUE A RULE EXIGE

Toda decisão entre Pagination e Virtualization **DEVE** considerar:

| Critério                      | Solução               |
|-------------------------------|-----------------------|
| Usuário precisa "ir para página X"? | **Pagination**  |
| Usuário precisa de deep links?      | **Pagination**  |
| SEO / indexação importa?            | **Pagination**  |
| Backend retorna em chunks?          | **Pagination**  |
| UX de "página" faz sentido?         | **Pagination**  |
| Dataset completo no client?         | **Virtualization** |
| Scroll infinito faz sentido?        | **Virtualization** |
| 10k+ rows em memória?               | **Virtualization** |
| Performance > navegação?            | **Virtualization** |

---

## 🧠 POR QUE ISSO É UMA RULE (e não só guideline)

Porque ela afeta diretamente:

| Área           | Impacto                               |
|----------------|---------------------------------------|
| UX             | Navegação discreta vs contínua        |
| Backend        | API paginada vs dataset completo      |
| SEO            | URLs indexáveis vs client-only        |
| Performance    | Chunks pequenos vs windowing          |
| Estado         | Page number vs scroll position        |
| A11y           | Página N de M vs scroll infinito      |

**Isso não é preferência de UI. É decisão arquitetural.**

---

## 🏷️ CLASSIFICAÇÃO DA RULE

| Campo       | Valor                          |
|-------------|--------------------------------|
| Rule ID     | Canon Rule #15                 |
| Categoria   | Data Navigation / Performance  |
| Tipo        | Architectural Rule             |
| Severidade  | **High**                       |
| Scope       | UI / UX / Performance / Backend|
| Violação    | **Review Blocker**             |

---

## 📊 Pagination vs Virtualization: Comparação

| Aspecto          | Pagination           | Virtualization       |
|------------------|----------------------|----------------------|
| **UX**           | Discrete (páginas)   | Continuous (scroll)  |
| **Navegação**    | "Ir para página X"   | Scroll position      |
| **Backend**      | API paginada         | Dataset completo     |
| **SEO**          | ✅ URLs indexáveis   | ❌ Client-only       |
| **Deep links**   | ✅ `/page/5`         | ❌ Não suporta       |
| **A11y**         | ✅ "Página N de M"   | ⚠️ Scroll infinito   |
| **Performance**  | Chunks pequenos      | Windowing (O(1))     |
| **Estado**       | Page number          | Scroll offset        |
| **Max items**    | Ilimitado (chunks)   | Dataset em memória   |

---

## 🧪 COMO ESSA RULE É APLICADA NA PRÁTICA

### Em Code Review

**Checklist obrigatório:**

- [ ] PR usa VirtualTable com paginação?
- [ ] PR usa DataTable paginado com 100k rows?
- [ ] PR mistura ambas estratégias?
- [ ] PR documenta a escolha Pagination vs Virtualization?

**Se falhar → PR NÃO APROVA**

---

## 🎯 QUANDO USAR CADA UM

### ✅ Use Pagination quando:

- **SEO importa:** Páginas indexáveis (`/products?page=3`)
- **Deep links:** Usuário pode compartilhar "página 5"
- **Backend paginado:** API retorna chunks (ex: `/api/users?page=2&size=20`)
- **UX de "página" faz sentido:** Catálogo, listagens, resultados de busca
- **Acessibilidade:** "Página 3 de 10" é claro
- **Controle humano:** Usuário quer ir direto para página X

**Exemplos:**
- E-commerce (produtos, categorias)
- Admin panels (usuários, pedidos)
- Resultados de busca (Google, mercados)
- Listagens públicas (blogs, notícias)

### ✅ Use Virtualization quando:

- **Dataset completo no client:** 10k-1M rows já carregados
- **Scroll infinito faz sentido:** Logs, traces, feeds
- **Performance crítica:** Rendering O(1) necessário
- **Navegação contínua:** Usuário não pensa em "páginas"
- **Desktop-first:** Scroll infinito natural
- **Dados técnicos:** Logs, métricas, analytics

**Exemplos:**
- Logs viewer (100k entries)
- Metrics dashboard (time-series)
- Distributed tracing (spans infinitos)
- Feeds (Twitter-like, scroll infinito)

---

## 🚫 ANTI-PATTERNS

### ❌ VirtualTable paginado
```rust
// PROIBIDO - conflito conceitual
<VirtualTable
    rows=logs
    pagination=true  // ❌ não existe!
/>
```

### ❌ DataTable virtualizado
```rust
// PROIBIDO - já vimos na Rule #14
<DataTable virtual=true />
```

### ❌ Componente híbrido automático
```rust
// PROIBIDO - decisão deve ser explícita
fn auto_strategy(rows: usize) {
    if rows > 1000 {
        use_virtualization()  // ❌
    } else {
        use_pagination()
    }
}
```

---

## ✅ PADRÕES CORRETOS

### Pagination (UX-driven)
```rust
#[component]
fn ProductList() -> impl IntoView {
    let page = RwSignal::new(1);
    let page_size = 20;
    
    let products = Resource::new(
        move || page.get(),
        |p| async move {
            fetch_products(p, page_size).await
        }
    );
    
    view! {
        <DataTable<Product>
            data=products
            columns=vec![/* ... */]
        />
        
        <Pagination
            current=page
            total_pages=100
            on_change=move |p| page.set(p)
        />
    }
}
```

### Virtualization (Performance-driven)
```rust
#[component]
fn LogsViewer() -> impl IntoView {
    let logs = RwSignal::new(
        fetch_all_logs() // 100k entries
    );
    
    view! {
        <VirtualTable
            rows=logs.into()
            columns=vec![/* ... */]
            row_height=36.0
            viewport_height=600.0
        />
    }
}
```

---

## 🧱 DIFERENÇA ARQUITETURAL

### Pagination

**Backend:**
```
GET /api/users?page=2&size=20
→ retorna 20 items + total_count
```

**Frontend:**
- Estado: `page: number`
- Navegação: botões "Anterior/Próxima"
- URLs: `/users?page=2`
- SEO: ✅ indexável

### Virtualization

**Backend:**
```
GET /api/logs
→ retorna TODOS os logs (ou streaming)
```

**Frontend:**
- Estado: `scroll_offset: number`
- Navegação: scroll contínuo
- URLs: não suporta deep link
- SEO: ❌ client-only

---

## 📚 Tokens e Famílias

### Pagination Component
- **Canônicos:** spacing, typography, color, state, motion
- **Família D:** navigation patterns
- **Type:** 3 (Interactive Component)

### Virtualization (VirtualTable)
- **Canônicos:** spacing, typography (mono), color
- **Família D:** data visualization
- **Type:** 4 (Performance Engine)

---

## 🔄 Casos Híbridos (Permitidos)

### ✅ Server-side Pagination + Client-side Filtering
```rust
// Backend: páginas
// Frontend: filtro local na página atual
<DataTable
    data=paginated_data
    client_filter=true
/>
<Pagination />
```

**Permitido porque:**
- Paginação no backend (chunks)
- Filtro só na página corrente
- Não há conflito conceitual

### ❌ Virtualization + Pagination
```rust
// PROIBIDO - não faz sentido
<VirtualTable />
<Pagination />
```

**Proibido porque:**
- VirtualTable já tem dataset completo
- Pagination implica chunks do backend
- Conflito arquitetural

---

## 🏁 VEREDITO FINAL

- ✅ É a **Canon Rule #15**
- ✅ É **arquitetural**, não de UX visual
- ✅ Ela **bloqueia PR errado**
- ✅ Ela protege **SEO, Backend API design, UX clarity**
- ✅ Ela força **decisão consciente e documentada**

---

## Referências

- Canon Rule #14 (DataTable vs VirtualTable): Relacionado
- Canon Rule #12 (Select vs Combobox): Padrão análogo
- Implementação VirtualTable: `/packages-rust/rs-design/src/ui/virtual_table/`
- Futuro Pagination component: `/packages-rust/rs-design/src/ui/pagination/`

---

**Mantra:** *Pagination é UX. Virtualization é engine. Nunca misture.*
