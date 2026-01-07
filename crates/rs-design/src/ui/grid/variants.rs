use super::types::GridCols;

impl GridCols {
    pub fn to_class(&self) -> String {
        match self {
            GridCols::Fixed(n) => format!("grid-cols-{}", n),
            GridCols::Responsive => {
                // 1 col mobile → 2 tablet → 3 desktop → 4 wide
                "grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4".to_string()
            }
            GridCols::Auto { min } => {
                format!("grid-cols-[repeat(auto-fit,minmax({}px,1fr))]", min)
            }
        }
    }
}
