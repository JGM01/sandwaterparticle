use std::sync::Arc;
use wgpu::{Device, Queue, Surface, SurfaceConfiguration};
use winit::window::Window;

pub struct State {
    pub surface: Surface<'static>,
    pub device: Device,
    pub queue: Queue,
    pub config: SurfaceConfiguration,
    pub is_surface_configured: bool,
    pub window: Arc<Window>,
}

impl State {
    pub async fn new(window: Arc<Window>) -> anyhow::Result<Self> {
        Ok(Self {
            surface: todo!(),
            device: todo!(),
            queue: todo!(),
            config: todo!(),
            is_surface_configured: false,
            window,
        })
    }

    pub fn resize(&mut self, _width: u32, _height: u32) {
        // TODO: handle resizing
    }

    pub fn render(&mut self) {
        self.window.request_redraw();
        // TODO: rendering logic
    }
}
