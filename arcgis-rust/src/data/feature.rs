use std::collections::HashMap;
use geo::{Geometry};
use crate::core::geometry::Envelope;

/// Represents a geographic feature with geometry and attributes.
#[derive(Debug, Clone)]
pub struct Feature {
    /// Geometry of the feature.
    pub geometry: Geometry<f64>,
    /// Attributes of the feature.
    pub attributes: HashMap<String, String>,
}

/// Manages a collection of geographic features.
pub struct FeatureLayer {
    /// Collection of features in the layer.
    pub features: Vec<Feature>,
}

impl FeatureLayer {
    /// Queries the feature layer for features within a given envelope.
    ///
    /// # Arguments
    ///
    /// * `envelope` - A reference to an `Envelope` used to filter features.
    ///
    /// # Returns
    ///
    /// * A vector of references to features within the envelope.
    ///
    /// # Examples
    ///
    /// ```
    /// use arcgis_rust::data::feature::{Feature, FeatureLayer};
    /// use arcgis_rust::core::geometry::{Envelope};
    /// use geo::point;
    /// use geo::Geometry;
    /// use std::collections::HashMap;
    ///
    /// let point1 = point!(x: 5.0, y: 5.0);
    /// let feature1 = Feature {
    ///     geometry: Geometry::Point(point1),
    ///     attributes: HashMap::new(),
    /// };
    /// let layer = FeatureLayer { features: vec![feature1.clone()] };
    ///
    /// let envelope = Envelope { min: point!(x: 0.0, y: 0.0), max: point!(x: 10.0, y: 10.0) };
    /// let result = layer.query(&envelope);
    ///
    /// assert_eq!(result.len(), 1);
    /// assert_eq!(result[0].geometry, Geometry::Point(point1));
    /// ```
    pub fn query(&self, envelope: &Envelope) -> Vec<&Feature> {
        self.features.iter().filter(|feature| {
            match &feature.geometry {
                Geometry::Point(point) => envelope.contains(point),
                // Add logic for Polygon and LineString as needed
                _ => false,
            }
        }).collect()
    }
}
