// src/rendering/feature_layer.rs
pub use crate::data::feature::FeatureLayer;
pub use crate::rendering::map_view::Layer;


impl Layer for FeatureLayer {
    fn render(&self) {
        // TODO: mplement feature layer rendering logic using wgpu
        for feature in &self.features {
            println!("{:?}", feature);
        }
    }
}
