# Canon Rule #18 — Sorting: Client vs Server

**Enunciado curto (para lembrar fácil):**  
Sort onde os dados estão. Não onde o usuário clica.

---

## Definição Formal
```
Client-side Sorting = Dataset completo no client, sort local (O(n log n))
Server-side Sorting = Backend retorna sorted data (database ORDER BY)
```

**Análogo à Rule #16 (Filtering), mas para ordenação.**

---

## 🔒 O QUE ESSA RULE PROÍBE

### ❌ É PROIBIDO

- Client-side sorting com 10k+ rows
- Server-side sorting sem otimização (índices)
- Mixing client and server sorting no mesmo dataset
- Sorting machine-scale data no client

---

## ✅ DECISION MATRIX

| Critério                     | Solução            |
|------------------------------|--------------------|
| Dataset < 500 rows no client | **Client-side**    |
| Sorting instantâneo crítico  | **Client-side**    |
| Dataset > 10k rows           | **Server-side**    |
| Database tem índices         | **Server-side**    |
| SEO (sorted URLs)            | **Server-side**    |

---

## 🎯 QUANDO USAR

### Client-side Sorting
```rust
// Dataset pequeno, sort instantâneo
let sorted = Memo::new(move |_| {
    let mut data = users.get();
    data.sort_by(|a, b| a.name.cmp(&b.name));
    data
});
```

### Server-side Sorting
```rust
// Dataset grande, query otimizada
let sorted_users = Resource::new(
    move || (sort_field.get(), sort_order.get()),
    |(field, order)| async move {
        fetch_users_sorted(field, order).await
    }
);
```

---

**Classification:** High severity, Review Blocker  
**Related:** Canon Rule #16 (Filtering), Rule #17 (Scale)
