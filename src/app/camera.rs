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

    pub fn screen_to_world_x(&self, x: f64) -> f64 {
        self.left * (1.0 - x) + self.right * x
    }

    pub fn screen_to_world_y(&self, y: f64) -> f64 {
        // y coordinates are flipped
        self.bottom * y + self.top * (1.0 - y)
    }

    pub fn screen_to_world(&self, x: f64, y: f64) -> (f64, f64) {
        (self.screen_to_world_x(x), self.screen_to_world_y(y))
    }

    pub fn world_to_screen_x(&self, x: f64) -> f64 {
        (x - self.left) / (self.right - self.left)
    }

    pub fn world_to_screen_y(&self, y: f64) -> f64 {
        // y coordinates are flipped
        (y - self.top) / (self.bottom - self.top)
    }

    pub fn world_to_screen(&self, x: f64, y: f64) -> (f64, f64) {
        (self.world_to_screen_x(x), self.world_to_screen_y(y))
    }

    pub fn translate(&mut self, delta_x: f64, delta_y: f64) {
        let offset_x = -delta_x * (self.right - self.left);
        // y coordinates are flipped
        let offset_y = delta_y * (self.top - self.bottom);
        self.left += offset_x;
        self.right += offset_x;
        self.bottom += offset_y;
        self.top += offset_y;
    }

    pub fn zoom(&mut self, pos_x: f64, pos_y: f64, scroll: f64) {
        let factor = (-scroll / 100.0).exp();
        let (pos_x, pos_y) = self.screen_to_world(pos_x, pos_y);

        self.left = pos_x + factor * (self.left - pos_x);
        self.right = pos_x + factor * (self.right - pos_x);

        self.bottom = pos_y + factor * (self.bottom - pos_y);
        self.top = pos_y + factor * (self.top - pos_y);
    }
}
