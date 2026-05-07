# canonrs-interactions-core

Kernel oficial de interação do CanonRS.

Este crate define os **contratos de comportamento** para todos os componentes interativos.
Não contém lógica de produto. Contém apenas regras e mecanismos universais.

---

## Princípios

- DOM é a única source of truth
- Estado é representado exclusivamente via `data-rs-state`
- Nenhum engine pode mutar DOM fora do core
- Nenhum componente pode implementar comportamento manual duplicado

---

## Estrutura

```
dom/
  state       → mutação de estado (data-rs-state)
  query       → seleção segura (is_connected obrigatório)
  attrs       → leitura de atributos
  lifecycle   → init guard + reinit

behavior/
  keyboard    → navegação por teclado (roving focus)
  outside     → click fora (listener global)
  events      → factories de eventos
  selection   → seleção ativa (tabs, nav, etc)
  disclosure  → open/close (accordion)

integration/
  aria        → sincronização ARIA
  form        → integração com forms nativos
```

---

## Regras (OBRIGATÓRIO)

### Estado

```rust
state::add(el, "active");
```

- PERMITIDO
- PROIBIDO manipular `class`, `style` ou `hidden` diretamente sem sincronizar estado

### Seleção

```rust
selection::activate(root, item, config);
```

- OBRIGATÓRIO para qualquer padrão de item ativo
- PROIBIDO reimplementar loop manual de ativação

### Disclosure

```rust
disclosure::toggle(root, item, config);
```

- OBRIGATÓRIO para open/close
- PROIBIDO chamar `state::open/close` diretamente em componentes compostos

### Query

```rust
query::safe_target(e);
```

- OBRIGATÓRIO
- PROIBIDO usar `e.target()` direto

### Lifecycle

```rust
lifecycle::init_guard(root);
```

- OBRIGATÓRIO em TODOS os engines
- PROIBIDO inicializar sem guard

---

## Proibições

- Reimplementar keyboard navigation (Arrow/Home/End)
- Reimplementar seleção ativa
- Manipular DOM fora do core
- Criar múltiplos listeners por instância quando existe global
- Operar em elementos sem validar `is_connected()`

---

## Responsabilidades

**Core**
- Garantir segurança (`is_connected`)
- Garantir consistência de estado
- Garantir acessibilidade base

**Engines** (`nav`, `overlay`, `selection`, `gesture`, `content`)
- Orquestrar comportamento específico do componente
- Nunca implementar lógica base já coberta pelo core

---

## Exemplo

CORRETO:
```rust
selection::activate(&root, &item, &config);
```

PROIBIDO:
```rust
for el in items {
    state::remove(&el, "active");
}
state::add(&item, "active");
```

---

## O que NÃO pertence ao core

| Módulo | Onde fica |
|---|---|
| `stack` (z-index, overlay registry) | `canonrs-interactions-overlay/runtime/` |
| `portal` (move to body, owner) | `canonrs-interactions-overlay/runtime/` |
| `inert` (background isolation) | `canonrs-interactions-overlay/runtime/` |
| `focus` (trap, restore) | `canonrs-interactions-overlay/runtime/` |
| `transition` (entering/exiting/closed) | `canonrs-interactions-overlay/runtime/` |
| `positioning` (auto flip) | `canonrs-interactions-overlay/runtime/` |
| Lógica específica de componente | engine do grupo correspondente |

Regra: se apenas um grupo usa → fica no runtime local desse grupo.

---

## Adicionando ao core

Um módulo entra no core se e somente se:

1. É consumido por 2 ou mais grupos de interação distintos
2. Não tem dependência de módulos overlay-específicos (`stack`, `portal`, `inert`)
3. É stateless ou usa apenas `thread_local` com registry global bem definido
4. Tem invariantes documentados neste README

---

## Quem usa

| Crate | Módulos consumidos |
|---|---|
| `canonrs-interactions-nav` | `dom/*`, `behavior/keyboard`, `behavior/selection`, `behavior/disclosure`, `integration/aria` |
| `canonrs-interactions-overlay` | `dom/*`, `integration/aria`, `integration/form` |
| `canonrs-interactions-selection` | `dom/*`, `behavior/selection`, `integration/aria`, `integration/form` |
| `canonrs-interactions-init` | `dom/*` |
| `canonrs-interactions-gesture` | `dom/*` |
| `canonrs-interactions-content` | `dom/*` |

---

## Dependências

```toml
wasm-bindgen = "0.2"
js-sys = "0.3"
web-sys = { version = "0.3", features = [...] }
```

Zero dependências de outros crates CanonRS.
Zero dependências de Leptos.
Compilável standalone como `rlib` ou `cdylib`.

---

_canonrs-interactions-core — CanonRS Team_
