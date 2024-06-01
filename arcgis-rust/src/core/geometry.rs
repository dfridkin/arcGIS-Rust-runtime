use geo::{point, Point, Polygon, LineString, MultiPolygon};
use geo::BooleanOps;
use geo::algorithm::bounding_rect::BoundingRect;
use geo::algorithm::coords_iter::CoordsIter;

/// Represents a rectangular envelope defined by its minimum and maximum coordinates.
///
/// # Examples
///
/// ```
/// use arcgis_rust::core::geometry::{Envelope};
/// use geo::point;
///
/// let envelope = Envelope {
///     min: point!(x: 0.0, y: 0.0),
///     max: point!(x: 10.0, y: 10.0),
/// };
/// let point_inside = point!(x: 5.0, y: 5.0);
/// let point_outside = point!(x: 15.0, y: 15.0);
///
/// assert!(envelope.contains(&point_inside));
/// assert!(!envelope.contains(&point_outside));
/// ```

#[derive(Debug, Clone)]
pub struct Envelope {
    /// Minimum coordinates of the envelope.
    pub min: Point<f64>,
    /// Maximum coordinates of the envelope.
    pub max: Point<f64>,
}

impl Envelope {
    /// Checks if a point is within the envelope.
    ///
    /// # Arguments
    ///
    /// * `point` - A reference to a `Point` to be checked.
    ///
    /// # Returns
    ///
    /// * `true` if the point is within the envelope, `false` otherwise.
    pub fn contains(&self, point: &Point<f64>) -> bool {
        point.x() >= self.min.x() && point.x() <= self.max.x() && 
        point.y() >= self.min.y() && point.y() <= self.max.y()
    }
}

/// Calculates the intersection of two polygons.
///
/// # Arguments
///
/// * `poly1` - A reference to the first `Polygon`.
/// * `poly2` - A reference to the second `Polygon`.
///
/// # Returns
///
/// * A `Polygon` representing the intersection of the two input polygons.
///
/// # Examples
///
/// ```
/// use geo::polygon;
/// use arcgis_rust::core::geometry::intersection;
///
/// let poly1 = polygon![
///     (x: 0.0, y: 0.0),
///     (x: 10.0, y: 0.0),
///     (x: 10.0, y: 10.0),
///     (x: 0.0, y: 10.0),
///     (x: 0.0, y: 0.0),
/// ];
///
/// let poly2 = polygon![
///     (x: 5.0, y: 5.0),
///     (x: 15.0, y: 5.0),
///     (x: 15.0, y: 15.0),
///     (x: 5.0, y: 15.0),
///     (x: 5.0, y: 5.0),
/// ];
///
/// let intersection_poly = intersection(&poly1, &poly2);
/// ```

pub fn intersection(poly1: &Polygon<f64>, poly2: &Polygon<f64>) -> MultiPolygon<f64> {
    poly1.intersection(poly2)
}

/// Merges two polygons into a single polygon (union).
///
/// # Arguments
///
/// * `poly1` - A reference to the first `Polygon`.
/// * `poly2` - A reference to the second `Polygon`.
///
/// # Returns
///
/// * A `Polygon` representing the union of the two input polygons.
///
/// # Examples
///
/// ```
/// use geo::polygon;
/// use arcgis_rust::core::geometry::union;
///
/// let poly1 = polygon![
///     (x: 0.0, y: 0.0),
///     (x: 10.0, y: 0.0),
///     (x: 10.0, y: 10.0),
///     (x: 0.0, y: 10.0),
///     (x: 0.0, y: 0.0),
/// ];
///
/// let poly2 = polygon![
///     (x: 5.0, y: 5.0),
///     (x: 15.0, y: 5.0),
///     (x: 15.0, y: 15.0),
///     (x: 5.0, y: 15.0),
///     (x: 5.0, y: 5.0),
/// ];
///
/// let union_poly = union(&poly1, &poly2);
/// ```

pub fn union(poly1: &Polygon<f64>, poly2: &Polygon<f64>) -> MultiPolygon<f64> {
    poly1.union(poly2)
}

/// Creates a bounding box as a simple buffer zone around a line.
///
/// # Arguments
///
/// * `line` - A reference to the `LineString` around which to create the buffer.
/// * `distance` - The buffer distance.
///
/// # Returns
///
/// * A `Polygon` representing the buffer zone around the input line.
///
/// # Examples
///
/// ```
/// use geo::LineString;
/// use arcgis_rust::core::geometry::buffer;
///
/// let line = LineString::from(vec![
///     (0.0, 0.0),
///     (10.0, 0.0),
/// ]);
///
/// let buffer_distance = 1.0;
/// let buffer_poly = buffer(&line, buffer_distance);
/// ```

pub fn buffer(line: &LineString<f64>, distance: f64) -> Polygon<f64> {
    let bounding_rect = line.bounding_rect().unwrap();
    let min_x = bounding_rect.min().x - distance;
    let max_x = bounding_rect.max().x + distance;
    let min_y = bounding_rect.min().y - distance;
    let max_y = bounding_rect.max().y + distance;

    Polygon::new(
        vec![
            (min_x, min_y),
            (max_x, min_y),
            (max_x, max_y),
            (min_x, max_y),
            (min_x, min_y),
        ]
        .into(),
        vec![],
    )
}
