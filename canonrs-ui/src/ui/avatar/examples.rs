use leptos::prelude::*;
use super::avatar_ui::*;

pub fn basic_example() -> impl IntoView {
    view! {
        <div style="display: flex; gap: 2rem; align-items: center;">
            <Avatar size=AvatarSize::Lg>
                <AvatarFallback>"JD"</AvatarFallback>
            </Avatar>
            <Avatar size=AvatarSize::Md>
                <AvatarFallback>"AS"</AvatarFallback>
            </Avatar>
            <Avatar size=AvatarSize::Sm>
                <AvatarFallback>"MK"</AvatarFallback>
            </Avatar>
        </div>
    }
}
