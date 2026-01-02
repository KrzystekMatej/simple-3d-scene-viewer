mod app;
mod core;
use app::MainWindowFactory;
use core::Application;
use winit::event_loop::EventLoop;

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
    let event_loop = EventLoop::new().expect("Failed to create event loop.");
    let mut app = Application::new(Box::new(MainWindowFactory::new()));
    event_loop.run_app(&mut app).unwrap();
}
