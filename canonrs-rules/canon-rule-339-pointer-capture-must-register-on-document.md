# Canon Rule #339: Pointer Capture Must Register Move and Up on Document

**Status:** ENFORCED
**Severity:** HIGH
**Version:** 1.0.0
**Date:** 2026-04-07

**Category:** interaction-architecture
**Tags:** pointer, drag, event, capture, document, wasm, interaction
**Language:** EN

---

**Intro:**
Any interaction that uses pointer capture for drag behavior must register `pointermove` and `pointerup` on the `document`, not on the handle element. Registering on the element causes events to stop firing when the pointer leaves the element boundary during fast movement.

**Problem:**
`pointermove` registered on the drag handle stops firing when the pointer moves faster than the element boundary. The drag freezes mid-interaction even though `setPointerCapture` was called.

**Solution:**
Register `pointerdown` on the handle. Register `pointermove` and `pointerup` on `document`. Use `setPointerCapture` on the handle to ensure the pointer ID is tracked correctly.

**Signals:**
- drag stops when moving fast
- bar does not follow pointer outside element bounds
- `pointerup` not detected when releasing outside the handle
- interaction works correctly on slow movement but breaks on fast movement

**Search Intent:**
pointermove stops firing, drag freezes fast movement, pointer capture not working, pointerup not detected outside element

**Keywords:**
pointermove document, pointer capture drag, setPointerCapture, pointerup outside element, wasm drag interaction

---

## Principle

`setPointerCapture` routes pointer events to the element but does not guarantee delivery if the listeners are on the element itself and the pointer leaves its bounds. Registering on `document` ensures all pointer events are captured regardless of position.

---

## Patterns

### Forbidden Pattern
```rust
// pointermove on handle — stops firing when pointer leaves bounds
handle.add_event_listener_with_callback("pointerdown", on_down).ok();
handle.add_event_listener_with_callback("pointermove", on_move).ok();
handle.add_event_listener_with_callback("pointerup", on_up).ok();
```

### Canonical Pattern
```rust
// pointerdown on handle, move and up on document
handle.add_event_listener_with_callback("pointerdown", on_down).ok();
handle.set_pointer_capture(e.pointer_id()).ok(); // inside on_down
let doc: &web_sys::EventTarget = document.as_ref();
doc.add_event_listener_with_callback("pointermove", on_move).ok();
doc.add_event_listener_with_callback("pointerup", on_up).ok();
```

---

## Contract

### Enforcement

- `pointerdown` is registered on the interactive element (handle, trigger, thumb)
- `pointermove` must be registered on `document`
- `pointerup` must be registered on `document`
- `setPointerCapture` must be called inside `pointerdown` handler
- pointer ID must be validated in `pointermove` and `pointerup` handlers
- applies to: resizable, slider, carousel, any drag-based interaction

### Exceptions

None.

---

## Version History

- 1.0.0 - Initial definition — derived from resizable drag failure (2026-04-07)
