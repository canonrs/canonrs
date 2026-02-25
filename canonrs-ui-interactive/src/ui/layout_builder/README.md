# CanonRS — Layout Builder Platform

Sistema completo de edição, versionamento, publicação e execução de layouts UI.

---

## Fase 1 — Fundação (AST Canônica)

Tipos canônicos que representam qualquer layout como árvore recursiva.

**Arquivos:**
- `canonrs-ui-interactive/src/ui/layout_builder/canon_ast.rs` — `CanonBlockType`, `CanonNode`, `CanonDocument`, `build_tree()`, `flatten_tree()`
- `canonrs-ui-interactive/src/ui/layout_builder/types.rs` — `Node`, `NodeKind`, `BlockDef`, `ActiveLayout`

---

## Fase 2 — Builder Interativo

Drag-and-drop com slots por layout, sidebar de blocos e canvas.

**Arquivos:**
- `canonrs-ui-interactive/src/ui/layout_builder/layout_builder_interactive.rs` — componente raiz, signals principais
- `canonrs-ui-interactive/src/ui/layout_builder/layout_canvas.rs` — canvas de renderização
- `canonrs-ui-interactive/src/ui/layout_builder/drop_zone.rs` — `insert_node()`, `move_node()`
- `canonrs-ui-interactive/src/ui/layout_builder/sidebar_blocks.rs` — paleta de blocos disponíveis

---

## Fase 3 — Inspector de Propriedades

Inspeção e edição de props do nó selecionado em tempo real.

**Arquivos:**
- `canonrs-ui-interactive/src/ui/layout_builder/inspector_panel.rs` — painel lateral
- `canonrs-ui-interactive/src/ui/layout_builder/types.rs` — `CanonProps`, `selected_node_id`

---

## Fase 4 — Renderer Canônico

Converte `CanonDocument` em view Leptos renderizável.

**Arquivos:**
- `canonrs-ui-interactive/src/ui/layout_builder/renderer.rs` — `render_document()`, mapeamento `CanonBlockType` → componente UI
- `products/canonrs-site/src/pages/preview.rs` — página de preview com painel de debug

---

## Fase 5 — Live Preview Determinístico

Protocolo READY/DOC/ACK via `postMessage` entre Builder e iframe. Zero loops reativos.

**Arquivos:**
- `canonrs-ui-interactive/src/ui/layout_builder/preview_protocol.rs` — máquina de estados `PreviewState`, `send_to_iframe()`, `retry_if_needed()`, `log_protocol()`
- `products/canonrs-site/src/pages/preview.rs` — receptor do protocolo, painel debug, validação de versão

---

## Fase 6 — Persistência

Serviço backend com SQLite, save/load, snapshots automáticos, hash SHA256.

**Arquivos:**
- `core-services/rs-canonrs-builder/src/documents/handlers.rs` — `save_document()`, `list_documents()`, `get_document()`, `delete_document()`
- `core-services/rs-canonrs-builder/src/documents/types.rs` — `SaveDocumentRequest`, `DocumentResponse`, `CanonDocumentRow`
- `core-services/rs-canonrs-builder/migrations/001_initial.sql` — tabelas `canon_documents`, `canon_snapshots`
- `canonrs-ui-interactive/src/ui/layout_builder/canvas_toolbar.rs` — toolbar com Save/Open, fetch para porta 8112

---

## Fase 7 — Codegen + Export

Gera código Leptos válido a partir do `CanonDocument`. Download direto do `.rs`.

**Arquivos:**
- `canonrs-ui-interactive/src/ui/layout_builder/codegen.rs` — `export_document_to_rs()`, mapeamento dos 30 `CanonBlockType`
- `canonrs-ui-interactive/src/ui/layout_builder/canvas_toolbar.rs` — botão Export, download via Blob API

---

## Fase 8 — Governança Transacional

Versão otimista, Undo/Redo estrutural, dirty state, schema_migrations enterprise.

**Arquivos:**
- `canonrs-ui-interactive/src/ui/layout_builder/history.rs` — `HistoryStack`, `suppress_next`, `init_history()`
- `core-services/rs-canonrs-builder/src/db.rs` — `schema_migrations`, migrations idempotentes
- `canvas_toolbar.rs` — `SaveError::Conflict`, botões Undo/Redo, indicador `● unsaved`

---

## Fase 9 — Publicação + Runtime Isolado

Pipeline completo: draft → publish → runtime → cache. Serviço de execução separado do builder.

**Arquivos:**
- `core-services/rs-canonrs-builder/migrations/002_published.sql` — tabelas `canon_published`, colunas `status/published_version/published_at`
- `core-services/rs-canonrs-builder/src/documents/handlers.rs` — `publish_document()`, `get_published()`
- `core-services/rs-canonrs-runtime/src/render.rs` — `render_document()`, `invalidate_cache()`, validação hash, cache HIT/MISS
- `core-services/rs-canonrs-runtime/src/main.rs` — porta 8113, rotas `/app/:id`, `/cache/:id`
- `canvas_toolbar.rs` — botões Publish, View, invalidação automática de cache

---

## Serviços

| Serviço | Porta | Responsabilidade |
|---|---|---|
| rs-canonrs-builder | 8112 | Edição, persistência, publicação |
| rs-canonrs-runtime | 8113 | Execução isolada, cache HTML |
| canonrs-site | 3000 | Frontend WASM do builder |

---

## Fluxo Principal
```
Edita no builder → Save (draft) → Publish (imutável) → Runtime serve HTML cacheado
```
