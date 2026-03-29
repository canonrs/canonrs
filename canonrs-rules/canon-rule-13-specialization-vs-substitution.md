# Canon Rule #13 — Specialization vs Substitution


**Status:** ENFORCED
**Severity:** HIGH
**Scope:** components, architecture
**Version:** 1.0.0
**Date:** 2025-01-16

---
**Enunciado curto (para lembrar fácil):**  
Um componente especializado nunca substitui seu componente base. Ele estende semântica, não a reescreve.

---

## Definição Formal
```
Base Component        = Componente genérico, fundacional
Specialized Component = Extensão semântica do base, caso específico
```

**Specialization** é sobre **estender semântica**.  
**Substitution** é sobre **reescrever comportamento**.

**A Rule #13 proíbe substitution disfarçada de specialization.**

---

## 🔒 O QUE ESSA RULE PROÍBE (binding)

### ❌ É PROIBIDO

#### 1. Usar especializado como default
```rust
// ❌ ERRADO - MaskedInput como default
fn UserForm() {
    view! {
        <MaskedInput value=name />  // nome não precisa de máscara!
    }
}

// ✅ CORRETO
fn UserForm() {
    view! {
        <Input value=name />  // genérico para nome
        <MaskedInput value=cpf mask_type=MaskType::CPF />  // especializado
    }
}
```

#### 2. Flags mágicas que transformam base em especializado
```rust
// ❌ PROIBIDO
<Input mask="cpf" />              // Input não conhece máscara
<Select searchable=true />        // Select não conhece busca
<Button loading=true spinner />   // Button não conhece loading state

// ✅ CORRETO
<MaskedInput mask_type=MaskType::CPF />
<Combobox />
<LoadingButton />
```

#### 3. Componentes "inteligentes" que decidem sozinhos
```rust
// ❌ PROIBIDO
<SelectOrCombo options=list />  // escolha automática

// ✅ CORRETO
if list.len() < 50 {
    <Select options=list />
} else {
    <Combobox options=list />
}
```

#### 4. Substituição silenciosa
```rust
// ❌ PROIBIDO - Combobox "melhorando" Select
type Select = Combobox;  // ❌

// ✅ CORRETO - são componentes distintos
<Select />   // caso genérico
<Combobox /> // caso com busca
```

---

## ✅ O QUE A RULE EXIGE

### 1. Base Component SEMPRE existe

Todo componente especializado **deve ter um base component genérico**:

| Base         | Existe como? | Propósito                    |
|--------------|--------------|------------------------------|
| `Input`      | ✅ Type 1    | Campo genérico               |
| `Select`     | ✅ Type 1    | Seleção nativa               |
| `Button`     | ✅ Type 1    | Ação genérica                |
| `DataTable`  | ✅ Type 3    | Tabela semântica human-scale |

### 2. Especialização é EXPLÍCITA e SEPARADA

| Base         | Specialized      | Relação                          |
|--------------|------------------|----------------------------------|
| `Input`      | `MaskedInput`    | Extensão (formato obrigatório)   |
| `Select`     | `Combobox`       | Extensão (busca + overlay)       |
| `Button`     | `IconButton`     | Extensão (visual variant)        |
| `DataTable`  | `VirtualTable`   | **Mudança de natureza** (Rule #17)|

### 3. Especializado DEPENDE semanticamente do Base

**MaskedInput:**
- ✅ Reusa tokens do Input
- ✅ Reusa Field wrapper
- ✅ Reusa validation patterns
- ✅ Adiciona: máscara determinística

**Combobox:**
- ✅ Reusa conceito de "seleção"
- ✅ Reusa option rendering
- ✅ Adiciona: busca, overlay, virtualização

---

## 🧠 POR QUE ISSO É UMA RULE

Esta Rule resolve o problema de **component creep**:

### Sem Rule #13:
```rust
// Input vira God Component
<Input 
    mask="cpf"           // ❌
    autocomplete=true    // ❌
    debounce=300         // ❌
    validation="email"   // ❌
    prefix_icon="user"   // ❌
    clearable=true       // ❌
/>
```

**Problema:** Input tenta ser tudo. Vira impossível manter.

### Com Rule #13:
```rust
<Input />                    // genérico
<MaskedInput />              // formato
<AutocompleteInput />        // busca
<DebouncedInput />           // delay
<EmailInput />               // validação
<IconInput />                // visual
```

**Solução:** Componentes pequenos, focados, composíveis.

---

## 🏷️ CLASSIFICAÇÃO DA RULE

| Campo       | Valor                              |
|-------------|------------------------------------|
| Rule ID     | Canon Rule #13                     |
| Categoria   | Component Design / Specialization  |
| Tipo        | Architectural Rule                 |
| Severidade  | **High**                           |
| Scope       | All Components / DX / Maintenance  |
| Violação    | **Review Blocker**                 |

---

## 🧬 MATRIZ CANÔNICA: Base → Specialized

| Base Component | Specialized Component | Tipo de Extensão      | Rule Relacionada |
|----------------|----------------------|-----------------------|------------------|
| `Input`        | `MaskedInput`        | Formato obrigatório   | #13              |
| `Input`        | `EmailInput`         | Validação específica  | #13              |
| `Select`       | `Combobox`           | Busca + overlay       | #12, #13         |
| `Button`       | `IconButton`         | Visual variant        | #13              |
| `Button`       | `LoadingButton`      | Estado especializado  | #13              |
| `DataTable`    | `VirtualTable`       | **Mudança de natureza**| #14, #17        |
| `Grid`         | -                    | Base (sem especialização)| -             |

**Nota:** DataTable → VirtualTable NÃO é specialization, é **mudança de scale** (Rule #17).

---

## 🎯 COMO A RULE #13 SE POSICIONA NO CANON
```
Rule #12 (Select vs Combobox)       ← Caso específico
    ↓
Rule #13 (Specialization vs Substitution) ← Princípio geral
    ↓
Rule #17 (Human vs Machine Scale)   ← Meta-fundamento
```

**Relacionamento:**
- **#12** é exemplo de #13
- **#13** explica quando componente muda (especialização)
- **#17** explica quando componente muda de natureza (scale)

---

## 📊 Specialization vs Substitution

| Aspecto              | Specialization (✅)        | Substitution (❌)         |
|----------------------|---------------------------|---------------------------|
| **Relação com base** | Estende                   | Substitui                 |
| **Tokens**           | Reutiliza                 | Redefine                  |
| **API**              | Adiciona props            | Muda comportamento base   |
| **Semântica**        | Preserva + adiciona       | Reescreve                 |
| **Uso**              | Caso específico           | "Versão melhorada"        |
| **Exemplo**          | MaskedInput extends Input | Input com flag mask       |

---

## 🧪 COMO ESSA RULE É APLICADA NA PRÁTICA

### Em Code Review

**Checklist obrigatório:**

- [ ] PR adiciona flag ao componente base que muda comportamento?
- [ ] PR usa componente especializado onde base resolveria?
- [ ] PR cria componente "inteligente" que escolhe automaticamente?
- [ ] Nova feature deveria ser componente separado?

**Se sim para qualquer pergunta → PR NÃO APROVA (violação da #13)**

### Em Design de Componentes

**Decisão: Adicionar feature ao componente X**

1. Feature é **caso específico** (CPF, busca, loading)? → Criar especializado
2. Feature é **genérica** (disabled, placeholder, error)? → Adicionar ao base
3. Feature **muda natureza** (scale, performance)? → Novo componente (Rule #17)

---

## 🚫 ANTI-PATTERNS

### ❌ God Component
```rust
// PROIBIDO - Input tentando ser tudo
<Input
    type="text"
    mask="cpf"              // deveria ser MaskedInput
    autocomplete=true       // deveria ser AutocompleteInput
    debounce=300            // deveria ser DebouncedInput
    currency=true           // deveria ser CurrencyInput
    validation="email"      // deveria ser EmailInput
/>
```

**Problema:** 
- Impossível testar
- Impossível manter
- Props conflitantes
- Bundle size explode

### ❌ Flags Mágicas
```rust
// PROIBIDO - Select com flag searchable
<Select 
    options=many_options
    searchable=true  // ❌ isso é Combobox!
/>

// CORRETO
<Combobox options=many_options />
```

### ❌ Substituição Silenciosa
```rust
// PROIBIDO - redefinir componente base
pub type Input = MaskedInput;  // ❌

// CORRETO - componentes distintos
pub struct Input { /* ... */ }
pub struct MaskedInput { /* ... */ }
```

---

## ✅ PADRÕES CORRETOS

### Pattern 1: Extensão Clara
```rust
// Base
#[component]
pub fn Input(
    value: Signal<String>,
    on_change: Callback<String>,
) -> impl IntoView { /* ... */ }

// Especializado (estende base)
#[component]
pub fn MaskedInput(
    value: Signal<String>,
    on_change: Callback<String>,
    mask_type: MaskType,  // ← adiciona semântica
) -> impl IntoView {
    // Internamente pode usar Input ou reimplementar
    // Mas API é explícita: MaskedInput ≠ Input
}
```

### Pattern 2: Composição
```rust
// Especializado compõe base
#[component]
pub fn EmailInput(
    value: Signal<String>,
    on_change: Callback<String>,
) -> impl IntoView {
    view! {
        <Field label="Email">
            <Input
                value=value
                on_change=on_change
                type="email"
            />
        </Field>
    }
}
```

### Pattern 3: Decisão Explícita no Parent
```rust
// Parent decide qual componente
#[component]
fn DocumentForm() -> impl IntoView {
    view! {
        <Input value=name />               // genérico
        <MaskedInput value=cpf mask_type=MaskType::CPF />  // especializado
        <EmailInput value=email />         // especializado
    }
}
```

---

## 🎓 Design Principles

### 1. Specialization Adds Semantics
```rust
Input       → campo genérico
MaskedInput → campo + formato obrigatório
EmailInput  → campo + validação email
```

Cada especialização **adiciona constraint semântico**.

### 2. Base is Always Available
```rust
// SEMPRE possível usar o base
<Input value=any_text />

// Especializado quando constraint faz sentido
<MaskedInput value=cpf mask_type=MaskType::CPF />
```

### 3. No Magic Flags
```rust
// ❌ Magic
<Input mask="cpf" />

// ✅ Explicit
<MaskedInput mask_type=MaskType::CPF />
```

Flags escondem decisões. Tipos explicitam.

---

## 📚 Exemplos Práticos

### Input → MaskedInput
```rust
// Cenário: formulário de cadastro

view! {
    <Input value=name placeholder="Nome completo" />
    <MaskedInput value=cpf mask_type=MaskType::CPF />
    <MaskedInput value=phone mask_type=MaskType::Phone />
    <Input value=email type="email" />
}
```

**Rationale:**
- Nome: texto livre → Input
- CPF: formato obrigatório → MaskedInput
- Telefone: formato obrigatório → MaskedInput
- Email: validação HTML → Input (type="email")

### Select → Combobox
```rust
// Cenário: seleção de país

// <50 países: Select
<Select options=short_list />

// 200+ países: Combobox
<Combobox options=all_countries />
```

**Rationale:** Decisão baseada em Rule #12 + #17 (scale).

---

## 🏁 VEREDITO FINAL

- ✅ É a **Canon Rule #13**
- ✅ **Princípio de especialização** (não caso específico)
- ✅ Posiciona-se entre #12 (exemplo) e #17 (meta)
- ✅ Previne **God Components** e **feature creep**
- ✅ Força **tipos explícitos** sobre flags mágicas
- ✅ Ela **bloqueia PR** que viola separation of concerns

---

## Referências

**Rules Relacionadas:**
- Canon Rule #12 (Select vs Combobox): Exemplo de especialização
- Canon Rule #17 (Human vs Machine Scale): Quando muda de natureza

**Implementações:**
- Base: `/packages-rust/rs-design/src/ui/{input,select,button}/`
- Specialized: `/packages-rust/rs-design/src/ui/{masked_input,combobox}/`

**Teoria:**
- Single Responsibility Principle
- Composition over Configuration
- Explicit over Implicit

---

**Mantra:** *Especialização estende semântica. Substituição a reescreve. Nunca confunda.*
