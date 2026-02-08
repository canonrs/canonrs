//! Sheet Component - Drawer lateral (variação de Dialog)

use leptos::prelude::*;
use crate::primitives::drawer::*;
use crate::primitives::dialog::DialogPrimitive;

#[derive(Clone, Copy, PartialEq)]
pub enum SheetSide {
    Left,
    Right,
    Top,
    Bottom,
}

impl SheetSide {
    pub fn as_str(&self) -> &str {
        match self {
            SheetSide::Left => "left",
            SheetSide::Right => "right",
            SheetSide::Top => "top",
            SheetSide::Bottom => "bottom",
        }
    }
}
 
#[component]
pub fn Sheet(
    children: ChildrenFn,
    #[prop(into)] id: String,
    #[prop(default = SheetSide::Right)] side: SheetSide,
) -> impl IntoView {
    let class = format!("sheet sheet--{}", side.as_str());
    
    view! {
        <DialogPrimitive
            id=id
            class=class
        >
            {children()}
        </DialogPrimitive>
    }
}
