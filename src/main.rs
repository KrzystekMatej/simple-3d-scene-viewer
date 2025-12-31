mod core;
use core::Application;
use winit::{
    event_loop::{EventLoop},
    window::WindowBuilder,
};

#[cfg(debug_assertions)]
fn init_logger() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
}

#[cfg(not(debug_assertions))]
fn init_logger() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
}


fn main() {
    init_logger();

    let event_loop = EventLoop::new().unwrap();
    
    let window = WindowBuilder::new()
        .with_title("Minimal winit window")
        .with_inner_size(winit::dpi::LogicalSize::new(800.0, 600.0))
        .build(&event_loop)
        .unwrap();

    let mut app = Application::new(window);
    
    let _ = event_loop.run(move |event, target| {
        app.handle_event(event, target);
    });
}
