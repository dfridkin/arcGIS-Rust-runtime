use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder, Window},
};

/// Represents the main view for rendering maps.
///
/// # Examples
///
/// ```
/// use my_arcgis_runtime::rendering::map_view::MapView;
/// use winit::event_loop::EventLoop;
///
/// let event_loop = EventLoop::new();
/// let map_view = MapView::new(&event_loop);
/// ```
pub struct MapView {
    window: Window, // Window for rendering the map
    pub layers: Vec<Box<dyn Layer>>, // Collection of layers to be rendered
}

impl MapView {
    /// Creates a new MapView instance.
    ///
    /// # Arguments
    ///
    /// * `event_loop` - A reference to the `EventLoop` for handling window events.
    ///
    /// # Returns
    ///
    /// * A new instance of `MapView`.
    pub fn new(event_loop: &EventLoop<()>) -> Self {
        let window = WindowBuilder::new().build(event_loop).unwrap();
        Self { window, layers: vec![] }
    }

    /// Adds a layer to the MapView.
    ///
    /// # Arguments
    ///
    /// * `layer` - A boxed layer implementing the `Layer` trait.
    pub fn add_layer(&mut self, layer: Box<dyn Layer>) {
        self.layers.push(layer);
    }

    /// Renders all layers in the MapView.
    pub fn render(&self) {
        for layer in &self.layers {
            layer.render();
        }
    }
}

/// Trait for defining a layer that can be rendered.
pub trait Layer {
    fn render(&self);
}
