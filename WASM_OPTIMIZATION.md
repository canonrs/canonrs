# WASM Optimization Guide — CanonRS

## Resultados Atuais

| Ambiente | Tamanho em disco | Transferido (Brotli) |
|----------|-----------------|----------------------|
| `make dev` | ~16MB | ~16MB (sem compressão) |
| `make build` | ~4.1MB | ~1MB |

---

## Configurações Aplicadas

### 1. Profile `wasm-release` — Produção
```toml
[profile.wasm-release]
inherits = "release"
opt-level = "z"       # Otimiza tamanho agressivo
lto = "fat"           # Link Time Optimization completo
codegen-units = 1     # Necessário para LTO funcionar
panic = "abort"       # Remove panic handling — menor binário
strip = "symbols"     # Remove símbolos de debug
```

Ativado via:
```toml
lib-profile-release = "wasm-release"
```

### 2. wasm-opt — Pós-processamento
```toml
wasm-opt = ["-Oz", "--enable-bulk-memory", "--enable-nontrapping-float-to-int"]
```

- `-Oz` → otimização máxima de tamanho
- `--enable-bulk-memory` → operações de memória em batch
- `--enable-nontrapping-float-to-int` → instruções WASM mais eficientes

### 3. Profile `wasm-dev` — Desenvolvimento
```toml
[profile.wasm-dev]
inherits = "dev"
opt-level = "s"       # Reduz dev de 31MB → 16MB sem perder hot-reload
```

Ativado via:
```toml
lib-profile-dev = "wasm-dev"
```

### 4. Compressão Brotli/Gzip — Runtime

Adicionado `CompressionLayer` no servidor Axum:
```rust
use tower_http::compression::CompressionLayer;

let app = Router::new()
    // ...rotas...
    .layer(CompressionLayer::new())
    .with_state(leptos_options);
```

Dependência em `Cargo.toml`:
```toml
tower-http = { version = "0.6", features = ["fs", "compression-br", "compression-gzip"] }
```

O browser recebe o header `Content-Encoding: br` — 4.1MB comprime para ~1MB em transferência.

### 5. syntect — Grammars Sob Demanda

O highlighter SSR usa `OnceLock` para carregar o `SyntaxSet` apenas uma vez:
```rust
static SYNTAX_SET: OnceLock<SyntaxSet> = OnceLock::new();

fn get_syntax_set() -> &'static SyntaxSet {
    SYNTAX_SET.get_or_init(|| SyntaxSet::load_defaults_newlines())
}
```

---

## Workflow
```
Durante o dia   →  make dev    →  16MB dev build, hot-reload
Fim do dia      →  make build  →  4.1MB release, wasm-opt, Brotli
```

---

## O que NÃO fazer

- ❌ Usar debug build em produção (186MB)
- ❌ Usar `load_defaults_newlines()` sem `OnceLock` — recarrega grammars a cada request
- ❌ Servir WASM sem compressão em produção

---

## Próximos Passos (futuro)

- [ ] Code splitting via `cargo leptos build --split`
- [ ] Carregar apenas grammars necessárias no syntect (reduz ~2-3MB)
- [ ] Streaming compilation (browser feature, sem ação necessária)
- [ ] pre-compress `.wasm.br` no build para servir arquivo já comprimido
