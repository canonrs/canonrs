//! Table of Contents types - shared entre SSR e utilities

#[derive(Clone, Debug)]
pub struct TocItem {
    pub id: String,
    pub text: String,
    pub level: u8,
}

impl TocItem {
    pub fn new(id: String, text: String, level: u8) -> Self {
        Self { id, text, level }
    }
}
