pub mod component_meta {
    #![allow(dead_code, unused_imports)]
    include!(concat!(env!("OUT_DIR"), "/generated/component_meta.rs"));
}
pub mod block_meta {
    #![allow(dead_code, unused_imports)]
    include!(concat!(env!("OUT_DIR"), "/generated/block_meta.rs"));
}
pub mod block_definitions {
    #![allow(dead_code, unused_imports)]
    include!(concat!(env!("OUT_DIR"), "/generated/block_definitions.rs"));
}
pub mod layout_definitions {
    #![allow(dead_code, unused_imports)]
    include!(concat!(env!("OUT_DIR"), "/generated/layout_definitions.rs"));
}
pub mod catalog {
    #![allow(dead_code, unused_imports)]
    include!(concat!(env!("OUT_DIR"), "/generated/catalog.rs"));
}
pub mod component_definitions {
    #![allow(dead_code, unused_imports)]
    include!(concat!(env!("OUT_DIR"), "/generated/component_definitions.rs"));
}
pub use component_meta::*;
pub use block_meta::*;

pub mod showcase {
    pub const SHOWCASE_JSON: &str = include_str!("showcase.json");
}
pub use showcase::SHOWCASE_JSON;
