use leptos::prelude::*;
use super::{Kbd, KbdSize, KbdVariant, KbdGroup, KbdSeparator};

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <div style="display: flex; flex-direction: column; gap: 2rem;">
            // Single keys
            <div>
                <h4>"Single Keys"</h4>
                <div style="display: flex; gap: 1rem; align-items: center;">
                    <Kbd>"Ctrl"</Kbd>
                    <Kbd>"Shift"</Kbd>
                    <Kbd>"Alt"</Kbd>
                    <Kbd>"Enter"</Kbd>
                </div>
            </div>

            // Shortcuts with group
            <div>
                <h4>"Shortcuts"</h4>
                <div style="display: flex; gap: 1rem; align-items: center;">
                    <KbdGroup>
                        <Kbd>"Ctrl"</Kbd>
                        <KbdSeparator />
                        <Kbd>"C"</Kbd>
                    </KbdGroup>
                    <KbdGroup>
                        <Kbd>"Ctrl"</Kbd>
                        <KbdSeparator />
                        <Kbd>"Shift"</Kbd>
                        <KbdSeparator />
                        <Kbd>"P"</Kbd>
                    </KbdGroup>
                </div>
            </div>

            // Sizes
            <div>
                <h4>"Sizes"</h4>
                <div style="display: flex; gap: 1rem; align-items: center;">
                    <Kbd size=KbdSize::Sm>"⌘"</Kbd>
                    <Kbd size=KbdSize::Md>"⌘"</Kbd>
                </div>
            </div>

            // Variants
            <div>
                <h4>"Variants"</h4>
                <div style="display: flex; gap: 1rem; align-items: center;">
                    <Kbd variant=KbdVariant::Default>"Esc"</Kbd>
                    <Kbd variant=KbdVariant::Muted>"Tab"</Kbd>
                </div>
            </div>

            // Combined
            <div>
                <h4>"Combined"</h4>
                <div style="display: flex; gap: 1rem; align-items: center;">
                    <KbdGroup>
                        <Kbd size=KbdSize::Sm variant=KbdVariant::Muted>"⌘"</Kbd>
                        <KbdSeparator />
                        <Kbd size=KbdSize::Sm variant=KbdVariant::Muted>"K"</Kbd>
                    </KbdGroup>
                </div>
            </div>
        </div>
    }
}
