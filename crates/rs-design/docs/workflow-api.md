# Workflow Component API

**Status:** Strict Compliance  
**Version:** 1.0.0  
**Canon Rules:** #21, #24, #36, #37

---

## Quick Start
```rust
use rs_design::components::workflow::*;

let steps = create_rw_signal(HashMap::from([
    (StepId("step1".into()), StepStatus::Pending),
]));

view! {
    <Workflow steps=steps.into()>
        <Step id label status />
    </Workflow>
}
```

---

## Core Types

### StepStatus (7 estados canônicos)
- `Pending` - Não iniciado
- `Active` - Em execução
- `Completed` - Finalizado com sucesso
- `Blocked` - Impedido por gate
- `Skipped` - Ignorado por condição
- `Failed` - Erro
- `NotApplicable` - Não se aplica

### GateType (4 tipos)
- `Evidence` - Documento obrigatório
- `Permission` - Role/scope
- `Rule` - Política/compliance
- `Temporal` - SLA/data

---

## Components

### `<Workflow>`
Orquestrador principal.

**Props:**
- `steps: Signal<HashMap<StepId, StepStatus>>` - Estados
- `dependencies: Vec<DependencyRule>` - Dependências (opcional)
- `on_audit: Option<AuditCallback>` - Callback de auditoria (opcional)

### `<Step>`
Unidade semântica.

**Props:**
- `id: StepId` - Identificador único
- `label: String` - Título
- `status: Signal<StepStatus>` - Estado atual
- `gates: Vec<Gate>` - Gates (opcional)
- `on_click: Option<Callback<StepId>>` - Handler (opcional)

---

## Functions

### `transition_step()`
Transiciona step respeitando regras.
```rust
transition_step(
    StepId("step1".into()),
    StepStatus::Completed,
    Some("user123".into()),
    Some("Approved".into()),
);
```

### `can_transition()`
Valida se transição é permitida.
```rust
if can_transition(StepStatus::Active, StepStatus::Completed) {
    // válido
}
```

---

## Data Attributes (Auditabilidade)

Cada step expõe:
```html
<li
  data-step-id="step1"
  data-status="blocked"
  data-gates="evidence,permission"
  aria-disabled="true"
/>
```

---

## Compliance

✅ Strict Canon (#36)  
✅ Semantic tokens (#21)  
✅ Provider-safe (#37)  
✅ ISO-ready (auditável)  
✅ A11y compliant (ARIA)

---

**Próximo:** Integração com Forms + Tree
