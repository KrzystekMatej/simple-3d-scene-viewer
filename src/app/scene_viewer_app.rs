use crate::app::config::AppConfig;
use crate::core::{AppClient, AppContext, AppFactory, GlWindow};

use crate::app::left_panel::LeftPanel;
use crate::app::scene_display::SceneDisplay;
use anyhow::Context;
use winit::{
    dpi::LogicalSize, event::WindowEvent, event_loop::ActiveEventLoop, window::WindowAttributes,
};

pub struct SceneViewerAppFactory {
    config: AppConfig,
}

pub struct SceneViewerApp {
    egui_ctx: egui::Context,
    egui_state: egui_winit::State,
    painter: egui_glow::Painter,

    left_panel: LeftPanel,
    scene_display: SceneDisplay,
}

impl SceneViewerAppFactory {
    pub fn new(config: AppConfig) -> Self {
        Self { config }
    }
}

impl AppFactory for SceneViewerAppFactory {
    fn create_client(&mut self, window: &GlWindow) -> anyhow::Result<Box<dyn AppClient>> {
        let egui_ctx = egui::Context::default();

        let egui_state = egui_winit::State::new(
            egui_ctx.clone(),
            egui_ctx.viewport_id(),
            window.raw_handle(),
            None,
            None,
            None,
        );

        let mut painter = egui_glow::Painter::new(window.gl_cloned(), "", None, false)
            .map_err(|e| anyhow::anyhow!("{e:?}"))
            .context("failed to create egui_glow::Painter")?;

        let scene_display = SceneDisplay::new(&mut painter, window.gl_cloned())
            .context("failed to create SceneDisplay")?;

        Ok(Box::new(SceneViewerApp {
            egui_ctx,
            egui_state,
            painter,
            left_panel: LeftPanel::new(),
            scene_display,
        }))
    }

    fn window_attributes(&mut self) -> WindowAttributes {
        let config = self.config.clone();

        WindowAttributes::default()
            .with_title(config.title)
            .with_min_inner_size(LogicalSize::new(
                config.min_width as f64,
                config.min_height as f64,
            ))
            .with_inner_size(LogicalSize::new(config.width as f64, config.height as f64))
    }
}

impl AppClient for SceneViewerApp {
    fn on_window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        ctx: &mut AppContext,
        event: &WindowEvent,
    ) {
        let response = self
            .egui_state
            .on_window_event(ctx.window.raw_handle(), event);

        if response.repaint {
            ctx.window.request_redraw();
        }

        if matches!(event, WindowEvent::CloseRequested) {
            event_loop.exit();
        }
    }

    fn render(&mut self, ctx: &mut AppContext) {
        let window = ctx.window.raw_handle();
        let raw_input = self.egui_state.take_egui_input(window);

        let full_output = self.egui_ctx.run(raw_input, |egui_ctx| {
            self.left_panel.ui(egui_ctx);
            self.scene_display.ui(egui_ctx);
        });

        self.scene_display.render_to_target(ctx.renderer);

        self.egui_state
            .handle_platform_output(window, full_output.platform_output);

        let window_size = ctx.window.inner_size();
        let clipped = self
            .egui_ctx
            .tessellate(full_output.shapes, full_output.pixels_per_point);

        self.painter.paint_and_update_textures(
            [window_size.width, window_size.height],
            full_output.pixels_per_point,
            &clipped,
            &full_output.textures_delta,
        );
    }

    fn shutdown(&mut self, _ctx: &mut AppContext) {
        self.scene_display.shutdown(&mut self.painter);
        self.painter.destroy();
    }
}
