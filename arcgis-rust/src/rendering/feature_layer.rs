// src/rendering/feature_layer.rs
pub use crate::data::feature::FeatureLayer;
pub use crate::rendering::map_view::Layer;

/// Represents a feature layer containing geographic features.
impl Layer for FeatureLayer {
    /// Renders the feature layer.
    fn render(&self) {
        // TODO: Implement feature layer rendering logic
        for feature in &self.features {
            println!("{:?}", feature);
        }
    }
}