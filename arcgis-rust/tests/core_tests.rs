// tests/core_tests.rs
use arcgis_rust::core::geometry::*;
use geo::point;

#[test]
fn test_point_creation() {
    let point = point!(x: 1.0, y: 2.0);
    assert_eq!(point.x(), 1.0);
    assert_eq!(point.y(), 2.0);
}

#[test]
fn test_envelope_contains() {
    let envelope = Envelope { min: point!(x: 0.0, y: 0.0), max: point!(x: 10.0, y: 10.0) };
    let point_inside = point!(x: 5.0, y: 5.0);
    let point_outside = point!(x: 15.0, y: 15.0);
    assert!(envelope.contains(&point_inside));
    assert!(!envelope.contains(&point_outside));
}
