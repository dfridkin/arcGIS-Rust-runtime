// src/main.rs
mod core;
mod data;
mod rendering;
mod tasks;

use core::geometry::*;
use data::feature::*;
use rendering::map_view::MapView;
use tasks::geoprocessing::GeoprocessingTask;
use geo::Point;

#[tokio::main]
async fn main() {
    // Entry point for the application
    println!("ArcGIS Runtime in Rust - by dfridkin");

    // Example usage
    let point = Point::new(1.0, 2.0);
    println!("Point: {:?}", point);

    // Example geoprocessing task
    let geoprocessing_task = GeoprocessingTask {
        url: "https://example.com/arcgis/rest/services/GP/Service".to_string(),
    };
    match geoprocessing_task.execute("parameters").await {
        Ok(result) => println!("Geoprocessing result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}
