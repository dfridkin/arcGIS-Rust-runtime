#![test_runner(crate::custom_runner)]

fn main() {
    custom_runner(&[
        test_map_view_creation,
        // add other tests here if needed
    ]);
}

fn custom_runner(tests: &[&dyn Fn()]) {
    for test in tests {
        test();
    }
}

fn test_map_view_creation() {
    use arcgis_rust::rendering::map_view::{MapView, Layer};
    use arcgis_rust::rendering::feature_layer::FeatureLayer;
    use arcgis_rust::data::feature::{Feature, FeatureLayer as DataFeatureLayer};
    use geo::{point, Geometry};
    use winit::event_loop::EventLoop;

    // Initialize an event loop
    let event_loop = EventLoop::new();

    // Create a MapView
    let map_view = MapView::new(&event_loop);
    assert!(map_view.layers.is_empty());
}
