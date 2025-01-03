pub struct ScrollBarInfo {
    pub scroll_position: usize,
    pub scroll_length: u16
}

impl ScrollBarInfo {
    pub fn new() -> Self {
        Self {
            scroll_position: 0,
            scroll_length: 0
        }
    }
    pub fn clear(&mut self) {
        self.scroll_position = 0;
        self.scroll_length = 0;
    }
}