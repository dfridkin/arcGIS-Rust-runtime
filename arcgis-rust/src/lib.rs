// src/lib.rs

pub mod core;
pub mod data;
pub mod rendering;
pub mod tasks;

use wasm_bindgen::prelude::*;
use geo::{Geometry, Point};
use serde::{Serialize, Deserialize};

#[wasm_bindgen]
pub fn create_point(x: f64, y: f64) -> JsValue {
    let point = Point::new(x, y);
    JsValue::from_serde(&point).unwrap()
}

#[wasm_bindgen]
pub fn contains_point(envelope_min_x: f64, envelope_min_y: f64, envelope_max_x: f64, envelope_max_y: f64, point_x: f64, point_y: f64) -> bool {
    let min = Point::new(envelope_min_x, envelope_min_y);
    let max = Point::new(envelope_max_x, envelope_max_y);
    let envelope = Envelope { min, max };
    let point = Point::new(point_x, point_y);
    envelope.contains(&point)
}

#[derive(Serialize, Deserialize)]
struct Envelope {
    min: Point<f64>,
    max: Point<f64>,
}

impl Envelope {
    fn contains(&self, point: &Point<f64>) -> bool {
        point.x() >= self.min.x() && point.x() <= self.max.x() && 
        point.y() >= self.min.y() && point.y() <= self.max.y()
    }
}