// src/rendering/map_view.rs
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder, Window},
};

pub struct MapView {
    window: Window,
    layers: Vec<Box<dyn Layer>>,
}

impl MapView {
    pub fn new(event_loop: &EventLoop<()>) -> Self {
        let window = WindowBuilder::new().build(event_loop).unwrap();
        Self { window, layers: vec![] }
    }

    pub fn add_layer(&mut self, layer: Box<dyn Layer>) {
        self.layers.push(layer);
    }

    pub fn render(&self) {
        // Implement rendering logic using wgpu
        for layer in &self.layers {
            layer.render();
        }
    }
}

pub trait Layer {
    fn render(&self);
}
