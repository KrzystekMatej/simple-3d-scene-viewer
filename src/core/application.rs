use crate::{core::Time, core::Window};
use serde::Deserialize;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow},
    window::WindowId,
};

#[derive(Deserialize)]
pub struct AppConfig {
    pub window_title: String,
    pub width: u32,
    pub height: u32,
}

const DEFAULT_CONFIG: &str = include_str!("app_config.toml");

pub struct Application {
    time: Time,
    main_window: Option<Window>,
}

impl Application {
    pub fn new() -> Self {
        Application {
            time: Time::new(),
            main_window: None,
        }
    }
}

impl ApplicationHandler for Application {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        event_loop.set_control_flow(ControlFlow::Poll);

        if self.main_window.is_none() {
            let config: AppConfig =
                toml::from_str(DEFAULT_CONFIG).expect("Failed to load default configuration");
            let window = Window::create_main_window(
                event_loop,
                &config.window_title,
                config.width,
                config.height,
            );
            self.main_window = Some(window);
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        let main_window = match &mut self.main_window {
            Some(w) => w,
            None => return,
        };

        if id != main_window.raw().id() {
            return;
        }

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                main_window.resize(size);
                main_window.raw().request_redraw();
            }
            WindowEvent::ScaleFactorChanged { .. } => {
                let size = main_window.raw().inner_size();
                main_window.resize(size);
                main_window.raw().request_redraw();
            }
            WindowEvent::RedrawRequested => {
                main_window.render_clear(0.1, 0.12, 0.16, 1.0);
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        self.time.update();
        if let Some(w) = &self.main_window {
            w.raw().request_redraw();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_parses() {
        let cfg: AppConfig = toml::from_str(DEFAULT_CONFIG).unwrap();
        assert!(!cfg.window_title.is_empty());
        assert!(cfg.width > 0);
        assert!(cfg.height > 0);
    }
}
