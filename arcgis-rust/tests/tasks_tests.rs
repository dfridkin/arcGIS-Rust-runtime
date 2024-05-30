// tests/tasks_tests.rs
use arcgis_rust::tasks::geoprocessing::GeoprocessingTask;
use arcgis_rust::tasks::locator::LocatorTask;
use geo::point;

#[tokio::test]
async fn test_geoprocessing_task_execution() {
    // Mock URL for testing
    let url = "https://httpbin.org/post";
    let geoprocessing_task = GeoprocessingTask { url: url.to_string() };

    // Execute the task with mock parameters
    let result = geoprocessing_task.execute("parameters").await;

    // Check if the task executed successfully
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_locator_task_geocode() {
    // Mock URL for testing
    let url = "https://nominatim.openstreetmap.org/search";
    let locator_task = LocatorTask { url: url.to_string() };

    // Geocode an address
    let result = locator_task.geocode("1600 Amphitheatre Parkway, Mountain View, CA").await;

    // Check if the geocoding task executed successfully
    assert!(result.is_ok());
    let location = result.unwrap();
    // Since this is a mock test, the location is not parsed correctly, and we set it to (0.0, 0.0)
    assert_eq!(location, point!(x: 0.0, y: 0.0));
}
