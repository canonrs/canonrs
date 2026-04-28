# CR-410 — CSR vs SSR Fetch Contract

## Principio

SSR e CSR tem estrategias de fetch diferentes e incompativeis.
Mistura-las causa bugs invisiveis de auth e hydration.

---

## Tabela de Decisao

| Contexto | Estrategia | Motivo |
|---|---|---|
| SSR inicial | ServerFn + extract | cookie disponivel no request HTTP |
| CSR autenticado | gloo_net + proxy | cookie vai automatico same-origin |
| Mutacao simples | ServerFn | idempotente, sem leitura autenticada |
| Endpoint publico | ServerFn ou gloo_net | sem restricao |

---

## 1. CSR Data Fetch = gloo_net

Dados autenticados no cliente usam fetch same-origin via proxy.

```rust
use gloo_net::http::Request;

let res = Request::get("/api/seo/analyses/{id}")
    .send()
    .await?;
let data = res.json::<Analysis>().await?;
```

- Cookie vai automatico (same-origin)
- Sem CORS
- Sem depender de extract no ServerFn
- Deterministico e previsivel

Padrao com LocalResource:

```rust
let res = LocalResource::new(move || async move {
    bff_get::<Analysis>(&format!("/api/seo/analyses/{}", id())).await
});
```

---

## 2. ServerFn = SSR ou Acoes Simples

Usar ServerFn apenas para:
- SSR inicial (dados que precisam estar no HTML)
- Acoes idempotentes simples (forms, mutations leves)
- Endpoints publicos

NAO usar ServerFn para:
- Leitura autenticada em CSR
- Dados que dependem de cookie em WASM POST

Motivo: extract::<HeaderMap>() nao propaga cookie em chamadas WASM POST para /api/*.

---

## 3. No Dual Networking Model

Nunca usar ServerFn e gloo_net juntos para o mesmo dominio de dados.

```
analysis_detail → gloo_net ✔
analysis_detail → ServerFn ❌
```

Causa: inconsistencia, race condition, bugs invisiveis.

---

## 4. Determinismo de Dados

Cada dado tem um unico caminho de obtencao.

```
Analysis → sempre via GET /api/seo/analyses/{id}
Projects → sempre via GET /api/seo/projects
```

Evita race condition e bugs invisiveis.

---

## 5. Separacao Rigida SSR vs CSR

| contexto | estrategia |
|---|---|
| SSR | ServerFn / extract |
| CSR | fetch + proxy |

Nunca misturar.

---

## Helper padrao (bff_get)

```rust
#[cfg(feature = "hydrate")]
async fn bff_get<T: for<'de> serde::Deserialize<'de>>(path: &str) -> Result<T, String> {
    use gloo_net::http::Request;
    let resp = Request::get(path)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    resp.json::<T>().await.map_err(|e| e.to_string())
}

#[cfg(not(feature = "hydrate"))]
async fn bff_get<T: for<'de> serde::Deserialize<'de>>(_path: &str) -> Result<T, String> {
    Err("SSR not supported".into())
}
```

---

## Resumo

```
CSR autenticado  → gloo_net → proxy → backend
SSR inicial      → ServerFn → extract → backend
Acoes simples    → ServerFn
Nunca misturar   → um modelo por dominio de dados
```
