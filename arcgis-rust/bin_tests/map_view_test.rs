use arcgis_rust::rendering::map_view::{MapView, Layer};
use arcgis_rust::rendering::feature_layer::FeatureLayer;
use arcgis_rust::data::feature::{Feature, FeatureLayer as DataFeatureLayer};
use geo::{point, Geometry};
use winit::event_loop::EventLoop;

fn run_map_view_creation_test() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize an event loop
    let event_loop = EventLoop::new();

    // Create a MapView
    let map_view = MapView::new(&event_loop);
    assert!(map_view.layers.is_empty());

    // Simulate accessing a geoprocessing endpoint (example)
    let response = reqwest::blocking::get("https://services9.arcgis.com/RHVPKKiFTONKtxq3/arcgis/rest/services/NDGD_SmokeForecast_v1/FeatureServer/0")?;
    if response.status().is_success() {
        println!("Geoprocessing result: {:?}", response.text()?);
    } else {
        eprintln!("Failed to fetch geoprocessing result: {}", response.status());
        eprintln!("Response: {:?}", response.text()?);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_view_creation() {
        run_map_view_creation_test().unwrap();
    }
}

fn main() {
    if let Err(e) = run_map_view_creation_test() {
        eprintln!("Test failed: {:?}", e);
    }
}
