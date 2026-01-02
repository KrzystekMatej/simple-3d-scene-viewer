use crate::core::{
    GlWindow,
    WindowFactory,
    WindowHandler,
};
use serde::Deserialize;
use winit::{
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::WindowAttributes,
    dpi::LogicalSize,
};


#[derive(Deserialize)]
pub struct MainWindowConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
}

const DEFAULT_CONFIG: &str = include_str!("main_window_config.toml");

pub struct MainWindowFactory;

impl MainWindowFactory {
    pub fn new() -> Self {
        Self
    }
}

impl WindowFactory for MainWindowFactory {
    fn create(&mut self, event_loop: &ActiveEventLoop) -> Box<dyn WindowHandler> {
        let config: MainWindowConfig =
            toml::from_str(DEFAULT_CONFIG).expect("Failed to load default main window configuration");

        let attributes = WindowAttributes::default()
            .with_title(config.title)
            .with_inner_size(LogicalSize::new(config.width as f64, config.height as f64));

        let window = GlWindow::new(event_loop, attributes);
        Box::new(MainWindow { gl_window: window })
    }
}

pub struct MainWindow {
    gl_window: GlWindow,
}

impl WindowHandler for MainWindow {
    fn id(&self) -> winit::window::WindowId {
        self.gl_window.raw().id()
    }

    fn request_redraw(&self) {
        self.gl_window.raw().request_redraw();
    }

    fn handle_event(&mut self, event_loop: &winit::event_loop::ActiveEventLoop, event: &winit::event::WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                self.gl_window.resize(*size);
                self.gl_window.raw().request_redraw();
            }
            WindowEvent::ScaleFactorChanged { .. } => {
                let size = self.gl_window.raw().inner_size();
                self.gl_window.resize(size);
                self.gl_window.raw().request_redraw();
            }
            WindowEvent::RedrawRequested => {
                self.gl_window.render_clear(0.1, 0.12, 0.16, 1.0);
            }
            _ => {}
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_parses() {
        let cfg: MainWindowConfig = toml::from_str(DEFAULT_CONFIG).unwrap();
        assert!(!cfg.title.is_empty());
        assert!(cfg.width > 0);
        assert!(cfg.height > 0);
    }
}
