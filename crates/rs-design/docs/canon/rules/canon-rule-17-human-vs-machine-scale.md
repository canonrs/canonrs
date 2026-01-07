# Canon Rule #17 — Human-scale vs Machine-scale Components

**Enunciado curto (para lembrar fácil):**  
Build for humans or build for machines. Never pretend one is the other.

---

## 🧠 A META-RULE

Esta é a **generalização conceitual** que fundamenta todas as Canon Rules anteriores:

| Canon Rule | Human-scale | Machine-scale |
|------------|-------------|---------------|
| **#12** | Select | Combobox |
| **#14** | DataTable | VirtualTable |
| **#15** | Pagination | Virtualization |
| **#16** | Client Filtering | Server Filtering |
| **#18** | Client Sorting | Server Sorting |
| **#19** | Snapshot | Streaming |
| **#20** | Eventual Consistency | Real-time |

**Padrão universal:**  
Quando você cruza de human-scale → machine-scale, **o componente muda fundamentalmente**.

---

## Definição Formal
```
Human-scale  = Componentes para quantidades que humanos processam (~1-1000)
Machine-scale = Componentes para quantidades que máquinas processam (10k-1M+)
```

**Human-scale** é sobre **cognição humana, UX rica, semântica**.  
**Machine-scale** é sobre **performance técnica, densidade, escalabilidade**.

---

## 🔒 O QUE ESSA RULE PROÍBE

### ❌ É PROIBIDO

- Usar componente human-scale para dados machine-scale
- Adicionar flags `virtual=true`, `big_data=true` em componentes human-scale
- Criar "componente inteligente" que escolhe automaticamente
- Usar componente machine-scale para dados human-scale (over-engineering)
- Escalar componente human-scale além do limite de cognição humana
- Desescalar componente machine-scale perdendo performance

👉 **Scale é decisão arquitetural, não configuração.**

---

## ✅ O QUE A RULE EXIGE

Toda decisão de componente **DEVE** considerar a escala dos dados:

| Aspecto                    | Human-scale      | Machine-scale    |
|----------------------------|------------------|------------------|
| **Quantidade**             | 1-1000           | 10k-1M+          |
| **Cognição**               | Humano processa  | Máquina processa |
| **UX**                     | Rica, semântica  | Performática     |
| **Semântica HTML**         | Completa         | Limitada/Divs    |
| **Acessibilidade**         | Total            | Parcial          |
| **SSR**                    | Preferencial     | Opcional         |
| **Performance target**     | UX > performance | Performance > UX |

---

## 🎯 THE THRESHOLD RULE

**Quando você cruza o threshold, o componente muda de natureza.**

### Thresholds Canônicos
```
Human-scale:    1 -----------------> ~1000
                ↑                    ↑
                trivial              cognitive limit
                
Machine-scale:  10k ----------------> 1M+
                ↑                     ↑
                performance           database scale
```

**Gray zone (1k-10k):**  
Área de decisão consciente. Pode ir para qualquer lado dependendo de:
- Contexto de uso
- Requisitos de UX vs Performance
- SSR necessário?
- Acessibilidade crítica?

---

## 📊 Human-scale vs Machine-scale: The Fundamental Divide

| Aspecto              | Human-scale                | Machine-scale              |
|----------------------|----------------------------|----------------------------|
| **Target**           | Cognição humana            | Performance técnica        |
| **Quantidade**       | 1-1000                     | 10k-1M+                    |
| **UX**               | Rica, exploratória         | Funcional, eficiente       |
| **Semântica**        | HTML nativo                | Divs otimizados            |
| **Acessibilidade**   | Total (ARIA completo)      | Limitada (trade-offs)      |
| **SSR**              | ✅ Preferencial            | ⚠️ Opcional/impossível     |
| **Performance**      | O(n) aceitável             | O(1) obrigatório           |
| **Navegação**        | Discreta (páginas, clicks) | Contínua (scroll, stream)  |
| **Componente Type**  | Type 1-3                   | Type 4                     |
| **Tokens**           | Todos + Famílias A-C       | Canônicos + Família D      |
| **Exemplos**         | Select, DataTable, Grid    | VirtualTable, Logs engine  |

---

## 🚫 ANTI-PATTERNS (Meta-level)

### ❌ Escalar componente human-scale além do limite
```rust
// PROIBIDO - DataTable com 50k rows
<DataTable data=fifty_thousand_rows />
```

**Problema:**  
- Componente construído para cognição humana
- Não otimizado para rendering em escala
- Performance degrada exponencialmente

### ❌ Usar componente machine-scale para dados human-scale
```rust
// PROIBIDO - VirtualTable para 10 items
<VirtualTable rows=ten_items />
```

**Problema:**  
- Over-engineering
- Perde semântica e acessibilidade
- Complexidade desnecessária

### ❌ Criar componente "universal" com flags
```rust
// PROIBIDO - tentativa de unificar escalas
<Table 
    data=rows
    virtual={rows.len() > 1000}  // ❌
    paginated={rows.len() > 100} // ❌
/>
```

**Problema:**  
- God component
- Esconde decisões arquiteturais críticas
- Viola separation of concerns

---

## ✅ PADRÕES CORRETOS
```rust
// Human-scale (200 users, rich UX)
<DataTable<User>
    data=users
    columns=vec![/* ... */]
    actions=|u| view! { <EditButton /> }
/>

// Machine-scale (100k logs, performance)
<VirtualTable
    rows=logs
    row_height=36.0
    viewport_height=600.0
/>
```

---

## 🏷️ CLASSIFICAÇÃO DA RULE

| Campo       | Valor                              |
|-------------|------------------------------------|
| Rule ID     | Canon Rule #17                     |
| Categoria   | Meta-Rule / Design Philosophy      |
| Tipo        | Foundational Architectural Rule    |
| Severidade  | **Critical**                       |
| Scope       | All Components / System-wide       |
| Violação    | **Review Blocker**                 |

---

## 📚 Scale e Canon Type System

| Type  | Scale          | Exemplos                    |
|-------|----------------|-----------------------------|
| 1     | Human-scale    | Button, Input, Select       |
| 2     | Human-scale    | Grid, MaskedInput           |
| 3     | Human-scale    | DataTable, Combobox, Tabs   |
| 4     | Machine-scale  | VirtualTable, Performance   |

**Type 4 só existe em machine-scale.**  
Se é human-scale, é Type 1-3.

---

## 🧬 The DNA of Scale

**Human-scale DNA:**
```
Semântica → Cognição → UX Rica → SSR → Acessibilidade Total
```

**Machine-scale DNA:**
```
Performance → Matemática → Eficiência → Client-only → Trade-offs
```

**Você não pode misturar DNAs.**

---

## 🏁 VEREDITO FINAL

- ✅ É a **Canon Rule #17**
- ✅ É a **Meta-Rule** que fundamenta #12, #14, #15, #16, #18, #19, #20
- ✅ É **foundational**, não tática
- ✅ Ela **explica o porquê** de todas as outras
- ✅ Ela **bloqueia God Components** e over-engineering
- ✅ Ela força **consciência de scale** em toda decisão

---

## Referências

**Rules Derivadas:**
- Canon Rule #12 (Select vs Combobox): Human vs Machine selection
- Canon Rule #14 (DataTable vs VirtualTable): Human vs Machine tables
- Canon Rule #15 (Pagination vs Virtualization): Human vs Machine navigation
- Canon Rule #16 (Client vs Server Filtering): Human vs Machine filtering
- Canon Rule #18 (Client vs Server Sorting): Human vs Machine sorting
- Canon Rule #19 (Streaming vs Snapshot): Machine streaming vs human snapshots
- Canon Rule #20 (Real-time vs Eventual): Machine guarantees vs human acceptance

**Canon Record:**
- Canon Record #17: `/docs/canon/records/canon-record-17-meta-architectural-decision.md`

**Teoria:**
- Miller's Law (cognitive limits)
- Big-O Notation (performance math)
- Semantic Web (HTML design principles)

---

**Mantra:** *Build for humans or build for machines. Never pretend one is the other.*
