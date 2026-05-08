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
  state       → mutação de estado (data-rs-state) + State enum canônico
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

## `State` enum — tokens canônicos

Todos os tokens válidos de `data-rs-state` estão tipados no enum `State`.
Use sempre o enum. Strings literais são permitidas apenas para tokens dinâmicos ou específicos de componente.

```rust
use canonrs_interactions_core::dom::state::{State, set, unset, is};

state::set(el, State::Active);      // adiciona "active"
state::unset(el, State::Inactive);  // remove "inactive"
state::is(el, State::Disabled);     // verifica "disabled"
```

Tokens disponíveis:

```
Visibility:  Open, Closed, Hidden, Visible
Activity:    Active, Inactive
Selection:   Selected, Unselected
Expansion:   Expanded, Collapsed
Interaction: Focused, Hover, Disabled
Form/Toggle: Checked, Unchecked, On, Off
Async:       Loading, Idle, Error, Submitting
Feedback:    Copied, Paused
Transition:  Entering, Exiting
```

---

## Regras (OBRIGATÓRIO)

### Estado

```rust
state::set(el, State::Active);  // CORRETO — tipado
state::add(el, "active");       // permitido para tokens dinâmicos
```

- PROIBIDO manipular `class`, `style` ou `hidden` sem sincronizar `data-rs-state`

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
query::safe_target(e);       // OBRIGATÓRIO — valida is_connected
query::has_ancestor_attr(el, attr); // sobe árvore procurando atributo
```

- PROIBIDO usar `e.target()` direto

### Keyboard

```rust
init_nav(&root, selector, NavConfig { .. }, None, None);
```

- OBRIGATÓRIO para Arrow/Home/End em listas
- PROIBIDO reimplementar keyboard navigation manualmente

### Lifecycle

```rust
if !lifecycle::init_guard(&root) { return; }
```

- OBRIGATÓRIO em TODOS os engines
- Suporta `data-rs-reinit` para rebind após hydration

---

## Proibições

- Reimplementar keyboard navigation (Arrow/Home/End)
- Reimplementar seleção ativa
- Manipular DOM fora do core
- Criar múltiplos listeners por instância quando existe global
- Operar em elementos sem validar `is_connected()`
- Usar strings literais onde existe variante no enum `State`

---

## Responsabilidades

**Core**
- Garantir segurança (`is_connected`)
- Garantir consistência de estado
- Garantir acessibilidade base
- Tipar todos os tokens de estado

**Engines** (`nav`, `overlay`, `selection`, `gesture`, `content`)
- Orquestrar comportamento específico do componente
- Nunca implementar lógica base já coberta pelo core
- Declarar no cabeçalho quais módulos do core usa

---

## Exemplo

CORRETO:
```rust
// cabeçalho do engine
//! Tabs Interaction Engine
//! Core: dom/{lifecycle, state, query} + behavior/selection::activate_by_value

selection::activate_by_value(&root, &value, &config);
state::set(&trigger, State::Active);
```

PROIBIDO:
```rust
for el in items {
    state::remove(&el, "active");
}
state::add(&item, "active");
```

---

## Testes

```bash
cargo test -p canonrs-interactions-core
```

Testes de lógica pura rodam com `cargo test`.
Testes de DOM rodam via `wasm-pack test --headless`.

Módulos com testes: `dom/state`, `behavior/selection`, `behavior/disclosure`.

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
2. Não tem dependência de módulos overlay-específicos
3. É stateless ou usa apenas `thread_local` com registry global bem definido
4. Tem invariantes documentados neste README
5. Tem testes unitários

---

## Quem usa

| Crate | Módulos consumidos |
|---|---|
| `canonrs-interactions-nav` | `dom/*`, `behavior/keyboard`, `behavior/selection`, `behavior/disclosure`, `integration/aria` |
| `canonrs-interactions-overlay` | `dom/*`, `integration/aria`, `integration/form`, `behavior/events` |
| `canonrs-interactions-selection` | `dom/*`, `integration/aria`, `integration/form` |
| `canonrs-interactions-init` | `dom/*`, `integration/aria` |
| `canonrs-interactions-gesture` | `dom/*` |
| `canonrs-interactions-content` | `dom/state`, `dom/lifecycle` (via shared re-export) |

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
