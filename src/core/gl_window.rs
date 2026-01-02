use glow::HasContext;
use glutin::{
    config::ConfigTemplateBuilder,
    context::{ContextApi, ContextAttributesBuilder, PossiblyCurrentContext, Version},
    display::GetGlDisplay,
    prelude::*,
    surface::{Surface, SurfaceAttributesBuilder, SwapInterval, WindowSurface},
};
use glutin_winit::DisplayBuilder;
use std::{ffi::CString, num::NonZeroU32};
use winit::{
    dpi::PhysicalSize,
    event_loop::ActiveEventLoop,
    raw_window_handle::HasWindowHandle,
    window::{Window as WinitWindow, WindowAttributes},
};

pub struct GlWindow {
    winit_window: WinitWindow,
    gl_surface: Surface<WindowSurface>,
    gl_context: PossiblyCurrentContext,
    gl: glow::Context,
}

impl GlWindow {
    pub fn new(event_loop: &ActiveEventLoop, attributes: WindowAttributes) -> Self {
        let template = ConfigTemplateBuilder::new()
            .with_alpha_size(8)
            .with_depth_size(24);

        let display_builder = DisplayBuilder::new().with_window_attributes(Some(attributes));

        let (window, gl_config) = display_builder
            .build(event_loop, template, |configs| configs.max_by_key(|c| c.num_samples()).unwrap())
            .unwrap();

        let winit_window = window.unwrap();

        let window_handle = winit_window.window_handle().unwrap();
        let raw_window_handle = window_handle.as_raw();

        let gl_display = gl_config.display().clone();

        let context_attributes = ContextAttributesBuilder::new()
            .with_context_api(ContextApi::OpenGl(Some(Version::new(4, 1))))
            .build(Some(raw_window_handle));

        let fallback_attributes = ContextAttributesBuilder::new()
            .with_context_api(ContextApi::OpenGl(Some(Version::new(3, 3))))
            .build(Some(raw_window_handle));

        let not_current = unsafe {
            gl_display
                .create_context(&gl_config, &context_attributes)
                .or_else(|_| gl_display.create_context(&gl_config, &fallback_attributes))
                .unwrap()
        };

        let size = winit_window.inner_size();
        let width = size.width.max(1);
        let height = size.height.max(1);

        let surface_attributes = SurfaceAttributesBuilder::<WindowSurface>::new().build(
            raw_window_handle,
            NonZeroU32::new(width).unwrap(),
            NonZeroU32::new(height).unwrap(),
        );

        let gl_surface = unsafe {
            gl_display
                .create_window_surface(&gl_config, &surface_attributes)
                .unwrap()
        };

        let gl_context = not_current.make_current(&gl_surface).unwrap();

        let _ = gl_surface.set_swap_interval(
            &gl_context,
            SwapInterval::Wait(NonZeroU32::new(1).unwrap()),
        );

        let gl = unsafe {
            glow::Context::from_loader_function(|name| {
                let c_name = CString::new(name).unwrap();
                gl_display.get_proc_address(&c_name) as *const _
            })
        };

        unsafe {
            gl.viewport(0, 0, width as i32, height as i32);
        }

        Self {
            winit_window,
            gl_surface,
            gl_context,
            gl,
        }
    }

    pub fn raw(&self) -> &WinitWindow {
        &self.winit_window
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        let width = size.width.max(1);
        let height = size.height.max(1);

        self.gl_surface.resize(
            &self.gl_context,
            NonZeroU32::new(width).unwrap(),
            NonZeroU32::new(height).unwrap(),
        );

        unsafe {
            self.gl.viewport(0, 0, width as i32, height as i32);
        }
    }

    pub fn render_clear(&mut self, r: f32, g: f32, b: f32, a: f32) {
        unsafe {
            self.gl.clear_color(r, g, b, a);
            self.gl.clear(glow::COLOR_BUFFER_BIT);
        }
        self.gl_surface.swap_buffers(&self.gl_context).unwrap();
    }
}