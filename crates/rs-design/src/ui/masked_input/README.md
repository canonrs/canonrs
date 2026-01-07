# MaskedInput

Componente de input com máscara formatada (CPF, CNPJ, telefone, etc).

## Type Classification
- **Type:** 2 (Hybrid - SSR-safe com progressive enhancement)
- **SSR:** Safe (máscara determinística aplicada no client)
- **Bundle:** ~0.5KB (sem libs externas)

## Tokens Canônicos Aplicados
- `field.height` → 2.5rem
- `field.padding` → 0.5rem  
- `field.border` → 1px
- `font.size.md` → 1rem
- `font.weight.regular` → 400
- `radius.md` → 0.375rem
- `color.bg.surface` → bg-surface
- `color.fg.default` → text-fg-default
- `color.fg.muted` → placeholder
- `motion.duration.normal` → 300ms
- `motion.ease.standard` → cubic-bezier
- `state.focus.ring` → 2px solid
- `state.hover.opacity` → 0.8
- `state.disabled.opacity` → 0.5

## Tokens Família C Aplicados
- `field.border` → border width
- `field.height` → altura do campo
- `field.padding` → padding interno
- `field.placeholder` → cor do placeholder
- `validation.error` → variante error
- `validation.success` → variante success
- `validation.warning` → variante warning

## Uso
```rust
use rs_design::MaskedInput;

#[component]
fn MyForm() -> impl IntoView {
    let cpf = RwSignal::new(String::new());

    view! {
        // CPF
        <MaskedInput
            value=cpf.into()
            on_change=move |val| cpf.set(val)
            mask_type=MaskType::CPF
            variant=MaskedInputVariant::Default
            placeholder="000.000.000-00"
        />

        // Telefone com validação
        <MaskedInput
            value=phone.into()
            on_change=move |val| phone.set(val)
            mask_type=MaskType::Phone
            variant=if phone_valid() { 
                MaskedInputVariant::Success 
            } else { 
                MaskedInputVariant::Error 
            }
        />

        // Máscara customizada
        <MaskedInput
            value=custom.into()
            on_change=move |val| custom.set(val)
            mask_type=MaskType::Custom("AA-9999".to_string())
        />
    }
}
```

## Máscaras Disponíveis

- `MaskType::CPF` → 999.999.999-99
- `MaskType::CNPJ` → 99.999.999/9999-99
- `MaskType::Phone` → (99) 99999-9999
- `MaskType::CEP` → 99999-999
- `MaskType::Date` → 99/99/9999
- `MaskType::CreditCard` → 9999 9999 9999 9999
- `MaskType::Custom(pattern)` → padrão customizado

## Padrão de Máscara

- `9` → dígito numérico
- Outros caracteres → literais (inseridos automaticamente)

## Variantes

- `Default` → border padrão
- `Success` → border verde (validação ok)
- `Error` → border vermelho (erro)
- `Warning` → border amarelo (aviso)

## SSR Safety

✅ **SSR-Safe** porque:
- Máscara é determinística (baseada apenas em posição)
- Não usa APIs de cursor/seleção
- Render inicial = input normal
- Máscara aplicada progressivamente no client

## Canon Rule #13

Veja: `/docs/canon/rules/canon-rule-13-input-vs-maskedinput.md`

**MaskedInput NÃO substitui Input. É uma especialização.**
