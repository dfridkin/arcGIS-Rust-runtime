// src/tasks/geoprocessing.rs
use reqwest::Error;

/// Represents a geoprocessing task that executes geospatial operations.
///
/// # Examples
///
/// ```
/// use my_arcgis_runtime::tasks::geoprocessing::GeoprocessingTask;
/// use tokio;
///
/// #[tokio::main]
/// async fn main() {
///     let geoprocessing_task = GeoprocessingTask {
///         url: "https://example.com/arcgis/rest/services/GP/Service".to_string(),
///     };
///
///     match geoprocessing_task.execute("parameters").await {
///         Ok(result) => println!("Geoprocessing result: {}", result),
///         Err(e) => println!("Error: {}", e),
///     }
/// }
/// ```
pub struct GeoprocessingTask {
    pub url: String, // URL of the geoprocessing service
}

impl GeoprocessingTask {
    /// Executes the geoprocessing task with given parameters.
    ///
    /// # Arguments
    ///
    /// * `parameters` - Parameters for the geoprocessing task.
    ///
    /// # Returns
    ///
    /// * A result containing the response from the geoprocessing service or an error.
    pub async fn execute(&self, parameters: &str) -> Result<String, Error> {
        let client = reqwest::Client::new();
        let res = client.post(&self.url)
            .body(parameters.to_string())
            .send()
            .await?;
        Ok(res.text().await?)
    }
}
