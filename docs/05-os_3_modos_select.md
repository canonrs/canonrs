# Select — Modos de Uso

O runtime inicializa via `init_all()` após o hydrate. O DOM deve estar completo antes do init. (CR-405)

---

## Modo 1 — Client Prefetch (dados autenticados)

`LocalResource` no `App`, `Suspense` envolve apenas o Select no `AppShell`.

```rust
let projects_res = LocalResource::new(move || async move {
    list_project_options().await.unwrap_or_default()
});

<Suspense fallback=|| view! { <PageHeader ... /> }.into_any()>
{move || Suspend::new(async move {
    let projects = projects_res.await;
    view! { <ProjectSelector projects=projects /> }.into_any()
})}
</Suspense>
```

Login deve ser hard nav (`window.location.set_href`) — garante SSR fresh com cookie. Nunca usar `navigate()` após login.

---

## Modo 2 — Client Prefetch Isolado

`LocalResource` criado dentro do próprio componente. Cada instância faz seu próprio fetch.

---

## Modo 3 — Static Select (items fixos)

Items conhecidos em tempo de compilação. SSR renderiza completo, runtime inicializa imediatamente.

```rust
<Select name="kind">
    <SelectTrigger>
        <SelectValue placeholder="Escolha...">{""}</SelectValue>
    </SelectTrigger>
    <SelectContent>
        <SelectItem value="a">"Opcao A"</SelectItem>
        <SelectItem value="b">"Opcao B"</SelectItem>
    </SelectContent>
</Select>
```

**Form support nativo:** o prop `name` propaga como `data-rs-name` no DOM. O runtime injeta `<input type="hidden">` automaticamente a cada seleção — zero `use_select`, zero signal na página.

---

## Modo 4 — Cache sessionStorage

Projetos buscados server-side no login, gravados no `sessionStorage` antes do hard nav. `LocalResource` lê o cache no mount — select aparece sem segundo fetch.

```rust
let projects_res = LocalResource::new(move || async move {
    #[cfg(feature = "hydrate")]
    {
        let cached = read_projects_cache();
        if !cached.is_empty() { return cached; }
    }
    list_project_options().await.unwrap_or_default()
});
```

---

## Comparação

| | Modo 1 | Modo 2 | Modo 3 | Modo 4 |
|---|---|---|---|---|
| Dados | API autenticada | API qualquer | Fixos | Cache + API |
| SSR do Select | Não | Não | Sim | Não |
| Fetch | LocalResource no App | LocalResource local | Nenhum | Server-side no login |
| Latência | Suspense visível | Suspense visível | Zero | Zero |

---

## Nunca fazer

- `Resource` (SSR) sem cookie → items vazios
- `spawn_local` + fetch antes de hard nav → race condition
- `navigate()` do router após login → mismatch
- `use_select` + `RwSignal` manual para form — usar `name` prop do Select
