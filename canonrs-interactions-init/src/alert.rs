//! Alert Init — dismiss via [data-rs-alert-close]

use web_sys::Element;
use crate::runtime::{dismiss};

pub fn init(root: Element) {
    dismiss::init(&root, "[data-rs-alert-close]");
}
