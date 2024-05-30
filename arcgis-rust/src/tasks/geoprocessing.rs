// src/tasks/geoprocessing.rs
use reqwest::Error;

pub struct GeoprocessingTask {
    pub url: String,
}

impl GeoprocessingTask {
    pub async fn execute(&self, parameters: &str) -> Result<String, Error> {
        let client = reqwest::Client::new();
        let res = client.post(&self.url)
            .body(parameters.to_string())
            .send()
            .await?;
        Ok(res.text().await?)
    }
}
