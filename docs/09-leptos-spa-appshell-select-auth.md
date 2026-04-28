# CR-411 — Leptos SPA: AppShell, Select e Auth

## Contexto

Problemas encontrados e resolvidos durante implementacao do seo-agent-frontend com Leptos 0.8, CanonRS e autenticacao via cookie HttpOnly.

---

## 1. Auth em CSR — ServerFn nao propaga cookie

extract::<HeaderMap>() no Leptos nao propaga cookie quando chamado via WASM POST. token_len=0.

Solucao: proxy no main.rs extrai access_token do cookie e injeta Authorization: Bearer.

Browser -> /api/seo/* (proxy Leptos) -> backend

Nunca usar ServerFn para fetch autenticado em CSR. Documentado em CR-409 e CR-410.

---

## 2. Bug contains("selected") no runtime

"unselected".contains("selected") retorna true em Rust.

Consequencia: todo item era detectado como pre-selecionado no init. Runtime chamava
set_selected com o primeiro item — placeholder nunca aparecia.

Fix:
  errado:  s.contains("selected")
  correto: s.split_whitespace().any(|t| t == "selected")

Regra: comparacao de state sempre por token exato, nunca substring.

---

## 3. SelectionState::Selected e obrigatorio no init

Sem SelectionState::Selected no item correto:
- runtime nao detecta item selecionado no init
- set_selected nao e chamado
- rs-change nao e disparado
- label fica em placeholder mesmo com projeto selecionado

O runtime le data-rs-state no mount via split_whitespace — o item deve ter
estado "selected" no HTML inicial para o init funcionar corretamente.

Correto:
  let sel = if p.id == current { SelectionState::Selected } else { SelectionState::Unselected };
  SelectItem value=p.id selected=sel

---

## 4. Signal::derive vs closure simples

Closure simples cria dependencia indireta — Effect pode nao reexecutar quando query muda.

Errado:
  let project_id = move || query.get().get("project_id").unwrap_or_default();

Correto:
  let project_id = Signal::derive(move || query.get().get("project_id").unwrap_or_default());
  Effect::new(move |_| {
      let pid = project_id.get(); // dependencia explicita
      if pid.is_empty() { loading.set(false); return; }
  });

Regra: sempre Signal::derive para derivar estado de use_query_map em Effects.

---

## 5. Padrao bff_get para CSR autenticado

Helper padrao para fetch autenticado no cliente:

  #[cfg(feature = "hydrate")]
  async fn bff_get<T: for<de> serde::Deserialize<de>>(path: &str) -> Result<T, String> {
      use gloo_net::http::Request;
      Request::get(path).send().await.map_err(|e| e.to_string())?
          .json::<T>().await.map_err(|e| e.to_string())
  }

  #[cfg(not(feature = "hydrate"))]
  async fn bff_get<T: for<de> serde::Deserialize<de>>(_path: &str) -> Result<T, String> {
      Err("SSR not supported".into())
  }

Cookie vai automatico (same-origin). Proxy injeta Bearer. Zero config no cliente.

---

## 6. AppShell deve ser unico

Multiplos ParentRoute com AppShell duplicado causam remount a cada navegacao.

Errado:
  ParentRoute path=dashboard view=AppShell
  ParentRoute path=analysis  view=AppShell

Correto:
  ParentRoute path="" view=AppShell (unico)
    Route path=dashboard view=DashboardPage
    Route path=analysis  view=AnalysisPage

AppShell unico — nunca desmonta. Equivalente ao layout.tsx do Next.js.

---

## 7. Select CanonRS — on:rs-change nao funciona via Leptos

O runtime dispara rs-change com bubbles: true no elemento root data-rs-select.
Leptos nao captura esse evento via on:rs-change — o runtime substitui o DOM interno
apos o init e o listener do Leptos fica no elemento errado.

Provado via log:
  [select] dispatching rs-change value=xxx  <- runtime dispara
  [selector] rs-change val=               <- leptos nunca recebe

Solucao: thread_local com listener no document — registrado uma unica vez:

  thread_local! { static REGISTERED: Cell<bool> = Cell::new(false); }
  if !REGISTERED.with(|r| r.get()) {
      REGISTERED.with(|r| r.set(true));
      doc.add_event_listener_with_callback("rs-change", cb);
  }

Funciona porque AppShell nunca remonta — listener registrado uma vez, persiste sempre.

Nunca usar:
  addEventListener sem thread_local (duplica listeners por remount)
  on:rs-change via Leptos (nao captura evento customizado do runtime)
  node_ref listener (perde referencia no remount)

---

## 8. URL como unica fonte de verdade

Context duplica estado com URL e causa bugs de sincronizacao.

Regra:
  current = move || query.get().get("project_id").unwrap_or_default()
  SelectionState::Selected baseado na URL no momento do mount
  navigate com ?project_id= ao selecionar projeto
  localStorage como cache de persistencia entre sessoes (boot only)

---

## Resumo

| Problema | Solucao |
|---|---|
| Cookie nao chega no ServerFn CSR | Proxy BFF extrai cookie |
| contains("selected") bug | split_whitespace().any(t == "selected") |
| Label em placeholder com projeto selecionado | SelectionState::Selected obrigatorio |
| Effect nao reage a query param | Signal::derive obrigatorio |
| AppShell remonta a cada rota | Um unico ParentRoute raiz |
| on:rs-change nao captura evento | thread_local listener no document |
| Context e URL duplicam estado | URL como unica fonte de verdade |
