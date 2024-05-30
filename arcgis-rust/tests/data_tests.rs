// tests/data_tests.rs
use arcgis_rust::core::geometry::*;
use arcgis_rust::data::feature::*;
use geo::{point, Geometry};

#[test]
fn test_feature_layer_query() {
    let point1 = point!(x: 5.0, y: 5.0);
    let point2 = point!(x: 15.0, y: 15.0);
    let feature1 = Feature { geometry: Geometry::Point(point1), attributes: std::collections::HashMap::new() };
    let feature2 = Feature { geometry: Geometry::Point(point2), attributes: std::collections::HashMap::new() };
    let layer = FeatureLayer { features: vec![feature1.clone(), feature2.clone()] };

    let envelope = Envelope { min: point!(x: 0.0, y: 0.0), max: point!(x: 10.0, y: 10.0) };
    let result = layer.query(&envelope);

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].geometry, Geometry::Point(point1));
}
