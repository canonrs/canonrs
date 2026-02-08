//! @canon-level: strict
//! @canon-owner: blocks-team

use leptos::prelude::*;
use crate::ui::tabs::{Tabs, TabsList, TabsTrigger, TabsContent};
use crate::ui::code_block::CodeBlock;

#[component]
pub fn CodeSectionBlock(
    #[prop(into)] primitive_code: String,
    #[prop(into)] primitive_html: String,
    #[prop(into)] primitive_responsibility: String,
    #[prop(into)] ui_code: String,
    #[prop(into)] ui_composition: String,
    #[prop(into)] usage_code: String,
    #[prop(into)] usage_cases: String,
    #[prop(into)] contract_props: String,
    #[prop(into)] contract_guarantees: String,
) -> impl IntoView {
    let primitive_code = StoredValue::new(primitive_code);
    let primitive_html = StoredValue::new(primitive_html);
    let primitive_responsibility = StoredValue::new(primitive_responsibility);
    let ui_code = StoredValue::new(ui_code);
    let ui_composition = StoredValue::new(ui_composition);
    let usage_code = StoredValue::new(usage_code);
    let usage_cases = StoredValue::new(usage_cases);
    let contract_props = StoredValue::new(contract_props);
    let contract_guarantees = StoredValue::new(contract_guarantees);

    view! {
        <section data-code-section="" class="canon-code-section">
            <h3 class="canon-code-section-title">"Code Examples"</h3>

            <Tabs>
                <TabsList>
                    <TabsTrigger value="primitive">
                        "Primitive Layer"
                    </TabsTrigger>
                    <TabsTrigger value="ui">
                        "UI Layer"
                    </TabsTrigger>
                    <TabsTrigger value="usage">
                        "Application Usage"
                    </TabsTrigger>
                    <TabsTrigger value="contract">
                        "Component Contract"
                    </TabsTrigger>
                </TabsList>

                <TabsContent value="primitive">
                    <div class="canon-code-tab-content">
                        <h4>"Rust Code (Primitive)"</h4>
                        <CodeBlock
                            code={primitive_code.get_value()}
                            language="rust"
                            show_copy={true}
                        />

                        <h4>"HTML Output (SSR)"</h4>
                        <CodeBlock
                            code={primitive_html.get_value()}
                            language="html"
                            show_copy={true}
                        />

                        <div class="canon-code-responsibility">
                            <h5>"Responsibility"</h5>
                            <div inner_html=primitive_responsibility.get_value()></div>
                        </div>
                    </div>
                </TabsContent>

                <TabsContent value="ui">
                    <div class="canon-code-tab-content">
                        <h4>"Rust Code (UI Layer)"</h4>
                        <CodeBlock
                            code={ui_code.get_value()}
                            language="rust"
                            show_copy={true}
                        />

                        <div class="canon-code-composition">
                            <h5>"Composition & Defaults"</h5>
                            <div inner_html=ui_composition.get_value()></div>
                        </div>
                    </div>
                </TabsContent>

                <TabsContent value="usage">
                    <div class="canon-code-tab-content">
                        <h4>"Real-World Usage"</h4>
                        <CodeBlock
                            code={usage_code.get_value()}
                            language="rust"
                            show_copy={true}
                        />

                        <div class="canon-code-cases">
                            <h5>"Common Patterns"</h5>
                            <div inner_html=usage_cases.get_value()></div>
                        </div>
                    </div>
                </TabsContent>

                <TabsContent value="contract">
                    <div class="canon-code-tab-content">
                        <div class="canon-code-contract">
                            <h4>"Props Summary"</h4>
                            <div inner_html=contract_props.get_value()></div>

                            <h4>"Guarantees"</h4>
                            <div inner_html=contract_guarantees.get_value()></div>
                        </div>
                    </div>
                </TabsContent>
            </Tabs>
        </section>
    }
}
