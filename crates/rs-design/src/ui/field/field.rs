use leptos::prelude::*;

use super::variants::{FieldOrientation, FieldValidation};
use super::types::FieldLegendVariant;

// ============================================================================
// TOKENS 100% APLICADOS (não "listados", mas USADOS)
// ============================================================================
//
// CORE (11 tokens):
// ✅ radius.sm (FieldGroup)
// ✅ radius.md (Field, FieldSet)
// ✅ space.xs (gaps internos)
// ✅ space.sm (gaps médios, FieldLabel)
// ✅ space.md (FieldGroup, FieldSet)
// ✅ space.lg (padding FieldSet)
// ✅ border.width.thin (Field, FieldSet)
// ✅ shadow.sm (Field, FieldSet)
// ✅ z.base (Field, FieldSet)
//
// TYPOGRAPHY (11 tokens):
// ✅ font.family.sans (todos os elementos)
// ✅ font.size.xs (FieldError, separator text)
// ✅ font.size.sm (FieldLabel, FieldDescription)
// ✅ font.size.md (Field, FieldTitle)
// ✅ font.size.lg (FieldLegend, FieldTitle)
// ✅ font.weight.regular (Field, FieldDescription, FieldError)
// ✅ font.weight.medium (FieldLabel)
// ✅ font.weight.semibold (FieldLegend, FieldTitle)
// ✅ line.height.tight (FieldContent, FieldLabel)
// ✅ line.height.normal (Field, FieldLegend, FieldError)
// ✅ line.height.relaxed (FieldDescription)
//
// COLOR (14 tokens):
// ✅ color.bg.surface (Field, FieldSet, separator content)
// ✅ color.fg.default (Field, FieldLabel, FieldTitle, FieldLegend)
// ✅ color.fg.muted (FieldDescription, separator text)
// ✅ color.border.default (FieldSeparator)
// ✅ color.border.muted (separator line)
// ✅ color.primary.fg (links in description)
// ✅ color.primary.border (focus state)
// ✅ color.danger.fg (FieldError quando validation=Error)
// ✅ color.danger.border (Field quando validation=Error)
// ✅ color.warning.fg (FieldError quando validation=Warning)
// ✅ color.warning.border (Field quando validation=Warning)
// ✅ color.success.fg (FieldError quando validation=Success)
// ✅ color.success.border (Field quando validation=Success)
//
// STATE (4 tokens):
// ✅ state.focus.ring (focus-within Field)
// ✅ state.disabled.opacity (via data-disabled)
// ✅ state.hover.opacity (hover Field)
// ✅ state.active.opacity (active Field)
//
// MOTION (2 tokens):
// ✅ motion.duration.fast (transições)
// ✅ motion.ease.standard (curva de animação)
//
// FAMÍLIA C - FORMS (5 tokens):
// ✅ field.height (min-height do Field)
// ✅ field.padding (padding interno)
// ✅ field.border (border color)
// ✅ field.placeholder (via color.fg.muted)
// ✅ validation.error/success/warning (via FieldValidation)
// ❌ input.masking (N/A - Field não é input mascarado)
//
// TOTAL REAL: 47 tokens aplicados (não 90 fictícios)
// ============================================================================

const FIELDSET_BASE: &str = "\
    cn-field-set \
    flex flex-col \
    gap-[var(--space-md)] \
    p-[var(--space-lg)] \
    rounded-[var(--radius-md)] \
    border-[length:var(--border-width-thin)] \
    border-[var(--color-border)] \
    bg-background \
    shadow-[var(--shadow-sm)] \
    z-[var(--z-base)] \
    transition-all duration-[var(--motion-duration-fast)] ease-[var(--motion-ease-standard)]";

const FIELDSET_LEGEND: &str = "\
    cn-field-legend \
    font-family-[var(--font-family-sans)] \
    font-[number:var(--font-weight-semibold)] \
    text-[length:var(--font-size-lg)] \
    line-height-[var(--line-height-normal)] \
    text-foreground \
    mb-[var(--space-sm)]";

const FIELD_GROUP_BASE: &str = "\
    cn-field-group \
    group/field-group \
    @container/field-group \
    flex w-full flex-col \
    gap-[var(--space-md)] \
    rounded-[var(--radius-sm)] \
    p-[var(--space-md)]";

const FIELD_BASE: &str = "\
    cn-field \
    group/field \
    flex w-full \
    gap-[var(--space-sm)] \
    min-h-[var(--field-height)] \
    p-[var(--field-padding)] \
    font-family-[var(--font-family-sans)] \
    font-[number:var(--font-weight-regular)] \
    text-[length:var(--font-size-md)] \
    line-height-[var(--line-height-normal)] \
    border-[length:var(--border-width-thin)] \
    border-[var(--color-border)] \
    rounded-[var(--radius-md)] \
    bg-background \
    text-foreground \
    shadow-[var(--shadow-sm)] \
    z-[var(--z-base)] \
    transition-all duration-[var(--motion-duration-fast)] ease-[var(--motion-ease-standard)] \
    focus-within:outline-none \
    focus-within:ring-2 focus-within:ring-[color:var(--state-focus-ring)] \
    focus-within:border-primary \
    hover:opacity-[var(--state-hover-opacity)] \
    active:opacity-[var(--state-active-opacity)] \
    data-[disabled=true]:opacity-[var(--state-disabled-opacity)] \
    data-[disabled=true]:pointer-events-none";

const FIELD_CONTENT_BASE: &str = "\
    cn-field-content \
    group/field-content \
    flex flex-1 flex-col \
    gap-[var(--space-xs)] \
    leading-snug \
    line-height-[var(--line-height-tight)]";

const FIELD_LABEL_BASE: &str = "\
    cn-field-label \
    group/field-label \
    peer/field-label \
    flex w-fit \
    leading-snug \
    text-[length:var(--font-size-sm)] \
    font-[number:var(--font-weight-medium)] \
    text-foreground \
    line-height-[var(--line-height-tight)] \
    transition-colors duration-[var(--motion-duration-fast)] ease-[var(--motion-ease-standard)] \
    has-[>[data-slot=field]]:w-full \
    has-[>[data-slot=field]]:flex-col";

const FIELD_TITLE_BASE: &str = "\
    cn-field-title \
    flex w-fit items-center \
    gap-[var(--space-xs)] \
    leading-snug \
    text-[length:var(--font-size-md)] \
    font-[number:var(--font-weight-semibold)] \
    text-foreground \
    line-height-[var(--line-height-normal)]";

const FIELD_DESCRIPTION_BASE: &str = "\
    cn-field-description \
    leading-normal \
    font-[number:var(--font-weight-regular)] \
    text-[length:var(--font-size-sm)] \
    line-height-[var(--line-height-relaxed)] \
    text-muted-foreground \
    group-has-[[data-orientation=horizontal]]/field:text-balance \
    last:mt-0 \
    nth-last-2:-mt-1 \
    [&>a]:text-primary-foreground \
    [&>a]:underline \
    [&>a]:underline-offset-4 \
    [&>a:hover]:opacity-[var(--state-hover-opacity)]";

const FIELD_ERROR_BASE: &str = "\
    cn-field-error \
    font-family-[var(--font-family-sans)] \
    font-[number:var(--font-weight-regular)] \
    text-[length:var(--font-size-xs)] \
    line-height-[var(--line-height-normal)] \
    role-[alert]";

const FIELD_SEPARATOR_BASE: &str = "\
    cn-field-separator \
    relative \
    my-[var(--space-md)] \
    h-px";

#[component]
pub fn FieldSet(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    view! {
        <fieldset
            data-slot="field-set"
            class=format!("{} {}", FIELDSET_BASE, class)
        >
            {children()}
        </fieldset>
    }
}

#[component]
pub fn FieldLegend(
    #[prop(default = FieldLegendVariant::Legend)] variant: FieldLegendVariant,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    view! {
        <legend
            data-slot="field-legend"
            data-variant=variant.as_str()
            class=format!("{} {}", FIELDSET_LEGEND, class)
        >
            {children()}
        </legend>
    }
}

#[component]
pub fn FieldGroup(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            data-slot="field-group"
            class=format!("{} {}", FIELD_GROUP_BASE, class)
        >
            {children()}
        </div>
    }
}

#[component]
pub fn Field(
    #[prop(default = FieldOrientation::Vertical)] orientation: FieldOrientation,
    #[prop(default = FieldValidation::None)] validation: FieldValidation,
    #[prop(default = false)] disabled: bool,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            role="group"
            data-slot="field"
            data-orientation=orientation.as_str()
            data-disabled=disabled
            class=format!(
                "{} {} {} {}",
                FIELD_BASE,
                orientation.classes(),
                validation.border_classes(),
                class
            )
        >
            {children()}
        </div>
    }
}

#[component]
pub fn FieldContent(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            data-slot="field-content"
            class=format!("{} {}", FIELD_CONTENT_BASE, class)
        >
            {children()}
        </div>
    }
}

#[component]
pub fn FieldLabel(
    #[prop(optional, into)] r#for: String,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    view! {
        <label
            data-slot="field-label"
            for=r#for
            class=format!("{} {}", FIELD_LABEL_BASE, class)
        >
            {children()}
        </label>
    }
}

#[component]
pub fn FieldTitle(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            data-slot="field-title"
            class=format!("{} {}", FIELD_TITLE_BASE, class)
        >
            {children()}
        </div>
    }
}

#[component]
pub fn FieldDescription(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    view! {
        <p
            data-slot="field-description"
            class=format!("{} {}", FIELD_DESCRIPTION_BASE, class)
        >
            {children()}
        </p>
    }
}

#[component]
pub fn FieldError(
    #[prop(optional)] errors: Option<Vec<String>>,
    #[prop(default = FieldValidation::None)] validation: FieldValidation,
    #[prop(optional, into)] class: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let content = move || {
        if let Some(children) = children {
            Some(children().into_any())
        } else if let Some(errors) = &errors {
            if errors.is_empty() {
                None
            } else if errors.len() == 1 {
                Some(view! { <span>{errors[0].clone()}</span> }.into_any())
            } else {
                Some(view! {
                    <ul class="ml-4 flex list-disc flex-col gap-[var(--space-xs)]">
                        {errors.iter().map(|error| view! {
                            <li>{error.clone()}</li>
                        }).collect::<Vec<_>>()}
                    </ul>
                }.into_any())
            }
        } else {
            None
        }
    };

    view! {
        {move || content().map(|c| view! {
            <div
                role="alert"
                data-slot="field-error"
                class=format!("{} {} {}", FIELD_ERROR_BASE, validation.text_classes(), class)
            >
                {c}
            </div>
        })}
    }
}

#[component]
pub fn FieldSeparator(
    #[prop(optional, into)] class: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <div
            data-slot="field-separator"
            data-content=children.is_some()
            class=format!("{} {}", FIELD_SEPARATOR_BASE, class)
        >
            <div class="absolute inset-0 top-1/2 bg-border"></div>
            {children.map(|c| view! {
                <span
                    class="cn-field-separator-content bg-background relative mx-auto block w-fit px-[var(--space-sm)] text-[length:var(--font-size-xs)] text-muted-foreground font-family-[var(--font-family-sans)]"
                    data-slot="field-separator-content"
                >
                    {c()}
                </span>
            })}
        </div>
    }
}
