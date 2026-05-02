# Focus Lifecycle — CR-434

## Princípio

Todo overlay que abre deve gerenciar o foco de forma consistente:
salva onde estava o foco antes de abrir, move para dentro do overlay ao abrir,
e restaura o foco original ao fechar.

---

## Regras

**CR-434-A — Salvar foco antes de abrir**
`prev_focus.set(focus::active_element())` no início do `fn open`.
Sempre via `std::rc::Rc<std::cell::Cell<Option<Element>>>`.

**CR-434-B — Mover foco ao abrir**
Modal: `focus::focus_first(content)` — foca o primeiro elemento focável.
Non-modal: não move o foco — deixa o browser gerenciar.

**CR-434-C — Restaurar foco ao fechar**
`prev_focus.take()` → `.focus()` no elemento original.
Feito via `setTimeout` para garantir que o DOM está estável após a transição.

**CR-434-D — focusout em non-modal usa delay**
Ver CR-432. `setTimeout(0)` antes de verificar `active_element`.
Nunca fechar diretamente no `focusout`.

**CR-434-E — Modal usa focus trap (inert)**
Modais usam `inert::set_inert_background` para impedir Tab de sair.
Non-modal NÃO usa inert — Tab sai naturalmente.

---

## Matriz por tipo

| Overlay | Salva foco | Move foco | Restaura foco | inert | focusout delay |
|---|---|---|---|---|---|
| Dialog | ✔ | ✔ focus_first | ✔ | ✔ | — |
| ConfirmDialog | ✔ | ✔ focus_first | ✔ | ✔ | — |
| Sheet | ✔ | ✔ focus_first | ✔ | — | — |
| Drawer | ✔ | ✔ focus_first | ✔ | — | — |
| Popover | ✔ | ✗ | ✔ | ✗ | ✔ |
| DropdownMenu | — | — | — | ✗ | — |
| HoverCard | — | — | — | ✗ | — |
| ContextMenu | — | — | — | ✗ | — |

---

## Padrão de implementação

```rust
fn open(root: &Element, prev_focus: &Rc<Cell<Option<Element>>>) {
    prev_focus.set(focus::active_element()); // CR-434-A
    state::open(root);
    focus::focus_first(content);            // CR-434-B (modal)
}

fn close(root: &Element, prev_focus: &Rc<Cell<Option<Element>>>) {
    state::close(root);
    // via setTimeout para aguardar transição
    if let Some(el) = prev_focus.take() {
        let _ = el.dyn_into::<HtmlElement>().unwrap().focus(); // CR-434-C
    }
}
```

---

## Nunca fazer

- Mover foco em non-modal — deixa o browser gerenciar
- Fechar no `focusout` sem delay em non-modal
- Usar `inert` em non-modal
- Ignorar o restore do foco ao fechar
