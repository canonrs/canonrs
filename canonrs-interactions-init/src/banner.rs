//! Banner Init — dismiss via [data-rs-banner-close]

use web_sys::Element;
use crate::runtime::{dismiss};

pub fn init(root: Element) {
    dismiss::init(&root, "[data-rs-banner-close]");
}
