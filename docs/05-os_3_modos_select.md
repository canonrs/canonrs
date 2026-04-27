# Os 3 Modos do Select

## Contexto

O `ProjectSelector` usa `data-rs-interaction="selection"` — o runtime inicializa via `init_all()` uma única vez após o hydrate. O DOM deve estar completo antes do init. (CR-405)

---

## Modo 1 — Client Prefetch (padrão para dados autenticados)

Usado no `AppShell` para o `ProjectSelector` com dados da API.

```rust
// lib.rs — App
let projects_res = LocalResource::new(move || async move {
    list_project_options().await.unwrap_or_default()
});

// AppShell — Suspense envolve apenas o Select
<Suspense fallback=|| view! { <PageHeader ... /> }.into_any()>
{move || Suspend::new(async move {
    let projects = projects_res.await;
    view! { <ProjectSelector projects=projects /> }.into_any()
})}
</Suspense>
```

**Fluxo:**
1. Login → hard nav (`window.location.set_href`)
2. SSR renderiza AppShell **sem** o Select
3. Hydrate → `LocalResource` faz fetch com cookie disponível
4. `Suspense` resolve → Select monta com dados completos
5. Runtime `init_all()` inicializa o Select via `MutationObserver`

**Regras:**
- `LocalResource` — nunca roda no SSR, zero hydration mismatch
- Login deve ser hard nav — garante SSR fresh com cookie
- Nunca usar `navigate()` do router após login — causa race condition

---

## Modo 2 — Client Prefetch Isolado (componente standalone)

Usado quando o Select precisa de dados da API mas vive fora do AppShell.

```rust
#[component]
fn PrefetchSelectDemo() -> impl IntoView {
    let projects_res = LocalResource::new(|| async move {
        list_project_options().await.unwrap_or_default()
    });
    view! {
        <Suspense fallback=|| view! { "Carregando..." }>
            {move || Suspend::new(async move {
                let projects = projects_res.await;
                view! { <ProjectSelector projects=projects /> }
            })}
        </Suspense>
    }
}
```

**Diferença do Modo 1:** o `LocalResource` é criado dentro do componente, não no `App`. Cada instância faz seu próprio fetch.

---

## Modo 3 — Static Select (items fixos)

Usado quando os dados não vêm de API — items conhecidos em tempo de compilação.

```rust
<Select>
    <SelectTrigger>
        <SelectValue placeholder="Escolha...">{""}</SelectValue>
    </SelectTrigger>
    <SelectContent>
        <SelectItem value="a" selected=SelectionState::Unselected>"Opcao A"</SelectItem>
        <SelectItem value="b" selected=SelectionState::Unselected>"Opcao B"</SelectItem>
    </SelectContent>
</Select>
```

**Fluxo:** SSR renderiza o Select completo → hydrate → `init_all()` encontra o DOM pronto → funciona imediatamente.

---

## Comparação

| | Modo 1 | Modo 2 | Modo 3 |
|---|---|---|---|
| Dados | API autenticada | API qualquer | Fixos |
| SSR do Select | Não | Não | Sim |
| Fetch | LocalResource no App | LocalResource local | Nenhum |
| Hydration mismatch | Impossível | Impossível | Impossível |
| Init runtime | Via MutationObserver | Via MutationObserver | Via init_all direto |

---

## O que nunca fazer

- `Resource` (SSR) sem cookie disponível no contexto → items vazios no SSR
- Hard nav com `spawn_local` + fetch antes da navegação → race condition com cookie
- Renderizar Select com dados síncronos no SSR enquanto o cliente usa cache → mismatch

---

## Modo 4 — Cache sessionStorage (login → dashboard sem fetch duplo)

Usado quando o usuário faz login e vai direto para o dashboard. Os projetos são buscados server-side no momento do login e gravados no `sessionStorage` antes do hard nav. O `LocalResource` lê o cache no mount — select aparece instantaneamente sem segundo fetch.

**login.rs — server fn retorna projetos:**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResult {
    pub success:  bool,
    pub error:    Option<String>,
    pub projects: Vec<ProjectOption>, // buscados server-side com o token
}
```

**login.rs — Effect grava cache e navega:**
```rust
Effect::new(move |_| {
    if let Some(Ok(result)) = login_action.value().get() {
        if result.success {
            #[cfg(feature = "hydrate")]
            {
                cache_projects_before_nav(&result.projects);
                let window = leptos::web_sys::window().unwrap();
                let _ = window.location().set_href("/dashboard");
            }
        }
    }
});
```

**project_selector.rs — cache functions:**
```rust
#[cfg(feature = "hydrate")]
pub fn cache_projects_before_nav(projects: &[ProjectOption]) {
    let window = leptos::web_sys::window().unwrap();
    if let Some(s) = window.session_storage().ok().flatten() {
        if let Ok(json) = serde_json::to_string(projects) {
            let _ = s.set_item("canonrs_projects_cache", &json);
        }
    }
}

#[cfg(feature = "hydrate")]
pub fn read_projects_cache() -> Vec<ProjectOption> {
    let window = leptos::web_sys::window().unwrap();
    window.session_storage().ok().flatten()
        .and_then(|s| s.get_item("canonrs_projects_cache").ok().flatten())
        .and_then(|json| serde_json::from_str(&json).ok())
        .unwrap_or_default()
}
```

**lib.rs — LocalResource lê cache primeiro:**
```rust
let projects_res = LocalResource::new(move || async move {
    #[cfg(feature = "hydrate")]
    {
        let cached = read_projects_cache();
        if !cached.is_empty() {
            return cached;
        }
    }
    list_project_options().await.unwrap_or_default()
});
```

**Fluxo:**
1. Login → server fn autentica + busca projetos com o token
2. `LoginResult` retorna `projects` para o cliente
3. Cliente grava no `sessionStorage`
4. Hard nav → SSR do dashboard sem select (LocalResource)
5. Hydrate → `LocalResource` lê cache → dados prontos imediatamente
6. `Suspense` resolve → select monta com 10 items
7. Runtime `init_all` inicializa o select via MutationObserver

**Por que funciona sem race condition:**
- O fetch acontece server-side no login — token garantido
- Nenhum fetch no cliente após o hard nav
- Cache é lido síncronamente antes do async do LocalResource

**Comparação com Modo 1:**

| | Modo 1 | Modo 4 |
|---|---|---|
| Fetch após login | Sim (cliente) | Não |
| Latência no dashboard | Visível (Suspense) | Zero (cache) |
| Segundo fetch | Sim | Não |
| Complexidade | Baixa | Média |
