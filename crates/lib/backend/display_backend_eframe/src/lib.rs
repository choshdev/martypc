/*
    MartyPC
    https://github.com/dbalsom/martypc

    Copyright 2022-2025 Daniel Balsom

    Permission is hereby granted, free of charge, to any person obtaining a
    copy of this software and associated documentation files (the “Software”),
    to deal in the Software without restriction, including without limitation
    the rights to use, copy, modify, merge, publish, distribute, sublicense,
    and/or sell copies of the Software, and to permit persons to whom the
    Software is furnished to do so, subject to the following conditions:

    The above copyright notice and this permission notice shall be included in
    all copies or substantial portions of the Software.

    THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
    AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
    FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
    DEALINGS IN THE SOFTWARE.

    ---------------------------------------------------------------------------

    display_backend_pixels::lib.rs

    Implements DisplayBackend for the Pixels backend
*/

#[cfg(feature = "use_wgpu")]
compile_error!("Wrong backend was selected for use_wgpu feature!");

mod display_window;
mod surface;

use std::sync::{Arc, RwLock};

pub use display_backend_trait::{
    BufferDimensions,
    DisplayBackend,
    DisplayBackendBuilder,
    DynDisplayTargetSurface,
    TextureDimensions,
    //DisplayBackendError
};
pub use surface::EFrameBackendSurface;

use marty_scaler_null::DisplayScaler;

use anyhow::{anyhow, Error};
use display_backend_trait::DisplayTargetSurface;
use egui;

#[derive(Debug)]
pub enum EFrameBackendType {
    RenderPass,
    EguiWindow,
}

pub struct EFrameBackend {
    be_type: EFrameBackendType,
    ctx: egui::Context,
}

impl EFrameBackend {
    pub fn new(
        be_type: EFrameBackendType,
        ctx: egui::Context,
        buffer_dim: BufferDimensions,
        surface_dim: TextureDimensions,
        //wgpu_render_state: &eframe::RenderState,
        _adapter_info: Option<()>,
    ) -> Result<EFrameBackend, Error> {
        Ok(EFrameBackend { be_type, ctx })
    }
}

impl DisplayBackendBuilder for EFrameBackend {
    fn build(_buffer_size: BufferDimensions, _surface_size: TextureDimensions) -> Self
    where
        Self: Sized,
    {
        todo!()
    }
}

pub type EFrameScalerType =
    Box<dyn DisplayScaler<(), (), (), NativeTextureView = (), NativeEncoder = (), NativeRenderPass = ()>>;

impl DisplayBackend<'_, '_, ()> for EFrameBackend {
    type NativeDevice = ();
    type NativeQueue = ();
    type NativeTexture = egui::TextureHandle;
    type NativeTextureFormat = ();
    type NativeBackend = ();
    type NativeBackendAdapterInfo = ();

    type NativeScaler = EFrameScalerType;

    fn adapter_info(&self) -> Option<Self::NativeBackendAdapterInfo> {
        None
    }

    fn device(&self) -> Arc<Self::NativeDevice> {
        Arc::new(())
    }

    fn queue(&self) -> Arc<Self::NativeQueue> {
        Arc::new(())
    }

    fn create_surface(
        &self,
        buffer_dim: BufferDimensions,
        surface_dim: TextureDimensions,
    ) -> Result<DynDisplayTargetSurface, Error> {
        let cpu_buffer = vec![0; buffer_dim.w as usize * buffer_dim.h as usize * 4];

        let buffer_image = egui::ColorImage {
            size:   [buffer_dim.w as usize, buffer_dim.h as usize],
            pixels: cpu_buffer
                .chunks_exact(4)
                .map(|rgba| egui::Color32::from_rgba_premultiplied(rgba[0], rgba[1], rgba[2], rgba[3]))
                .collect(),
        };
        let buffer_handle =
            self.ctx
                .load_texture("marty_buffer_texture", buffer_image, egui::TextureOptions::default());

        Ok(Box::new(EFrameBackendSurface {
            cpu_buffer,
            buffer: buffer_handle,
            buffer_dim,
            surface_dim,
        }))
    }

    fn resize_backing_texture(
        &mut self,
        surface: &mut DynDisplayTargetSurface,
        new_dim: BufferDimensions,
    ) -> Result<(), Error> {
        surface.resize_backing(Arc::new(()), new_dim)?;
        Ok(())
    }

    fn resize_surface_texture(
        &mut self,
        surface: &mut DynDisplayTargetSurface,
        new_dim: TextureDimensions,
    ) -> Result<(), Error> {
        //self.pixels.resize_surface(new.w, new.h)?;
        surface.resize_surface(Arc::new(()), Arc::new(()), new_dim)?;
        Ok(())
    }

    fn get_backend_raw(&mut self) -> Option<&mut Self::NativeBackend> {
        None
    }

    fn render(
        &mut self,
        surface: &mut DynDisplayTargetSurface,
        _scaler: Option<&mut Self::NativeScaler>,
        _gui: Option<&mut ()>,
    ) -> Result<(), Error> {
        // Update backing texture here if dirty.
        Ok(())
    }
}
