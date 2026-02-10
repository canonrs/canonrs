Dá para fazer — e dá para fazer de forma canônica em Rust, sem mentir sobre SSR/CSR e sem esconder a realidade.
Mas precisa ficar claro o que exatamente isso resolve (e o que não resolve).

Vou direto.

O que você quer resolver (formulado corretamente)

Você não quer eliminar CSR vs SSR.
Você quer eliminar o atrito manual de:

imports diferentes

use canonrs::ui_interactive::...

#[cfg(feature = "hydrate")] espalhado

código feio nas páginas

👉 Ou seja: reduzir fricção, não negar a física.

Isso é legítimo.

Solução CANÔNICA em Rust (e que faz sentido no CanonRS)
Ideia central

Criar um crate gerado (ou módulo gerado) no app, não no framework, com:

exports já “prontos”

fallback SSR explícito

CSR real encapsulado

O app importa um módulo único, e não pensa mais nisso.

Arquitetura proposta (clean)
1️⃣ CanonRS fornece a verdade (inalterado)

Nada muda aqui:

canonrs::ui → SSR-safe

canonrs::ui_interactive → CSR-only

Sem gambiarras. Sem auto-import mágico.

2️⃣ O CLI gera um módulo no APP

Durante canonrs dev / build, gerar algo como:

/src/.canonrs/generated/interactive.rs

Esse arquivo é do app, não do framework.

3️⃣ Conteúdo gerado (exemplo real)
//! AUTO-GENERATED — DO NOT EDIT
use leptos::prelude::\*;

/// Command component (CSR-only)
pub mod command {
use super::\*;

    #[cfg(feature = "hydrate")]
    pub use canonrs::ui_interactive::command::examples::BasicExample;

    #[cfg(not(feature = "hydrate"))]
    #[component]
    pub fn BasicExample() -> impl IntoView {
        view! {
            <div class="canonrs-csr-only">
                "Command (CSR only component)"
            </div>
        }
    }

}

4️⃣ Uso na página (limpo, sem cfg)
use crate::canonrs::interactive::command::BasicExample;

view! {
<BasicExample />
}

📌 Sem #[cfg] na página
📌 Sem import de ui_interactive direto
📌 Sem mentira sobre SSR

O que isso REALMENTE resolve
✅ Resolve

Páginas limpas

DX excelente

Import único e estável

CSR-only explícito, mas encapsulado

Framework continua honesto

Build-time seguro

❌ Não resolve (e nunca vai)

SSR executar componentes CSR-only

CSR “virar” SSR magicamente

O fato de que fallback SSR existe

Mas isso ninguém resolve, nem React, nem Vue, nem Svelte — eles só escondem.

Onde esse código deve existir

⚠️ Regra importante

Esse módulo NÃO deve viver dentro do canonrs.

Motivo:

depende de features do app

depende do target (ssr/hydrate)

depende das escolhas do produto

📍 Lugar correto:
products/_/src/.canonrs/generated/_

Como gerar isso (tecnicamente)

Você já tem tudo:

canonrs-cli

detecção de features

workspace generator

conhecimento dos componentes CSR-only

Passos no CLI:

Mapear componentes ui_interactive

Gerar wrappers com fallback SSR

Escrever arquivos Rust

Regerar quando framework atualizar

Nenhuma macro, nenhum proc-macro, nenhum UB.

Veredito arquitetural

✔️ Sua ideia é boa
✔️ É idiomática em Rust
✔️ Não viola SSR
✔️ Não mente
✔️ Melhora DX drasticamente
✔️ Mantém CanonRS enterprise

A regra é simples:

O framework não esconde a verdade.
O app escolhe como lidar com ela.
