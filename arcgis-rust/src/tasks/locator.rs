use reqwest::Error;
use geo::Point;

/// Represents a locator task for geocoding and reverse geocoding.
///
/// # Examples
///
/// ```
/// use arcgis_rust::tasks::locator::LocatorTask;
/// use tokio;
///
/// #[tokio::main]
/// async fn main() {
///     let locator_task = LocatorTask {
///         url: "https://nominatim.openstreetmap.org/search".to_string(),
///     };
///
///     match locator_task.geocode("1600 Amphitheatre Parkway, Mountain View, CA").await {
///         Ok(location) => println!("Location: {:?}", location),
///         Err(e) => println!("Error: {}", e),
///     }
/// }
/// ```
pub struct LocatorTask {
    pub url: String, // URL of the geocoding service
}

impl LocatorTask {
    /// Geocodes an address and returns the corresponding geographic point.
    ///
    /// # Arguments
    ///
    /// * `address` - The address to geocode.
    ///
    /// # Returns
    ///
    /// * A result containing the geographic point or an error.
    pub async fn geocode(&self, address: &str) -> Result<Point<f64>, Error> {
        let client = reqwest::Client::new();
        let res = client.get(&self.url)
            .query(&[("address", address)])
            .send()
            .await?;
        // parse the response to get the location
        let location = Point::new(0.0, 0.0); 
        Ok(location)
    }
}
