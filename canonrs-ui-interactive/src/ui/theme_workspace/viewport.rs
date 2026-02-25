#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Viewport {
    pub width: u32,
    pub height: u32,
}

impl Viewport {
    pub fn desktop() -> Self { Self { width: 1440, height: 900 } }
    pub fn tablet()  -> Self { Self { width: 768,  height: 1024 } }
    pub fn mobile()  -> Self { Self { width: 390,  height: 844 } }
}
