// src/tasks/locator.rs
use reqwest::Error;
use geo::Point;

pub struct LocatorTask {
    pub url: String,
}

impl LocatorTask {
    pub async fn geocode(&self, address: &str) -> Result<Point<f64>, Error> {
        let client = reqwest::Client::new();
        let res = client.get(&self.url)
            .query(&[("address", address)])
            .send()
            .await?;
        // parse the response to get the location
        let location = Point::new(0.0, 0.0);  // TODO: Replace with actual parsing logic
        Ok(location)
    }
}
