use macroquad::prelude::*;

pub struct View {
    pub x: f32,
    pub y: f32,
    pub scale: f32,
}

impl View {
    pub fn new() -> Self {
        View {
            scale: 1.0,
            x: 0.0,
            y: 0.0,
        }
    }
    fn to_camera(&self) -> Camera2D {
        Camera2D {
            rotation: 0.0,
            zoom: Vec2 { x: self.scale / screen_width(), y: self.scale / screen_height() },
            target: Vec2 { x: self.x, y: self.y },
            offset: Vec2::ZERO,
            render_target: None,
            viewport: None,
        }
    }

    pub fn update(&mut self) {
        if is_mouse_button_down(MouseButton::Right) {
            let delta = mouse_delta_position();
            self.x += delta.x * screen_width() / self.scale;
            self.y += delta.y * screen_height() / self.scale;
        }
        self.scale += mouse_wheel().1 / 1000.0;
        self.scale = self.scale.clamp(0.5, 4.0);
        set_camera(&self.to_camera());
    }
}
