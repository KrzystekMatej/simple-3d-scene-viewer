use std::sync::Arc;

use crate::core::{RenderTarget, Renderer};
use anyhow::Context;
use egui_glow::Painter;
use winit::dpi::PhysicalSize;

pub struct SceneDisplay {
    render_target: RenderTarget,
    texture_id: egui::TextureId,
}

impl SceneDisplay {
    pub fn new(painter: &mut Painter, gl: Arc<glow::Context>) -> anyhow::Result<Self> {
        let render_target =
            RenderTarget::new(gl).context("failed to create RenderTarget for scene display")?;
        let texture_id = painter.register_native_texture(render_target.color_texture());
        Ok(Self {
            render_target,
            texture_id,
        })
    }

    pub fn ui(&mut self, egui_ctx: &egui::Context) {
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.inner_margin(egui::Margin::ZERO))
            .show(egui_ctx, |ui| {
                ui.spacing_mut().item_spacing = egui::Vec2::ZERO;

                let pixels_per_point = egui_ctx.pixels_per_point();
                let available_points = ui.available_size();

                let texture_pixels = self.render_target.size();
                let texture_points = egui::Vec2::new(
                    texture_pixels.width as f32 / pixels_per_point,
                    texture_pixels.height as f32 / pixels_per_point,
                );

                let image = egui::Image::from_texture(egui::load::SizedTexture::new(
                    self.texture_id,
                    texture_points,
                ))
                .fit_to_exact_size(available_points)
                .maintain_aspect_ratio(false);

                let response = ui.add_sized(available_points, image);
                let allocated_points = response.rect.size();

                let desired_pixels = PhysicalSize::new(
                    (allocated_points.x * pixels_per_point).max(1.0) as u32,
                    (allocated_points.y * pixels_per_point).max(1.0) as u32,
                );

                if desired_pixels != self.render_target.size() {
                    self.render_target
                        .resize(desired_pixels)
                        .unwrap_or_else(|err| {
                            log::error!("render target resize failed: {:#}", err);
                        });
                }
            });
    }

    pub fn points_to_pixels(
        allocated_points: egui::Vec2,
        pixels_per_point: f32,
    ) -> PhysicalSize<u32> {
        let w = (allocated_points.x * pixels_per_point).max(1.0) as u32;
        let h = (allocated_points.y * pixels_per_point).max(1.0) as u32;
        PhysicalSize::new(w, h)
    }

    pub fn render_to_target(&mut self, renderer: &mut Renderer) {
        self.render_target.bind();
        renderer.render_scene_pass();
        self.render_target.unbind();
    }

    pub fn texture_id(&self) -> egui::TextureId {
        self.texture_id
    }

    pub fn shutdown(&mut self, painter: &mut Painter) {
        painter.free_texture(self.texture_id);
    }
}
