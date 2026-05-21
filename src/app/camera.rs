pub struct Camera {
    pub left: f64,
    pub right: f64,
    pub bottom: f64,
    pub top: f64,
}

impl Camera {
    pub fn home(aspect_ratio: f64) -> Self {
        Self {
            left: -5.0 * aspect_ratio,
            right: 5.0 * aspect_ratio,
            bottom: -5.0,
            top: 5.0,
        }
    }

    pub fn screen_to_world(&self, x: f64, y: f64) -> (f64, f64) {
        (
            self.left * (1.0 - x) + self.right * x,
            // y coordinates are flipped
            self.bottom * y + self.top * (1.0 - y),
        )
    }
}
