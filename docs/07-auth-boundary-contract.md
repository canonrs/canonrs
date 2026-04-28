# CR-409 — Auth Boundary Contract

## Principio

Autenticacao nao e funcao do framework. E funcao da arquitetura.

```
Browser → /api/* (mesmo dominio) → Proxy (Leptos/Axum) → Backend
```

---

## 1. Auth Boundary no Servidor

Cliente nunca fala com backend autenticado diretamente.

```
Browser → /api/* → Proxy → Backend
```

O proxy:
- Le Cookie HttpOnly
- Injeta Authorization: Bearer
- Normaliza headers
- Retorna JSON

Um unico ponto de auth. Zero logica no cliente.

---

## 2. Cookie HttpOnly = Unica Fonte de Auth

```
Cookie HttpOnly = unica fonte de autenticacao
```

- Cliente NAO le token
- Cliente NAO monta header Authorization
- Cliente NAO usa localStorage para auth

Zero Trust real.

---

## 3. Same-Origin API Contract

Frontend so chama /api/*

Nunca:
```
localhost:3010        ❌
dominio externo direto ❌
```

Tudo passa pelo proxy.

---

## 4. Proxy como Adaptador

O proxy faz:
```
Cookie → Bearer
Headers → Sanitizacao
Path → Normalizacao
```

Implementacao padrao (Axum):

```rust
async fn proxy_handler(
    State((client, base)): State<(reqwest::Client, String)>,
    OriginalUri(uri): OriginalUri,
    method: Method,
    headers: HeaderMap,
    body: Bytes,
) -> impl IntoResponse {
    let url = format!("{}{}", base, uri.path_and_query().map(|p| p.as_str()).unwrap_or(""));
    let mut req = client.request(method, &url);

    // Cookie → Bearer
    if let Some(cookie) = headers.get(COOKIE) {
        req = req.header(COOKIE, cookie);
        if let Ok(s) = cookie.to_str() {
            if let Some(token) = s.split(';')
                .find(|p| p.trim().starts_with("access_token="))
                .map(|p| p.trim().trim_start_matches("access_token=").to_string())
            {
                req = req.header(AUTHORIZATION, format!("Bearer {}", token));
            }
        }
    }

    match req.body(body).send().await {
        Ok(resp) => {
            let status = StatusCode::from_u16(resp.status().as_u16()).unwrap_or(INTERNAL_SERVER_ERROR);
            let bytes = resp.bytes().await.unwrap_or_default();
            (status, [(CONTENT_TYPE, "application/json")], bytes).into_response()
        }
        Err(e) => (BAD_GATEWAY, e.to_string()).into_response(),
    }
}
```

Registro no router:

```rust
Router::new()
    .route("/api/seo/{*path}", any(proxy_handler).with_state(proxy_state.clone()))
    .route("/api/scraper/{*path}", any(proxy_handler).with_state(proxy_state))
```

---

## 5. Zero Hidden Transport

- Sem confiar em extract automatico
- Sem confiar em hydration
- Sem confiar em auto-propagacao do framework

Tudo explicito.

---

## Resumo

```
Auth     → sempre no servidor (proxy)
Dados    → sempre via /api/* (same-origin)
Cliente  → nunca conhece token
Cookie   → HttpOnly, unica fonte de verdade
```
