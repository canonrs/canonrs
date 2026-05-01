# Popover — Padrões DOM-driven

## Princípio

Popover é um overlay **não-modal** — não prende foco, não bloqueia background, não usa `inert`.
O DOM é a fonte de verdade para contexto e estado.

---

## Comportamento correto

- **Tab** — sai do popover normalmente, fecha ao perder foco
- **Escape** — fecha
- **Click fora** — fecha
- **Click no trigger** — toggle (abre/fecha)

## O que NÃO fazer

- `inert` no background — só para modais
- focus trap — popover é non-modal
- `RwSignal` para estado do popover
- fechar diretamente no `focusout` — usar `setTimeout(0)` para aguardar o próximo `focus`

---

## Regra de escopo de eventos

Eventos de fechamento são registrados no `document` e filtrados por `data-rs-uid` do root.
O listener de click usa `stack::register_click` — despacha apenas para o overlay no topo do stack.
O listener de keydown usa `stack::register_keydown` com `capture=true` — intercepta antes do browser.
O `focusout` é registrado diretamente no root — não no document.

```
document (capture) → keydown → stack → uid → root
document (bubble)  → click   → stack → uid → root
root               → focusout → setTimeout(0) → check active_element
```

---

## Múltiplos popovers

O sistema usa `stack` com `KIND="popover"` — apenas um popover por grupo pode estar aberto.
Quando um segundo popover abre, o `stack::push` registra o novo uid como topo.
Escape fecha apenas o topo via `stack::is_top`.
Click fora fecha todos os popovers que não contêm o target.

```
CR — Popover stacking: múltiplos permitidos, Escape fecha o topo
CR — Click fora fecha todos os popovers abertos que não contêm o target
```

---

## Context via DOM

Trigger carrega contexto via `data-rs-*`, runtime propaga para o root no open.

```rust
<PopoverTrigger value=item.id.to_string() label=item.name.clone()>
    "Options"
</PopoverTrigger>
```

Runtime no `open()`:
```
trigger[data-rs-value] → root[data-rs-current-value]
trigger[data-rs-label] → root[data-rs-current-label]
```

---

## Form submission (uso explícito)

Popover não é input por natureza — `name` é opcional e deve ser usado conscientemente.
Quando presente, runtime mantém `<input type="hidden">` sincronizado com `data-rs-current-value`.

```rust
// SÓ usar quando o Popover atua como seletor de valor para um form
<Popover name="selected_option">
    <PopoverTrigger value="42" label="Option A">"Option A"</PopoverTrigger>
    ...
</Popover>
// form recebe: selected_option=42
```

---

## Posicionamento

Runtime usa `positioning::auto_side` após o open:
- Lê `data-rs-side` do content (preferência do SSR)
- Calcula espaço disponível via `getBoundingClientRect()`
- Faz flip automático se não há espaço na direção preferida
- Prioridade: `bottom → top → right → left`

Limitações atuais:
- Não considera scroll container — só viewport
- Não recalcula em resize — só no open
- Overflow horizontal não é tratado

```rust
<PopoverContent side=PopoverSide::Bottom>...</PopoverContent>
// flip automático para Top se não há espaço abaixo
```

---

## focusout com delay

```
ERRADO: on focusout → close()
CORRETO: on focusout → setTimeout(0) → check active_element → se fora → close()
```

---

## Regras formais

- **CR-430** — Overlay Positioning Contract
- **CR-431** — Trigger → Overlay Lifecycle
- **CR-432** — Outside Click + Focus Policy
- **CR-433** — Non-modal overlays MUST NOT use inert
- **CR-434** — focusout MUST use setTimeout(0) before closing
- **CR-435** — Event listeners scoped to document, filtered by uid
