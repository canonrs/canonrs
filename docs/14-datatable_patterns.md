# DataTable — Padrões DOM-driven

## Princípio

O DataTable segue o mesmo modelo do Select e ConfirmDialog: **o DOM é a fonte de verdade**. Estado de interação (seleção, sort, filtro, paginação) vive em `data-rs-*` no root — zero signal na página para comportamento.

---

## Row Actions

O runtime dispara `rs-datatable-action` no root quando qualquer action é clicada. O evento buble com `action` e `rowId` no detail.

```rust
// página escuta uma vez no mount
let cb = Closure::<dyn Fn(leptos::web_sys::CustomEvent)>::wrap(Box::new(move |e| {
    let action = js_sys::Reflect::get(&e.detail(), &"action".into())...;
    let row_id = js_sys::Reflect::get(&e.detail(), &"rowId".into())...;
    match action.as_str() {
        "view"   => { navigate to /sources/{row_id} }
        "edit"   => { navigate to /sources/{row_id}/edit }
        "delete" => { open ConfirmDialog via DOM }
        _ => {}
    }
}));
doc.add_event_listener_with_callback("rs-datatable-action", cb...);
```

O runtime propaga no root antes de disparar o evento:
- `data-rs-current-action` — action clicada
- `data-rs-current-row` — row_id real (via `row_id_fn`)
- `data-rs-current-label` — label da linha (primeira coluna)

---

## Bulk Actions

Mesmo padrão — `rs-datatable-bulk-action` com `action` e `ids[]` no detail.

```rust
doc.add_event_listener_with_callback("rs-datatable-bulk-action", cb...);
// ids = root.get_attribute("data-rs-selected-ids").split(',')
```

---

## Form submission nativa

Prop `name` no `DataTable` — runtime mantém `<input type="hidden">` sincronizado com `data-rs-selected-ids`. Mesmo padrão do Select.

```rust
<DataTable name="selected_sources" data=rows columns=cols />
// form recebe: selected_sources=1,3,7
```

---

## row_id_fn

Por padrão o `row_id` é o índice. Para usar o id real do item:

```rust
<DataTable
    data=sources
    row_id_fn=std::sync::Arc::new(|s: &Source| s.id.to_string())
    ...
/>
```

---

## O que vive no DOM

| Atributo | Descrição |
|---|---|
| `data-rs-selected-ids` | ids selecionados separados por vírgula |
| `data-rs-current-row` | row_id da última action |
| `data-rs-current-label` | label da última action |
| `data-rs-current-action` | action clicada |
| `data-rs-current-bulk-action` | bulk action clicada |
| `data-rs-sort-col` | coluna de sort ativa |
| `data-rs-sort-asc` | direção do sort |
| `data-rs-current-page` | página atual |

---

## Nunca fazer

- `RwSignal` para linha selecionada — usar `data-rs-selected-ids`
- `on:click` por linha para capturar id — usar `rs-datatable-action`
- Registrar listener dentro de `Effect` — registrar uma vez no mount
