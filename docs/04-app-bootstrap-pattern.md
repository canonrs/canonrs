# CR-406 — App Bootstrap Pattern

## Responsabilidades

Cada arquivo tem uma responsabilidade unica e nao pode assumir a do outro.

**main.rs** — entry point do servidor. Configura Axum, registra rotas, serve assets. Nao conhece dados de dominio.
**lib.rs** — define App, AppShell, rotas e contexto global.
**context.rs** — define o contrato de dados compartilhados entre componentes.

---

## Padrao main.rs

```rust
.leptos_routes(&leptos_options, generate_route_list(App), {
    let opts = leptos_options.clone();
    move || shell(opts.clone())
})
```

Regras:
- main.rs nao conhece ProjectContext, ProjectOption ou qualquer dado de dominio
- Se dados sao necessarios no init de componentes interativos, devem vir do SSR handler — nunca de Resource criado no App
- Nunca usar block_in_place para fetch — bloqueia o runtime do Tokio

---

## Padrao lib.rs — Client Prefetch

O modelo atual e client prefetch: dados resolvidos no cliente antes do mount do componente interativo.

```rust
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let ctx = ProjectContext::new();
    provide_context(ctx);

    // LocalResource — client-only, nao roda no SSR.
    // Dados chegam antes do mount do ProjectSelector via Suspense.
    let projects_res = LocalResource::new(|| async move {
        list_project_options().await.unwrap_or_default()
    });
    let projects_res = StoredValue::new(projects_res);

    view! {
        <Router>
            <Routes fallback=|| view! { "Not found" }>
                <Route path=StaticSegment("login") view=LoginPage/>
                <ParentRoute path=StaticSegment("dashboard") view=move || view! {
                    <AppShell projects_res=projects_res.get_value() />
                }>
                    <Route path=StaticSegment("") view=DashboardPage/>
                </ParentRoute>
            </Routes>
        </Router>
    }
}
```

Regras:
1. App cria o contexto e o LocalResource — nunca dentro do AppShell
2. LocalResource criado uma vez, passado via StoredValue para as rotas
3. Login e rotas publicas ficam fora do AppShell
4. ParentRoute com AppShell para rotas autenticadas

---

## Padrao AppShell

```rust
#[component]
fn AppShell(projects_res: LocalResource<Vec<ProjectOption>>) -> impl IntoView {
    let header_fn: ChildrenFn = Arc::new(move || {
        view! {
            <Suspense fallback=|| view! { <PageHeader ... /> }.into_any()>
            {move || Suspend::new(async move {
                let projects = projects_res.await;
                view! {
                    <PageHeader ... />
                    <ProjectSelector projects=projects />
                }.into_any()
            })}
            </Suspense>
        }.into_any()
    });

    view! {
        <DashboardLayout
            header=header_fn
            sidebar=sidebar_fn
            content=slot!(|| view! { <Outlet/> }.into_any())
        />
    }
}
```

Regras:
1. AppShell nao faz fetch — recebe dados como prop
2. Layout nasce uma vez — nunca dentro de Suspense
3. Suspense envolve apenas o componente interativo, nunca o layout
4. ProjectSelector nasce apos a resolucao do LocalResource no cliente — dados completos antes do mount

---

## Regra de Ouro

```
Layout     = estatico
Dados      = resolvidos antes do mount do componente interativo
Interativo = nasce uma vez com DOM completo
```

---

## SSR vs Client Prefetch

| Modelo | Dados no SSR | Hydration mismatch | Complexidade |
|---|---|---|---|
| Client Prefetch (atual) | Nao | Impossivel | Baixa |
| SSR Snapshot | Sim (via handler) | Possivel se mal implementado | Alta |

O modelo Client Prefetch e seguro e correto desde que:
- Login use hard nav (set_href) — nunca navigate() do router
- Suspense segure o mount do componente interativo ate os dados chegarem
- O runtime seja inicializado via MutationObserver para capturar mounts pos-hydrate
