// TODO: Fix the tests
// tests/rendering_tests.rs
//use arcgis_rust::rendering::map_view::{MapView, Layer};
//use arcgis_rust::rendering::feature_layer::FeatureLayer;
//use arcgis_rust::data::feature::{Feature, FeatureLayer as DataFeatureLayer};
//use geo::{point, Geometry};
//use winit::event_loop::EventLoop;


//#[test]
//fn test_add_layer_to_map_view() {
    // Initialize an event loop
//    let event_loop = EventLoop::new();

    // Create a MapView
//    let mut map_view = MapView::new(&event_loop);

    // Create a FeatureLayer and add it to the MapView
//    let features = vec![
//        Feature { geometry: Geometry::Point(point!(x: 1.0, y: 1.0)), attributes: std::collections::HashMap::new() }
//    ];
//    let feature_layer = Box::new(FeatureLayer::new(features));
//    map_view.add_layer(feature_layer);

    // Check if the layer was added
//    assert_eq!(map_view.layers.len(), 1);
//}

//#[test]
//fn test_render_feature_layer() {
    // Create a FeatureLayer with one feature
//    let features = vec![
//        Feature { geometry: Geometry::Point(point!(x: 1.0, y: 1.0)), attributes: std::collections::HashMap::new() }
//    ];
 //   let feature_layer = FeatureLayer::new(features);

    // Use a closure to check if rendering is called
//    let mut rendered = false;
//    {
//        let render_fn = || {
//            rendered = true;
//        };
        // Call the render function
//        render_fn();
//    }

    // Check if the render method was called
//    assert!(rendered);
//}
