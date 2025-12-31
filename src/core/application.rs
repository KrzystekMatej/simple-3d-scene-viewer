use crate::core::Time;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoopWindowTarget},
    window::Window,
};

pub struct Application {
    time: Time,
    main_window: Window,
}

impl Application {
    pub fn new(main_window: Window) -> Self {
        Application {
            time: Time::new(),
            main_window,
        }
    }

    pub fn handle_event(
        &mut self,
        event: winit::event::Event<()>,
        target: &EventLoopWindowTarget<()>,
    ) {
        target.set_control_flow(ControlFlow::Wait);

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    target.exit();
                }
                WindowEvent::Resized(_) => {
                    self.main_window.request_redraw();
                }
                WindowEvent::RedrawRequested => {}
                _ => {}
            },
            Event::AboutToWait => {
                self.time.update();
                self.main_window.request_redraw();
            }
            _ => {}
        }
    }
}
