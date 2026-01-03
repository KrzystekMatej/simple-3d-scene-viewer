use std::sync::Arc;

use anyhow::{Context, anyhow};
use glow::HasContext;
use winit::dpi::PhysicalSize;

pub struct RenderTarget {
    gl: Arc<glow::Context>,
    size: PhysicalSize<u32>,
    framebuffer: glow::NativeFramebuffer,
    depth_buffer: glow::NativeRenderbuffer,
    color_texture: glow::NativeTexture,
}

impl RenderTarget {
    pub fn new(gl: Arc<glow::Context>) -> anyhow::Result<Self> {
        let framebuffer = unsafe {
            gl.create_framebuffer()
                .map_err(anyhow::Error::msg)
                .context("failed to create framebuffer")?
        };

        let depth_buffer = unsafe {
            gl.create_renderbuffer()
                .map_err(anyhow::Error::msg)
                .context("failed to create renderbuffer")?
        };

        let color_texture = unsafe {
            gl.create_texture()
                .map_err(anyhow::Error::msg)
                .context("failed to create color texture")?
        };

        let mut this = Self {
            gl,
            size: PhysicalSize::new(1, 1),
            framebuffer,
            depth_buffer,
            color_texture,
        };
        this.resize(this.size)?;
        Ok(this)
    }

    pub fn bind(&self) {
        unsafe {
            self.gl
                .bind_framebuffer(glow::FRAMEBUFFER, Some(self.framebuffer));
            self.gl
                .viewport(0, 0, self.size.width as i32, self.size.height as i32);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            self.gl.bind_framebuffer(glow::FRAMEBUFFER, None);
        }
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) -> anyhow::Result<()> {
        let width = size.width.max(1);
        let height = size.height.max(1);
        self.size = PhysicalSize::new(width, height);

        unsafe {
            self.gl
                .bind_texture(glow::TEXTURE_2D, Some(self.color_texture));
            self.gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MIN_FILTER,
                glow::LINEAR as i32,
            );
            self.gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MAG_FILTER,
                glow::LINEAR as i32,
            );
            self.gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_WRAP_S,
                glow::CLAMP_TO_EDGE as i32,
            );
            self.gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_WRAP_T,
                glow::CLAMP_TO_EDGE as i32,
            );
            self.gl.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                glow::RGBA8 as i32,
                width as i32,
                height as i32,
                0,
                glow::RGBA,
                glow::UNSIGNED_BYTE,
                glow::PixelUnpackData::Slice(None),
            );
            self.gl.bind_texture(glow::TEXTURE_2D, None);

            self.gl
                .bind_renderbuffer(glow::RENDERBUFFER, Some(self.depth_buffer));
            self.gl.renderbuffer_storage(
                glow::RENDERBUFFER,
                glow::DEPTH24_STENCIL8,
                width as i32,
                height as i32,
            );
            self.gl.bind_renderbuffer(glow::RENDERBUFFER, None);

            self.gl
                .bind_framebuffer(glow::FRAMEBUFFER, Some(self.framebuffer));
            self.gl.framebuffer_texture_2d(
                glow::FRAMEBUFFER,
                glow::COLOR_ATTACHMENT0,
                glow::TEXTURE_2D,
                Some(self.color_texture),
                0,
            );
            self.gl.framebuffer_renderbuffer(
                glow::FRAMEBUFFER,
                glow::DEPTH_STENCIL_ATTACHMENT,
                glow::RENDERBUFFER,
                Some(self.depth_buffer),
            );

            let status = self.gl.check_framebuffer_status(glow::FRAMEBUFFER);
            self.gl.bind_framebuffer(glow::FRAMEBUFFER, None);

            if status != glow::FRAMEBUFFER_COMPLETE {
                return Err(anyhow!("framebuffer incomplete: {:#x}", status));
            }
        }

        Ok(())
    }

    pub fn framebuffer(&self) -> glow::NativeFramebuffer {
        self.framebuffer
    }

    pub fn color_texture(&self) -> glow::NativeTexture {
        self.color_texture
    }

    pub fn size(&self) -> PhysicalSize<u32> {
        self.size
    }
}

impl Drop for RenderTarget {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_framebuffer(self.framebuffer);
            self.gl.delete_renderbuffer(self.depth_buffer);
            self.gl.delete_texture(self.color_texture);
        }
    }
}
