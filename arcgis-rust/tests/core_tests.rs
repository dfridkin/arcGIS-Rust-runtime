// tests/core_tests.rs
use arcgis_rust::core::geometry::*;
use geo::{point, polygon, LineString, MultiPolygon, Coord};
use std::collections::HashSet;

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

    let expected_intersection = MultiPolygon(vec![polygon![
        (x: 5.0, y: 5.0),
        (x: 10.0, y: 5.0),
        (x: 10.0, y: 10.0),
        (x: 5.0, y: 10.0),
        (x: 5.0, y: 5.0),
    ]]);

    let mut result: Vec<Coord<f64>> = intersection(&poly1, &poly2).0[0].exterior().clone().0;
    result.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap().then(a.y.partial_cmp(&b.y).unwrap()));
    result.dedup();

    let mut expected: Vec<Coord<f64>> = expected_intersection.0[0].exterior().clone().0;
    expected.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap().then(a.y.partial_cmp(&b.y).unwrap()));
    expected.dedup();

    assert_eq!(result, expected);
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

    let expected_union = MultiPolygon(vec![polygon![
        (x: 0.0, y: 0.0),
        (x: 10.0, y: 0.0),
        (x: 10.0, y: 5.0),
        (x: 15.0, y: 5.0),
        (x: 15.0, y: 15.0),
        (x: 5.0, y: 15.0),
        (x: 5.0, y: 10.0),
        (x: 0.0, y: 10.0),
        (x: 0.0, y: 0.0),
    ]]);

    let mut result: Vec<Coord<f64>> = union(&poly1, &poly2).0[0].exterior().clone().0;
    result.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap().then(a.y.partial_cmp(&b.y).unwrap()));
    result.dedup();

    let mut expected: Vec<Coord<f64>> = expected_union.0[0].exterior().clone().0;
    expected.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap().then(a.y.partial_cmp(&b.y).unwrap()));
    expected.dedup();

    assert_eq!(result, expected);
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