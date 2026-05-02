# Overlay Stack — CR-433

## Princípio

Todos os overlays compartilham um stack global único.
O stack gerencia z-index, prioridade de eventos e política de fechamento.

---

## Tipos de overlay

| Tipo | Kind | Modal |
|---|---|---|
| Dialog | `dialog` | ✔ |
| ConfirmDialog | `confirm-dialog` | ✔ |
| Modal | `modal` | ✔ |
| Sheet | `sheet` | ✔ |
| Drawer | `drawer` | ✔ |
| AlertDialog | `alert-dialog` | ✔ |
| Popover | `popover` | ✗ |
| DropdownMenu | `dropdown-menu` | ✗ |
| HoverCard | `hover-card` | ✗ |
| ContextMenu | `context-menu` | ✗ |
| Tooltip | `tooltip` | ✗ |

---

## Regras do stack

**CR-433-A — Modal bloqueia non-modal**
Non-modal overlays não abrem quando qualquer modal está no stack.
`stack::has_modal_open()` deve ser consultado no `fn open()` de cada non-modal.

**CR-433-B — Escape fecha apenas o topo**
Keydown é despachado apenas para o overlay no topo do stack via `stack::is_top`.

**CR-433-C — Click despacha para o topo**
Quando stack ativo, click é despachado apenas para o topo.
Quando stack vazio, click é despachado para todos os registros.

**CR-433-D — z-index automático**
Cada overlay recebe `BASE_Z + (len * 10)` ao entrar no stack.
Overlay mais recente tem sempre z-index maior.

**CR-433-E — scroll-lock liberado quando stack vazio**
Modais ativam scroll-lock. O lock é liberado apenas quando o stack está vazio.

---

## Escopo de eventos

```
document (bubble)  → click   → stack → topo → root
window  (capture)  → keydown → stack → topo → root
root               → focusout → setTimeout(0) → check active_element
```

---

## Múltiplos overlays simultâneos

```
stack: [popover(z=1000), dialog(z=1010)]

Escape → fecha dialog (topo) → stack: [popover(z=1000)]
Escape novamente → fecha popover

dialog abre sobre popover → popover permanece aberto (non-modal)
click fora do dialog → fecha dialog, popover continua
```

---

## Nunca fazer

- Abrir non-modal quando modal está no stack
- Registrar listener global fora do `stack::ensure_global_listeners()`
- Usar z-index hardcoded — sempre via `stack::push`
- Fechar todos os overlays no Escape — apenas o topo
