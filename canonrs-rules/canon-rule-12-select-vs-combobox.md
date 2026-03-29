# Canon Rule #12 — Select vs Combobox


**Status:** ENFORCED
**Severity:** MEDIUM
**Scope:** components, ui
**Version:** 1.0.0
**Date:** 2025-01-16

---
**Enunciado curto (para lembrar fácil):**  
Select e Combobox são componentes semanticamente diferentes e **NÃO podem ser usados como substitutos entre si**.

---

## Definição Formal
```
Select   = Controle HTML nativo (Type 1)
Combobox = Componente interativo com overlay e busca (Type 3)
```

---

## 🔒 O QUE ESSA RULE PROÍBE (binding)

### ❌ É PROIBIDO

- Usar Combobox onde Select resolve
- Adicionar busca, overlay ou JS ao Select
- Criar "Select inteligente", "Select avançado", "SelectSearch"
- Criar componente híbrido `SelectOrCombobox`
- Escolha automática baseada em heurística (`if items > x`)

👉 **A escolha deve ser EXPLÍCITA e JUSTIFICADA.**

---

## ✅ O QUE A RULE EXIGE

Toda decisão entre Select e Combobox **DEVE** considerar:

| Critério              | Obrigatório            |
|-----------------------|------------------------|
| SSR crítico?          | **Select**             |
| Lista pequena (<50)?  | **Select**             |
| Mobile-first?         | **Select**             |
| Busca necessária?     | **Combobox**           |
| Overlay necessário?   | **Combobox**           |
| UX rica > performance?| **Combobox**           |

---

## 🧠 POR QUE ISSO É UMA RULE (e não só guideline)

Porque ela afeta diretamente:

| Área          | Impacto                      |
|---------------|------------------------------|
| Arquitetura   | Type 1 vs Type 3             |
| SSR           | Native vs Client-only        |
| Performance   | O(1) vs O(n)                 |
| Acessibilidade| HTML nativo vs ARIA manual   |
| Mobile UX     | OS picker vs overlay         |
| Bundle size   | ~2KB vs ~8KB                 |
| Governança    | Evita decisões emocionais    |

**Isso não é preferência estética. É decisão estrutural.**

---

## 🏷️ CLASSIFICAÇÃO DA RULE

| Campo       | Valor                          |
|-------------|--------------------------------|
| Rule ID     | Canon Rule #12                 |
| Categoria   | Component Choice               |
| Tipo        | Architectural Rule             |
| Severidade  | **High**                       |
| Scope       | UI / Forms / UX / SSR          |
| Violação    | **Review Blocker**             |

---

## 🧪 COMO ESSA RULE É APLICADA NA PRÁTICA

### Em Code Review

**Checklist obrigatório:**

- [ ] PR usa Combobox com <20 opções?
- [ ] PR usa Combobox em formulário SSR-crítico?
- [ ] PR documenta a escolha entre Select vs Combobox?

**Se falhar → PR NÃO APROVA**

### Em Auditoria

Você pode fazer queries como:
```sql
SELECT file, component
FROM component_usage
WHERE component = 'Combobox'
  AND list_size < 10;
```

👉 Isso é nível **plataforma**, não app.

---

## 🧱 ESTRUTURA DE DOCUMENTAÇÃO
```
docs/canon/
├── rules/
│   └── canon-rule-12-select-vs-combobox.md     ← Norma permanente
└── records/
    └── canon-record-12-architectural-decision.md ← Histórico da decisão
```

- **Rule:** norma permanente
- **Record:** histórico da decisão

---

## 🏁 VEREDITO FINAL

- ✅ É a **Canon Rule #12**
- ✅ É **arquitetural**, não estilística
- ✅ Ela **bloqueia PR errado**
- ✅ Ela protege **SSR, DX, UX e performance**
- ✅ Ela evita **"component creep"** no futuro

---

## Referências

- Canon Record #12: `/docs/canon/records/canon-record-12-architectural-decision.md`
- Implementação: `/packages-rust/rs-design/src/ui/combobox/README.md`
- Discussão original: `/docs/canon/12-select-vs-combobox.md`
