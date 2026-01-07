# Canon Design System

Sistema de regras arquiteturais e decisões de design do rs-design.

## 📚 Estrutura
```
docs/canon/
├── rules/          ← Regras permanentes (normativas)
└── records/        ← Histórico de decisões (ADRs)
```

---

## 🎯 Canon Rules (Normativas)

### Component Choice & Architecture

- **[Rule #12](rules/canon-rule-12-select-vs-combobox.md)** — Select vs Combobox  
  *"Select e Combobox são semanticamente diferentes e NÃO podem ser substitutos"*

- **[Rule #14](rules/canon-rule-14-datatable-vs-virtualtable.md)** — DataTable vs VirtualTable  
  *"Semântica não escala. Performance não semantiza."*

- **[Rule #15](rules/canon-rule-15-pagination-vs-virtualization.md)** — Pagination vs Virtualization  
  *"Pagination é UX. Virtualization é engine. Nunca misture."*

### Data Operations

- **[Rule #16](rules/canon-rule-16-client-vs-server-filtering.md)** — Client-side vs Server-side Filtering  
  *"Filtering é sobre onde o dado está. Não sobre onde o usuário vê."*

- **[Rule #18](rules/canon-rule-18-client-vs-server-sorting.md)** — Client-side vs Server-side Sorting  
  *"Sort onde os dados estão. Não onde o usuário clica."*

### Data Architecture

- **[Rule #19](rules/canon-rule-19-streaming-vs-snapshot.md)** — Streaming vs Snapshot  
  *"Streaming é para fluxo contínuo. Snapshot é para estado fixo."*

- **[Rule #20](rules/canon-rule-20-realtime-vs-eventual.md)** — Real-time vs Eventual Consistency  
  *"Real-time é garantia. Eventual é aceitação."*

### Meta-Rule (Foundational)

- **[Rule #17](rules/canon-rule-17-human-vs-machine-scale.md)** — Human-scale vs Machine-scale Components  
  *"Build for humans or build for machines. Never pretend one is the other."*  
  **Meta-rule que fundamenta todas as anteriores (#12, #14, #15, #16)**

---

## 📖 Canon Records (Decisões Arquiteturais)

### Fundamentais (01-11)

- **[Record #01](records/canon-record-01-types.md)** — Type System
- **[Record #02](records/canon-record-02-ownership.md)** — Ownership Patterns
- **[Record #03](records/canon-record-03-lists.md)** — List Rendering
- **[Record #04](records/canon-record-04-hydration.md)** — Hydration Strategy
- **[Record #05](records/canon-record-05-ssr-effects.md)** — SSR Effects
- **[Record #06](records/canon-record-06-visual-state.md)** — Visual State Management
- **[Record #07](records/canon-record-07-token-governance.md)** — Token Governance
- **[Record #08](records/canon-record-08-overlay-islands.md)** — Overlay Islands
- **[Record #09](records/canon-record-09-clipboard-apis.md)** — Clipboard APIs
- **[Record #10](records/canon-record-10-modal-state.md)** — Modal State
- **[Record #11](records/canon-record-11-multi-callback-ownership.md)** — Multi-Callback Ownership

### Arquiteturais (12-20)

- **[Record #12](records/canon-record-12-architectural-decision.md)** — Select vs Combobox
- **[Record #14](records/canon-record-14-architectural-decision.md)** — DataTable vs VirtualTable
- **[Record #15](records/canon-record-15-architectural-decision.md)** — Pagination vs Virtualization
- **[Record #16](records/canon-record-16-architectural-decision.md)** — Client vs Server Filtering
- **[Record #17](records/canon-record-17-meta-architectural-decision.md)** — Human-scale vs Machine-scale (Meta)
- **[Record #18](records/canon-record-18-architectural-decision.md)** — Client vs Server Sorting
- **[Record #19](records/canon-record-19-architectural-decision.md)** — Streaming vs Snapshot
- **[Record #20](records/canon-record-20-architectural-decision.md)** — Real-time vs Eventual Consistency

---

## 🎓 Como Usar

### Para Desenvolvedores

1. **Escolhendo Componentes:** Consulte Rules #12, #14, #15
2. **Decisões de Dados:** Consulte Rules #16, #18, #19, #20
3. **Entendendo Scale:** Leia Rule #17 (Meta-Rule)

### Para Code Review

- Rules são **Review Blockers** (severidade High/Critical)
- Toda escolha de componente deve ter rationale documentado
- Violações não aprovam PR

### Para Arquitetos

- Records 01-11: Fundamentos técnicos
- Records 12-20: Decisões arquiteturais
- Rule #17: Filosofia de design (meta)

---

## 📊 Mapa de Decisões
```
Scale (Rule #17) — Meta-Rule
    ├─ Component Choice
    │   ├─ Select vs Combobox (#12)
    │   ├─ DataTable vs VirtualTable (#14)
    │   └─ Pagination vs Virtualization (#15)
    └─ Data Operations
        ├─ Filtering (#16)
        ├─ Sorting (#18)
        ├─ Streaming (#19)
        └─ Real-time (#20)
```

---

## 🔗 Referências Externas

- **Implementações:** `/packages-rust/rs-design/src/ui/`
- **Tokens:** `/packages-rust/rs-design/src/tokens/`
- **Tipos:** `/packages-rust/rs-design/docs/canon/01-types.md`

---

**Mantido por:** rs-design team  
**Última atualização:** 2024-12-30
