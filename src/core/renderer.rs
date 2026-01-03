use std::sync::Arc;

use glow::HasContext;

pub struct Renderer {
    gl: Arc<glow::Context>,
}

impl Renderer {
    pub fn new(gl: Arc<glow::Context>) -> Self {
        Self { gl }
    }

    pub fn render_color(&mut self, red: f32, green: f32, blue: f32) {
        unsafe {
            self.gl.clear_color(red, green, blue, 1.0);
            self.gl.clear(glow::COLOR_BUFFER_BIT);
        }
    }

    pub fn render_scene_pass(&mut self) {
        unsafe {
            self.gl.clear_color(0.2, 0.22, 0.26, 1.0);
            self.gl
                .clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
        }
    }
}
