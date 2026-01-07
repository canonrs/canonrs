use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GridCols {
    Fixed(usize),           // 3 colunas fixas
    Responsive,             // 1 (mobile) → 2 (tablet) → 3 (desktop) → 4 (wide)
    Auto { min: usize },    // auto-fit com min width
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GridGap {
    None,
    Xs,  // 0.25rem
    Sm,  // 0.5rem
    Md,  // 1rem
    Lg,  // 1.5rem
    Xl,  // 2rem
}

impl GridGap {
    pub fn to_class(&self) -> &'static str {
        match self {
            GridGap::None => "gap-0",
            GridGap::Xs => "gap-[0.25rem]",
            GridGap::Sm => "gap-[0.5rem]",
            GridGap::Md => "gap-[1rem]",
            GridGap::Lg => "gap-[1.5rem]",
            GridGap::Xl => "gap-[2rem]",
        }
    }
}
