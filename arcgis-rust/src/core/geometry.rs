use geo::{Point};

/// Represents a rectangular envelope defined by its minimum and maximum coordinates.
#[derive(Debug, Clone)]
pub struct Envelope {
    pub min: Point<f64>,
    pub max: Point<f64>,
}

impl Envelope {
    /// Checks if a point is within the envelope.
    pub fn contains(&self, point: &Point<f64>) -> bool {
        point.x() >= self.min.x() && point.x() <= self.max.x() && 
        point.y() >= self.min.y() && point.y() <= self.max.y()
    }
}
