use crate::{core::Time};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow},
    window::WindowId,
};

pub trait WindowHandler {
    fn id(&self) -> WindowId;
    fn request_redraw(&self);
    fn handle_event(&mut self, event_loop: &ActiveEventLoop, event: &WindowEvent);
}

pub trait WindowFactory {
    fn create(&mut self, event_loop: &ActiveEventLoop) -> Box<dyn WindowHandler>;
}

pub struct Application {
    time: Time,
    main_window_factory: Box<dyn WindowFactory>,
    main_window: Option<Box<dyn WindowHandler>>,
}

impl Application {
    pub fn new(main_window_factory: Box<dyn WindowFactory>) -> Self {
        Application {
            time: Time::new(),
            main_window_factory: main_window_factory,
            main_window: None,
        }
    }
}

impl ApplicationHandler for Application {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        event_loop.set_control_flow(ControlFlow::Poll);
        if self.main_window.is_none() {
            self.main_window = Some(self.main_window_factory.create(event_loop));
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        let main_window = match &mut self.main_window {
            Some(w) => w,
            None => return,
        };

        if id != main_window.id() {
            return;
        }

        main_window.handle_event(event_loop, &event);
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        self.time.update();
        if let Some(w) = &self.main_window {
            w.request_redraw();
        }
    }
}