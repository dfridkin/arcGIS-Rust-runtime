// src/data/feature.rs
use std::collections::HashMap;
use geo::{Geometry};
use crate::core::geometry::Envelope;

#[derive(Debug, Clone)]
pub struct Feature {
    pub geometry: Geometry<f64>,
    pub attributes: HashMap<String, String>,
}

pub struct FeatureLayer {
    pub features: Vec<Feature>,
}

impl FeatureLayer {
    /// Queries the feature layer for features within a given envelope.
    pub fn query(&self, envelope: &Envelope) -> Vec<&Feature> {
        self.features.iter().filter(|feature| {
            match &feature.geometry {
                Geometry::Point(point) => envelope.contains(point),
                // TODO: Add logic for Polygon and LineString as needed
                _ => false,
            }
        }).collect()
    }
}
