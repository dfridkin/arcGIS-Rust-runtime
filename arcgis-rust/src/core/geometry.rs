use geo::{point, Point, Polygon, LineString};

/// Represents a rectangular envelope defined by its minimum and maximum coordinates.
///
/// # Examples
///
/// ```
/// use my_arcgis_runtime::core::geometry::{Envelope, point};
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
