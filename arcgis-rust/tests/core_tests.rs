// tests/core_tests.rs
use arcgis_rust::core::geometry::*;
use super::*;
use geo::{point, polygon, LineString};


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


#[test]
fn test_intersection() {
    let poly1 = polygon![
        (x: 0.0, y: 0.0),
        (x: 10.0, y: 0.0),
        (x: 10.0, y: 10.0),
        (x: 0.0, y: 10.0),
        (x: 0.0, y: 0.0),
    ];

    let poly2 = polygon![
        (x: 5.0, y: 5.0),
        (x: 15.0, y: 5.0),
        (x: 15.0, y: 15.0),
        (x: 5.0, y: 15.0),
        (x: 5.0, y: 5.0),
    ];

    let expected_intersection = polygon![
        (x: 5.0, y: 5.0),
        (x: 10.0, y: 5.0),
        (x: 10.0, y: 10.0),
        (x: 5.0, y: 10.0),
        (x: 5.0, y: 5.0),
    ];

    assert_eq!(intersection(&poly1, &poly2), expected_intersection);
}

#[test]
fn test_union() {
    let poly1 = polygon![
        (x: 0.0, y: 0.0),
        (x: 10.0, y: 0.0),
        (x: 10.0, y: 10.0),
        (x: 0.0, y: 10.0),
        (x: 0.0, y: 0.0),
    ];

    let poly2 = polygon![
        (x: 5.0, y: 5.0),
        (x: 15.0, y: 5.0),
        (x: 15.0, y: 15.0),
        (x: 5.0, y: 15.0),
        (x: 5.0, y: 5.0),
    ];

    let expected_union = polygon![
        (x: 0.0, y: 0.0),
        (x: 10.0, y: 0.0),
        (x: 10.0, y: 5.0),
        (x: 15.0, y: 5.0),
        (x: 15.0, y: 15.0),
        (x: 5.0, y: 15.0),
        (x: 5.0, y: 10.0),
        (x: 0.0, y: 10.0),
        (x: 0.0, y: 0.0),
    ];

    assert_eq!(union(&poly1, &poly2), expected_union);
}

#[test]
fn test_buffer() {
    let line = LineString::from(vec![
        (0.0, 0.0),
        (10.0, 0.0),
    ]);

    let buffer_distance = 1.0;
    let expected_buffer = polygon![
        (x: -1.0, y: -1.0),
        (x: 11.0, y: -1.0),
        (x: 11.0, y: 1.0),
        (x: -1.0, y: 1.0),
        (x: -1.0, y: -1.0),
    ];

    assert_eq!(buffer(&line, buffer_distance), expected_buffer);
}