use crate::core::{
    asset_manager::AssetManager, gl_window::GlWindow, renderer::Renderer, scene::Scene, time::Time,
};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow},
    window::{WindowAttributes, WindowId},
};

pub struct AppContext<'a> {
    pub time: &'a Time,
    pub scene: &'a mut Scene,
    pub assets: &'a mut AssetManager,
    pub renderer: &'a mut Renderer,
    pub window: &'a mut GlWindow,
}

pub trait AppClient {
    fn on_window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        ctx: &mut AppContext,
        event: &WindowEvent,
    );
    fn render(&mut self, ctx: &mut AppContext);
    fn shutdown(&mut self, ctx: &mut AppContext);
}

pub trait AppFactory {
    fn window_attributes(&mut self) -> WindowAttributes;
    fn create_client(&mut self, ctx: &GlWindow) -> anyhow::Result<Box<dyn AppClient>>;
}

pub struct Application {
    time: Time,
    scene: Scene,
    assets: AssetManager,
    renderer: Option<Renderer>,
    main_window: Option<GlWindow>,
    app_factory: Box<dyn AppFactory>,
    app_client: Option<Box<dyn AppClient>>,
}

impl Application {
    pub fn new(app_factory: Box<dyn AppFactory>) -> Self {
        Self {
            time: Time::new(),
            scene: Scene::new(),
            assets: AssetManager::new(),
            renderer: None,
            main_window: None,
            app_factory,
            app_client: None,
        }
    }

    fn with_ctx<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut dyn AppClient, &mut AppContext),
    {
        let Some(client) = self.app_client.as_mut() else {
            return;
        };
        let Some(window) = self.main_window.as_mut() else {
            return;
        };
        let Some(renderer) = self.renderer.as_mut() else {
            return;
        };

        let mut ctx = AppContext {
            time: &self.time,
            scene: &mut self.scene,
            assets: &mut self.assets,
            renderer,
            window,
        };

        f(client.as_mut(), &mut ctx);
    }
}

impl ApplicationHandler for Application {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        event_loop.set_control_flow(ControlFlow::Poll);

        if self.main_window.is_none() {
            let attributes = self.app_factory.window_attributes();
            self.main_window = Some(GlWindow::new(event_loop, attributes));
        }

        if let Some(window) = &self.main_window {
            if self.renderer.is_none() {
                self.renderer = Some(Renderer::new(window.gl_cloned()));
            }

            if self.app_client.is_none() {
                match self.app_factory.create_client(window) {
                    Ok(client) => {
                        self.app_client = Some(client);
                    }
                    Err(err) => {
                        log::error!("App client creation failed: {:#}", err);
                    }
                }
            }
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        self.with_ctx(|client, ctx| {
            if id != ctx.window.id() {
                return;
            }

            let mut do_render = false;

            match &event {
                WindowEvent::Resized(size) => {
                    ctx.window.resize(*size);
                    ctx.window.request_redraw();
                }
                WindowEvent::ScaleFactorChanged { .. } => {
                    let size = ctx.window.inner_size();
                    ctx.window.resize(size);
                    ctx.window.request_redraw();
                }
                WindowEvent::RedrawRequested => {
                    do_render = true;
                }
                _ => {}
            }

            client.on_window_event(event_loop, ctx, &event);

            if do_render {
                client.render(ctx);
                ctx.window.swap_buffers();
            }
        });
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        self.time.update();
        self.scene.update();

        if let Some(w) = self.main_window.as_ref() {
            w.request_redraw();
        }
    }

    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        self.with_ctx(|client, ctx| {
            client.shutdown(ctx);
        });

        self.app_client = None;
    }
}
