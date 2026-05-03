# Pipeline Orchestrator

spawn_tokens — gera CSS de design tokens
copy_loaders — copia canon-loader.js e canonrs.bundle.js para assets/js/
ensure_wasm_hash — garante que assets/js/ existe
build_wasm — builda canonrs-interactions via wasm-pack, copia para assets/wasm/
spawn_wasm_watcher — thread que observa src/ dos 8 crates, rebuilda WASM quando .rs muda
spawn_core_watcher — thread que observa blocks/layouts/ui, toca build.rs quando muda
WS reload server — tokio spawn na porta 9099, notifica browser quando WASM rebuilda
build_css — roda npm run build:css
spawn_leptos — inicia cargo leptos watch
Loop principal — mantém tokio runtime vivo



# Pipeline completo do canonrs-interactions:

1. Entry point (lib.rs):

init_all() → runtime::scan_and_init()
init_subtree(el) → runtime::init_element(&el)
2. scan_and_init (runtime/mod.rs):

scanner::query("[data-rs-interaction]") — encontra todos os elementos
Para cada um: init_element(&el)
observer::observe() — inicia MutationObserver
3. init_element:

Se tem data-rs-interaction → registry::should_init → dispatcher::dispatch
Senão → scan_children
4. dispatcher::dispatch:

Lê data-rs-interaction do elemento
Roteia para o handler correto:
"init" → canonrs_interactions_init::init_init
"overlay" → canonrs_interactions_overlay::init_overlay
"data" → canonrs_interactions_data::init_data
etc.
5. init_overlay (canonrs-interactions-overlay/src/lib.rs):

Roteia por atributo: data-rs-popover → popover::init, etc.
Conclusão: quando o loader chama init_all(), ele escaneia o DOM, encontra [data-rs-interaction="overlay"], chama init_overlay que chama popover::init. O código do overlay está no bundle. O loader não usa paths separados por grupo — usa o bundle único.



## `CANON_FEATURES` — Features extras do lib

O orchestrator suporta passar flags de features Rust adicionais para o `cargo leptos watch` via variável de ambiente `CANON_FEATURES`.

**Uso:**
```bash
CANON_FEATURES=webgl make dev
# ou
make dev-webgl
```

**Como funciona:**

Quando `CANON_FEATURES` está definida, o `spawn_leptos` adiciona `--lib-features <valor>` ao comando `cargo leptos watch`. Isso permite ativar features opcionais de compilação no crate lib do frontend sem modificar o código do orchestrator.

**Valores suportados atualmente:**

| Valor | Efeito |
|-------|--------|
| `webgl` | Ativa o renderer WebGL2 via `rs-canvas-renderer-webgl` no lugar do backend Canvas2D padrão |

**Adicionando novas features:**

Qualquer feature declarada no `Cargo.toml` do frontend na seção `[features]` pode ser ativada desta forma. Múltiplas features podem ser passadas como string separada por vírgulas: `CANON_FEATURES=webgl,feature2`.

