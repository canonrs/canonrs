# canonrs-core

Núcleo do sistema CanonRS. Define contratos, primitivos e infraestrutura compartilhada entre server e client.

---

## Estrutura
```
canonrs-core/
├── build/               # Módulos do build.rs (geração de código)
│   ├── types.rs         # Structs compartilhadas entre módulos do build
│   ├── utils.rs         # Helpers puros (pascal_to_kebab, to_const_name...)
│   ├── parsers.rs       # Parsers de primitivos, components.toml, blocks/layouts
│   └── generators.rs    # Geradores: schema.json, meta, catalog, definitions, api, llm
├── src/
│   ├── infra/           # Infraestrutura transversal de runtime
│   │   ├── theme/       # ThemeProvider, CanonRSRoot, use_theme, ThemeMode
│   │   ├── state_engine.rs  # Centraliza aria-*, data-rs-state por tipo de estado
│   │   └── dom_contract.rs  # Valida composição de componentes em dev/test
│   ├── primitives/      # Contratos puros de componentes (98 primitivos)
│   ├── generated/       # Código auto-gerado pelo build.rs (não editar)
│   │   ├── component_meta.rs
│   │   ├── block_meta.rs
│   │   ├── block_definitions.rs
│   │   ├── layout_definitions.rs
│   │   ├── catalog.rs
│   │   ├── llm_components.md  # Contexto LLM — componentes UI
│   │   ├── llm_blocks.md      # Contexto LLM — blocks com props e presets
│   │   └── llm_layouts.md     # Contexto LLM — layouts com slots e regiões
│   ├── meta.rs          # Enums de estado: VisibilityState, ActivityState...
│   ├── meta_types.rs    # ComponentMeta, Capability, ComponentFamily
│   ├── catalog_types.rs # CatalogEntry, CatalogCategory
│   ├── block_types.rs   # BlockDefinition, BlockVariant, LayoutDefinition...
│   ├── prelude.rs       # Re-exports públicos controlados
│   └── lib.rs           # Entry point da crate
├── components.toml      # Registro semântico dos componentes (SSOT)
├── build.rs             # Entry point do build — orquestra os módulos de build/
└── schema.json          # Gerado automaticamente — não commitar alterações manuais
```

---

## Como funciona o build

O `build.rs` roda antes da compilação e gera código Rust em `src/generated/`.

**Pipeline:**
1. **Parse primitivos** — lê `src/primitives/*.rs`, extrai `data-rs-component`, `data-rs-behavior` e enums de variantes
2. **Parse semântico** — lê `components.toml`, carrega metadados (família, capabilities, tags de catálogo)
3. **Parse blocks/layouts** — lê `canonrs-server/src/blocks/` e `layouts/`, extrai headers `@canon-*`
4. **Geração Rust** — produz `component_meta.rs`, `block_meta.rs`, `block_definitions.rs`, `layout_definitions.rs`, `catalog.rs` e `schema.json`
5. **Geração API** — produz `api.rs` por block e layout (contrato tipado de props)
6. **Geração LLM** — produz `llm_components.md`, `llm_blocks.md`, `llm_layouts.md` para consumo pelo pipeline de IA

Qualquer alteração em `src/primitives/`, `components.toml` ou nos blocks/layouts dispara o rebuild automaticamente.

---

## Camadas

| Camada | Responsabilidade |
|---|---|
| `infra/theme` | Runtime de tema (dark/light, contexto Leptos) |
| `infra/state_engine` | Mapeia estados de UI para atributos aria/data |
| `infra/dom_contract` | Valida composição correta de componentes |
| `primitives/` | Contrato puro de cada componente — sem lógica de negócio |
| `generated/` | Metadados, catálogo, API contracts e contexto LLM gerados automaticamente |

---

## Dependências externas

- `canonrs-style` — tipos declarativos de estilo (`StyleProps`, `Spacing`, etc.)
- `canonrs-tokens` — tokens de design (cores, espaçamentos) — usado pelo build de CSS
- `leptos` — framework reativo (apenas `infra/theme` depende diretamente)