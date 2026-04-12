use std::collections::HashSet;
use std::cell::RefCell;
use web_sys::Element;

thread_local! {
    static INITED: RefCell<HashSet<String>> = RefCell::new(HashSet::new());
}

pub fn should_init(el: &Element) -> bool {
    if let Some(uid) = el.get_attribute("data-rs-uid") {
        INITED.with(|set| {
            let mut s = set.borrow_mut();
            if s.contains(&uid) {
                false
            } else {
                s.insert(uid);
                true
            }
        })
    } else {
        true
    }
}
