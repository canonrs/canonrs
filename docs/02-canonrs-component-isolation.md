# CR-404 — Isolamento de Componentes Interativos

## Regra fundamental

Se o componente precisa sobreviver ao re-render, ele NAO pode estar dentro do componente que re-renderiza.

---

## Componentes que quebram com re-render

**Overlay group** — dialog, confirm_dialog, popover, tooltip, dropdown_menu, hover_card, sheet, drawer, modal, alert_dialog, context_menu. Usam portal, stack global e UID. Sensiveis a qualquer re-render.

**Input/Form group** — input, textarea, select, combobox, input_group, input_otp, form. Conflito DOM vs signal, reset inconsistente, hydration quebra listener.

**Nav/Selection** — tabs, accordion, toggle_group, radio_group, menu, navigation_menu. Estado multiplo via data-rs-state, selecao conflita com re-render.

**Data/Dynamic** — data_table, virtual_list, carousel, scroll_area. DOM dinamico, re-render frequente, listeners perdem binding.

**Gesture/Pointer** — resizable, slider. Pointer capture e document listeners com refs stale.

---

## Padrao de falha (sempre o mesmo)

1. componente re-renderiza
2. DOM muda
3. runtime aponta pro elemento antigo
4. quebra

---

## 6 Regras operacionais

**Regra 1 — Isolar roots interativos**
Dialog, Popover e similares devem ser componentes separados, nunca dentro de lista dinamica ou componente com .update().

**Regra 2 — DOM controla, runtime muta, page le**
Proibido: input controlado via signal, open via RwSignal, value via data-rs-value.
Correto: DOM e source of truth, runtime muta via evento, page le via query_selector.

**Regra 3 — Nunca capturar Element em closure**
Proibido: let root_cb = root.clone() — quebra apos re-render.
Correto: sempre buscar pelo uid no momento da execucao via doc.query_selector.

**Regra 4 — UID e o contrato**
data-rs-uid e a unica identidade confivel do runtime. name, id e classes sao atributos semanticos, nao identidade de runtime.

**Regra 5 — Nao depender de re-render**
Qualquer comportamento que quebra apos signal.update() esta errado por design.

**Regra 6 — Evento sobre signal**
overlay: fecha via evento rs:dialog:close.
input: le via DOM query_selector.
action: dispara via trigger explicito RwSignal.

---

## Checklist antes de usar qualquer componente interativo

- esta dentro de um For?
- esta dentro de componente com .update()?
- usa prop:value / open / selected?
- usa closure reativa?
- usa Element capturado em closure?

Se qualquer resposta for sim: extrair para componente isolado no nivel do Page.

---

## Onde vive o estado

ProjectsPage e o nivel correto para signals, actions e hooks.
DialogNewItem e ConfirmDelete sao componentes isolados sem estado proprio.
ItemList e reativo mas nao contem overlays.

Estado que muda (lista de itens, mensagens) fica no Page e e passado como prop para filhos que renderizam — nunca para overlays.
