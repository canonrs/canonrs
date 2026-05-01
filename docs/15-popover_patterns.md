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

## Form submission nativa

Prop `name` no root — runtime mantém `<input type="hidden">` sincronizado com `data-rs-current-value`.

```rust
<Popover name="selected_option">
    <PopoverTrigger value="42" label="Option A">"Option A"</PopoverTrigger>
    ...
</Popover>
// form recebe: selected_option=42
```

---

## focusout com delay

O browser move o foco de forma assíncrona — `setTimeout(0)` aguarda o próximo `focus`
antes de verificar se o foco saiu do popover.

```
ERRADO: on focusout → close()
CORRETO: on focusout → setTimeout(0) → check active_element → se fora → close()
```

---

## Posicionamento

Runtime usa `positioning::auto_side` — respeita `data-rs-side` do SSR e faz flip automático.

---

## Regras formais

- **CR-430** — Overlay Positioning Contract
- **CR-431** — Trigger → Overlay Lifecycle
- **CR-432** — Outside Click + Focus Policy
