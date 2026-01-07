# Canon Record #13 — Specialization vs Substitution: Architectural Decision

**Data:** 2024-12-30  
**Tipo:** Architectural Decision Record  
**Status:** Approved → Virou Canon Rule #13  
**Scope:** Component Design / API Design / Maintenance  

---

## Contexto

Durante o desenvolvimento de MaskedInput e Combobox, surgiu o padrão de "componente especializado vs componente base".

**Questão:** Quando criar componente novo vs adicionar flag ao existente?

---

## Decisão

**Componentes especializados NÃO substituem componentes base.**

### Specialization (✅)
- Cria componente novo
- Estende semântica
- API explícita
- Caso específico

### Substitution (❌)
- Adiciona flag ao base
- Muda comportamento
- API implícita
- "Versão melhorada"

**Regra:**  
Se feature adiciona **constraint semântico** → componente especializado.  
Se feature é **genérica** → adiciona ao base.

---

## Rationale

### Por que separar?

1. **Single Responsibility:**
   - Input: campo genérico
   - MaskedInput: campo + formato

2. **Type Safety:**
   - `<Input mask="cpf" />` → tipo errado aceito
   - `<MaskedInput mask_type=MaskType::CPF />` → tipo correto

3. **Maintenance:**
   - God Component cresce infinitamente
   - Componentes focados são testáveis

4. **Bundle Size:**
   - Input base: ~1KB
   - MaskedInput especializado: +0.5KB
   - Input com tudo: 5KB+

### Casos Resolvidos

| Decisão | Base | Specialized | Motivo |
|---------|------|-------------|--------|
| ✅ | Input | MaskedInput | Formato é constraint |
| ✅ | Select | Combobox | Busca muda natureza |
| ✅ | Button | IconButton | Visual variant |
| ❌ | Input com mask flag | - | Flag esconde decisão |

---

## Implementação

### Pattern: Componentes Separados
```rust
// Base
pub struct Input { /* ... */ }

// Specialized
pub struct MaskedInput {
    // Pode compor Input internamente
    // Ou reimplementar
    // Mas API é distinta
}
```

### Pattern: Tipos Explícitos
```rust
// ✅ Type-safe
mask_type: MaskType::CPF

// ❌ Stringly-typed
mask: "cpf"
```

---

## Consequências

### Positivas
✅ APIs explícitas  
✅ Type safety  
✅ Maintenance simples  
✅ Bundle size controlado  
✅ Single Responsibility  

### Negativas
⚠️ Mais componentes para aprender  
⚠️ Decisão requer análise  

**Benefício supera custo.**

---

## Exemplos

### ✅ Correto
```rust
<Input value=name />
<MaskedInput value=cpf mask_type=MaskType::CPF />
```

### ❌ Errado
```rust
<Input value=cpf mask="cpf" />
```

---

## Decisão Final

**Specialization > Substitution**  
**Explicit Types > Magic Flags**

---

## Referências

- Canon Rule #13: `/docs/canon/rules/canon-rule-13-specialization-vs-substitution.md`
- Implementações: Input, MaskedInput, Select, Combobox
