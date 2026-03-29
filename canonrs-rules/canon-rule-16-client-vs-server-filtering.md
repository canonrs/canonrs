# Canon Rule #16 — Client-side vs Server-side Filtering


**Status:** ENFORCED
**Severity:** MEDIUM
**Scope:** state, architecture
**Version:** 1.0.0
**Date:** 2025-01-16

---
**Enunciado curto (para lembrar fácil):**  
Filtering é sobre onde o dado está. Não sobre onde o usuário vê.

---

## Definição Formal
```
Client-side Filtering = Dataset completo no client, filtro local (O(n) scan)
Server-side Filtering = Backend retorna apenas filtered data (database query)
```

**Client-side** é sobre manipular dados já carregados.  
**Server-side** é sobre requisitar apenas o necessário.

---

## 🔒 O QUE ESSA RULE PROÍBE (binding)

### ❌ É PROIBIDO

- Client-side filtering com 10k+ rows
- Server-side filtering sem debounce/optimization
- Criar componente com flag `client_filter=true/false` automático
- Usar client-side quando backend suporta filtro
- Usar server-side para datasets pequenos já carregados
- Misturar ambos sem estratégia clara

👉 **A escolha deve ser EXPLÍCITA baseada em onde os dados estão.**

---

## ✅ O QUE A RULE EXIGE

Toda decisão entre Client-side e Server-side filtering **DEVE** considerar:

| Critério                          | Solução              |
|-----------------------------------|----------------------|
| Dataset completo já no client?    | **Client-side**      |
| < 500 rows totais?                | **Client-side**      |
| Filtro instantâneo necessário?    | **Client-side**      |
| Dataset > 10k rows?               | **Server-side**      |
| Backend tem índices otimizados?   | **Server-side**      |
| Dados sensíveis/privados?         | **Server-side**      |
| SEO importa (filtros em URL)?     | **Server-side**      |
| Economia de bandwidth crítica?    | **Server-side**      |

---

## 🧠 POR QUE ISSO É UMA RULE (e não só guideline)

Porque ela afeta diretamente:

| Área           | Impacto                                    |
|----------------|--------------------------------------------|
| Performance    | O(n) local vs O(log n) database            |
| Bandwidth      | Carregar tudo vs apenas necessário        |
| Latency        | Instantâneo vs network round-trip         |
| Segurança      | Expor dataset vs controle backend         |
| Escalabilidade | Limitado a memória vs database scale      |
| SEO            | Client-only vs URLs indexáveis            |

**Isso não é preferência de implementação. É decisão de arquitetura de dados.**

---

## 🏷️ CLASSIFICAÇÃO DA RULE

| Campo       | Valor                              |
|-------------|------------------------------------|
| Rule ID     | Canon Rule #16                     |
| Categoria   | Data Architecture / Performance    |
| Tipo        | Architectural Rule                 |
| Severidade  | **High**                           |
| Scope       | UI / Backend / Performance / Data  |
| Violação    | **Review Blocker**                 |

---

## 📊 Client-side vs Server-side: Comparação

| Aspecto          | Client-side              | Server-side              |
|------------------|--------------------------|--------------------------|
| **Performance**  | O(n) scan local          | O(log n) database query  |
| **Latency**      | Instantâneo (0ms)        | Network (~50-200ms)      |
| **Bandwidth**    | Dataset completo         | Apenas filtered data     |
| **Scale**        | Limitado (~500 rows)     | Ilimitado (database)     |
| **SEO**          | ❌ Client-only           | ✅ URLs indexáveis       |
| **Segurança**    | ⚠️ Expõe dataset        | ✅ Controle backend      |
| **Implementação**| Simples (`.filter()`)    | Requer API + debounce    |
| **UX**           | Feedback instantâneo     | Loading states           |

---

## 🧪 COMO ESSA RULE É APLICADA NA PRÁTICA

### Em Code Review

**Checklist obrigatório:**

- [ ] PR usa client-side filtering com 10k+ rows?
- [ ] PR usa server-side filtering para 50 rows?
- [ ] PR tem debounce em server-side filtering?
- [ ] PR expõe dados sensíveis via client-side?
- [ ] PR documenta a escolha Client vs Server filtering?

**Se falhar → PR NÃO APROVA**

---

## 🎯 QUANDO USAR CADA UM

### ✅ Use Client-side Filtering quando:

- **Dataset completo já carregado:** <500 rows totais
- **Feedback instantâneo crítico:** Admin panels, dashboards
- **Múltiplos filtros simultâneos:** Combo de filtros sem delay
- **Offline-first:** PWA, aplicações que funcionam offline
- **Dataset estático:** Não muda durante sessão

**Exemplos:**
- Admin panel de usuários (200 usuários)
- Dashboard com métricas (50 cards)
- Catálogo pequeno de produtos (300 items)
- Lista de configurações do sistema

### ✅ Use Server-side Filtering quando:

- **Dataset grande:** 10k+ rows ou crescimento ilimitado
- **Dados sensíveis:** Não deve expor dataset completo
- **SEO crítico:** Filtros devem gerar URLs indexáveis
- **Bandwidth limitado:** Mobile, conexões lentas
- **Database otimizado:** Índices preparados para queries
- **Streaming/Real-time:** Dados mudam constantemente

**Exemplos:**
- E-commerce público (milhares de produtos)
- Search engines (milhões de resultados)
- Logs/analytics (datasets massivos)
- APIs públicas com rate limiting

---

## 🚫 ANTI-PATTERNS

### ❌ Client-side com dataset grande
```rust
// PROIBIDO - 50k rows no client
let all_products = fetch_all_products().await; // 50k items

let filtered = Memo::new(move |_| {
    all_products.get()
        .iter()
        .filter(|p| p.name.contains(&search.get())) // O(50k) scan!
        .collect()
});
```

**Problema:**
- 50k rows na memória
- O(50k) scan a cada keystroke
- Bandwidth desperdiçado
- Expõe dataset completo

### ❌ Server-side sem otimização
```rust
// PROIBIDO - sem debounce
let search = RwSignal::new(String::new());

// Trigger a cada keystroke!
let filtered = Resource::new(
    move || search.get(),
    |query| async move {
        fetch_filtered(query).await // API call toda vez!
    }
);
```

**Problema:**
- API call a cada letra digitada
- Backend sobrecarregado
- UX ruim (loading states constantes)

---

## ✅ PADRÕES CORRETOS

### Client-side Filtering (Dataset pequeno)
```rust
#[component]
fn UserAdmin() -> impl IntoView {
    // Dataset pequeno (200 users) já carregado
    let users = Resource::new(|| (), |_| fetch_all_users());
    let search = RwSignal::new(String::new());
    
    // Filtro local instantâneo
    let filtered = Memo::new(move |_| {
        let query = search.get().to_lowercase();
        users.get()
            .unwrap_or_default()
            .iter()
            .filter(|u| {
                u.name.to_lowercase().contains(&query) ||
                u.email.to_lowercase().contains(&query)
            })
            .cloned()
            .collect::<Vec<_>>()
    });
    
    view! {
        <Input value=search placeholder="Search users..." />
        <DataTable<User> data=filtered />
    }
}
```

**Vantagens:**
- ✅ Instantâneo (0ms)
- ✅ Simples
- ✅ Múltiplos filtros fáceis

### Server-side Filtering (Dataset grande)
```rust
#[component]
fn ProductSearch() -> impl IntoView {
    let search = RwSignal::new(String::new());
    
    // Debounce de 300ms
    let debounced = use_debounced(search, 300.0);
    
    // API call apenas após debounce
    let products = Resource::new(
        move || debounced.get(),
        |query| async move {
            if query.is_empty() {
                vec![]
            } else {
                fetch_products_filtered(query).await
            }
        }
    );
    
    view! {
        <Input value=search placeholder="Search products..." />
        <Suspense fallback=|| view! { <Spinner /> }>
            <DataTable<Product> data=products />
        </Suspense>
    }
}
```

**Vantagens:**
- ✅ Escalável (milhões de produtos)
- ✅ Bandwidth otimizado
- ✅ Não expõe dataset
- ✅ SEO-friendly (com URLs)

---

## 🔄 Casos Híbridos (Permitidos)

### ✅ Server-side + Client-side Refinement
```rust
// Backend: query principal
// Frontend: refinamento local

let server_results = Resource::new(
    move || main_filter.get(),
    |filter| fetch_filtered(filter) // ~500 results
);

let client_refined = Memo::new(move |_| {
    server_results.get()
        .unwrap_or_default()
        .iter()
        .filter(|item| local_filter(item)) // refina localmente
        .collect()
});
```

**Permitido porque:**
- Backend reduz dataset grande → ~500 items
- Client refina apenas subset pequeno
- Melhor dos dois mundos

### ❌ Client-side de dataset grande + Server pagination
```rust
// PROIBIDO - conflito conceitual
let all_data = fetch_all_10k_rows(); // dataset completo

let filtered = all_data.filter(...); // client-side

let paginated = fetch_page(page); // server-side?
```

**Proibido porque:**
- Se já tem dataset completo, não precisa pagination
- Se precisa pagination, não deveria ter dataset completo
- Decisão contraditória

---

## 🛡️ Segurança

### ⚠️ Client-side expõe dados
```rust
// CUIDADO - todos os usuários ficam expostos no client
let all_users = fetch_all_users(); // inclui emails, roles, etc
```

**Risco:**
- Dados sensíveis no client
- Engenharia reversa do dataset
- Scraping facilitado

### ✅ Server-side protege dados
```rust
// Seguro - backend controla o que retorna
let filtered_users = fetch_filtered_users(query);
// Backend pode aplicar ACL, redação, etc
```

---

## 📚 Tokens e Implementação

### Client-side Filtering
- **Componente:** DataTable/Grid com Memo local
- **Tokens:** N/A (lógica de filtro)
- **Performance:** O(n) aceitável se n < 500

### Server-side Filtering
- **Componente:** Resource + debounce
- **Backend:** Database query com índices
- **Performance:** O(log n) database

---

## 🏁 VEREDITO FINAL

- ✅ É a **Canon Rule #16**
- ✅ É **arquitetural**, baseada em localização dos dados
- ✅ Ela **bloqueia PR errado**
- ✅ Ela protege **Performance, Bandwidth, Segurança, SEO**
- ✅ Ela força **decisão consciente sobre onde filtrar**

---

## Referências

- Canon Rule #15 (Pagination vs Virtualization): Padrão relacionado
- Canon Rule #14 (DataTable vs VirtualTable): Base conceitual
- Implementação DataTable: `/packages-rust/rs-design/src/ui/data_table/`
- Futuro debounce hook: `/packages-rust/rs-design/src/hooks/use_debounced.rs`

---

**Mantra:** *Filtering é sobre onde o dado está. Não sobre onde o usuário vê.*
