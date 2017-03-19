#[derive(Clone, Copy, Debug)]
pub struct Spec {
    pub size: (u32, u32),
    pub font: f32,
    pub block: u8,
}

impl Spec {
    pub fn new(w: u32, h: u32) -> Self {
        Spec {
            size: (w, h),
            font: 0.5,
            block: 4,
        }
    }

    pub fn size(&self, w: u32, h: u32) -> Self {
        Spec { size: (w, h), ..*self }
    }

    pub fn font(&self, font: f32) -> Self {
        Spec { font: font, ..*self }
    }

    pub fn block(&self, block: u8) -> Self {
        Spec { block: block, ..*self }
    }
}
