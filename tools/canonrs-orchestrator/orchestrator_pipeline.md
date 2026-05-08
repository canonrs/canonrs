# CanonRS Orchestrator — Pipeline

O orchestrator é o processo central de desenvolvimento do CanonRS.
Ele inicializa e mantém vivos todos os subprocessos necessários para o ciclo de desenvolvimento:
tokens CSS, bundle WASM, servidor leptos, watchers de filesystem e servidor WebSocket de reload.
Cada responsabilidade está isolada em um módulo separado.
O `main.rs` não contém lógica — apenas orquestra a sequência correta de inicialização.

---

## `config.rs` — Configuração central

Fonte única de verdade para todos os paths e constantes do sistema.
Nenhum outro módulo deve hardcodar caminhos ou valores de configuração.
A função `root()` resolve o path raiz do monorepo a partir de `CARGO_MANIFEST_DIR`.
A constante `WASM_CRATES` lista os 9 crates cujos `src/` são observados pelo wasm_watcher.
Quando um novo crate de interação for criado, ele deve ser adicionado aqui — e apenas aqui.
As constantes `WASM_DEBOUNCE_MS` e `CORE_DEBOUNCE_MS` controlam o tempo mínimo entre rebuilds
para evitar builds excessivos durante edições rápidas.
`WS_PORT` define a porta do servidor WebSocket (padrão: 9099).

---

## `state.rs` — Estado global do pipeline

`SystemState` é a struct compartilhada entre threads via `Arc<Mutex<SystemState>>`.
Contém três campos: `tokens`, `wasm` e `leptos`, cada um representando o status atual
do respectivo subprocesso (ex: `"OK (312ms) hash=a3f2"`, `"building..."`, `"FAILED"`).
O método `print()` imprime o estado completo no terminal em formato tabular,
permitindo visualizar de forma rápida o que está funcionando e o que falhou.
É chamado automaticamente após cada build WASM e na inicialização do leptos.

---

## `wasm.rs` — Build WASM

Responsável por tudo relacionado ao bundle WASM do CanonRS.
`ensure_wasm_hash` garante que o diretório `assets/js/` existe antes do leptos iniciar,
evitando erros de path ao servir assets estáticos.
`wasm_hash` calcula um hash FNV-1a do arquivo `.wasm` gerado para cache busting no browser.
`inject_hash_in_html` escreve `wasm_hash.js` com `window.__CANON_WASM_HASH__`,
que o loader usa para carregar o bundle correto sem cache stale.
`build_wasm` executa `wasm-pack build` apontando para `canonrs-interactions`,
copia os artefatos gerados para `assets/wasm/`, injeta o hash e notifica o servidor WS.
Se a variável `CANON_RELEASE` estiver definida, o build usa `--release`; caso contrário, `--dev`.

---

## `pipeline.rs` — Sequência de inicialização

Contém as funções que rodam uma única vez durante o boot do orchestrator.
`spawn_tokens` executa `cargo run --bin tokens-engine` no crate `canonrs-tokens`,
gerando os arquivos CSS de design tokens antes do leptos iniciar.
`copy_loaders` copia `canon-loader.js` e `canonrs.bundle.js` de `src/loader/`
para `assets/js/`, substituindo o placeholder `__CANONRS_VERSION__` pela versão atual do crate.
`build_css` executa `npm run build:css` no diretório `canonrs-site`.
`spawn_leptos` inicia `cargo leptos watch --project <project>` e suporta a variável
`CANON_FEATURES` para ativar features Rust opcionais sem modificar código:
```bash
CANON_FEATURES=webgl make dev
```
Isso adiciona `--lib-features <valor>` ao comando do leptos.

---

## `watchers.rs` — File watchers

Três watchers rodando em threads separadas usando a crate `notify`.

`spawn_wasm_watcher` observa o diretório `src/` de todos os crates listados em `WASM_CRATES`.
Quando qualquer arquivo `.rs` é modificado, aguarda o debounce de 500ms e chama `build_wasm`.
Após build bem-sucedido, o browser é notificado via WebSocket.
Este é o watcher mais crítico do sistema — qualquer alteração em qualquer crate de interação
dispara automaticamente um novo bundle WASM sem intervenção manual.

`spawn_core_watcher` observa `blocks/`, `layouts/`, `ui/` do canonrs-server e o diretório `build/`
do canonrs-core. Quando arquivos `.rs` ou `.yaml` mudam, ele reescreve `build.rs` do canonrs-core
para atualizar seu mtime, forçando o leptos a recompilar o servidor sem reiniciar o processo.
Usa debounce de 1000ms por ser uma operação mais custosa.

`spawn_loader_watcher` observa `canonrs-client/src/loader/` em modo não-recursivo.
Quando `canon-loader.js` ou `canonrs.bundle.js` é alterado, recopia imediatamente para
`assets/js/` com substituição da versão, sem necessidade de reiniciar o orchestrator.

---

## `ws.rs` — WebSocket reload server

Servidor WebSocket assíncrono na porta `WS_PORT` (9099) que notifica browsers conectados
sempre que o bundle WASM é rebuiltado com sucesso.
Aceita múltiplos clientes simultâneos, cada um em uma tokio task separada.
Usa `tokio::select!` para escutar mensagens do cliente e eventos de reload ao mesmo tempo.
Quando recebe sinal do `reload_tx`, envia a mensagem `"reload"` para todos os clientes conectados.
Fecha a conexão corretamente em caso de mensagem `Close` ou erro de leitura.
O browser deve conectar em `ws://localhost:9099` e executar `location.reload()` ao receber `"reload"`.

---

## Fluxo completo

```
main()
 ├── spawn_tokens         → canonrs-tokens/tokens-engine
 ├── copy_loaders         → assets/js/
 ├── ensure_wasm_hash     → assets/js/ (mkdir)
 ├── build_wasm           → wasm-pack → assets/wasm/ + wasm_hash.js
 ├── spawn_wasm_watcher   → thread: observa src/ de 9 crates
 ├── spawn_loader_watcher → thread: observa loader/
 ├── spawn_core_watcher   → thread: observa blocks/layouts/ui
 ├── ws_reload_server     → tokio task: porta 9099
 ├── build_css            → npm run build:css
 ├── spawn_leptos         → cargo leptos watch
 └── loop                 → tokio::time::sleep (mantém runtime vivo)
```

---

## Adicionando um novo crate de interação

1. Criar o crate em `packages-rust/rs-canonrs/canonrs-interactions-<nome>/`
2. Adicionar o nome em `config.rs` → `WASM_CRATES`
3. Adicionar como dependência em `canonrs-interactions/Cargo.toml`
4. Registrar o handler em `canonrs-interactions/src/runtime/dispatcher.rs`

O wasm_watcher passa a observar o novo crate automaticamente na próxima inicialização.

---

_CanonRS Orchestrator — CanonRS Team_
