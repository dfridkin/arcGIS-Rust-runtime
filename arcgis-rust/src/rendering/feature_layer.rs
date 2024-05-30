// src/rendering/feature_layer.rs
use crate::data::feature::FeatureLayer;
use crate::rendering::map_view::Layer;

impl Layer for FeatureLayer {
    fn render(&self) {
        // Implement feature layer rendering logic using wgpu
        for feature in &self.features {
            println!("{:?}", feature);
        }
    }
}
