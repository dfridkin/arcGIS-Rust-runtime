# ArcGIS Runtime in Rust 🗺️

    🗺️  _~^~^~_   🔧
    \) /  o o  \ (/
      '_   -   _'
      / '-----' \

**NOTE**: This Runtime is still under development, please report any bugs or issues to help us make this runtime more reliable!


Welcome to **ArcGIS Rust**! This project is a Rust-based geospatial intelligence library inspired by ArcGIS Runtime. It provides core geospatial types, rendering capabilities, and task execution for geoprocessing and locator tasks.

## Table of Contents

- [Introduction](#introduction)
- [Installation](#installation)
- [Usage](#usage)
  - [Examples](#examples)
- [API Specifications](#api-specifications)
  - [Core Module](#core-module)
  - [Data Module](#data-module)
  - [Rendering Module](#rendering-module)
  - [Tasks Module](#tasks-module)
- [Contributing](#contributing)
- [License](#license)

## Introduction

This project aims to provide a comprehensive geospatial intelligence library in Rust. It includes:
- Core geospatial types (Point, Polygon, Envelope, etc.)
- Feature layers for managing geographic features
- Map view rendering capabilities using `wgpu`
- Task execution for geoprocessing and locator tasks using `reqwest`

## Installation

To use this library in your project, add the following to your `Cargo.toml`:

```toml
[dependencies]
arcgis_rust = { git = "https://github.com/dfridkin/arcgis-rust-runtime.git" }
geo = "0.23.0"
wgpu = "0.12.0"
winit = "0.26.0"
reqwest = { version = "0.11", features = ["json"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

## Usage

**Examples**

Creating and Using Geospatial Types
```rust
use arcgis_rust::core::geometry::*;
use geo::point;

fn main() {
    let point = point!(x: 1.0, y: 2.0);
    println!("Point: {:?}", point);

    let envelope = Envelope {
        min: point!(x: 0.0, y: 0.0),
        max: point!(x: 10.0, y: 10.0),
    };

    let point_inside = point!(x: 5.0, y: 5.0);
    let point_outside = point!(x: 15.0, y: 15.0);

    println!("Point inside: {}", envelope.contains(&point_inside));
    println!("Point outside: {}", envelope.contains(&point_outside));
}
```
Executing a Geoprocessing Task
```rust
use arcgis_rust::tasks::geoprocessing::GeoprocessingTask;
use tokio;

#[tokio::main]
async fn main() {
    let geoprocessing_task = GeoprocessingTask {
        url: "https://example.com/arcgis/rest/services/GP/Service".to_string(),
    };

    match geoprocessing_task.execute("parameters").await {
        Ok(result) => println!("Geoprocessing result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}
```

## API Specifications

**Core Module**

`Point`

Represents a 2D point with x and y coordinates.
```rust
#[derive(Debug, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}
```


`Envelope`

Represents a rectangular envelope defined by its minimum and maximum coordinates.

```rust
#[derive(Debug, Clone)]
pub struct Envelope {
    pub min: Point<f64>,
    pub max: Point<f64>,

    pub fn contains(&self, point: &Point<f64>) -> bool;
}
```

## Data Module
`Feature`

Represents a geographic feature with geometry and attributes.

```rust

#[derive(Debug, Clone)]
pub struct Feature {
    pub geometry: Geometry<f64>,
    pub attributes: HashMap<String, String>,
}
```
`FeatureLayer`

Manages a collection of features.

```rust

pub struct FeatureLayer {
    pub features: Vec<Feature>,

    pub fn query(&self, envelope: &Envelope) -> Vec<&Feature>;
}
```
## Rendering Module
`MapView`

Manages the rendering of a map view.

```rust

pub struct MapView {
    pub fn new(event_loop: &EventLoop<()>) -> Self;
    pub fn add_layer(&mut self, layer: Box<dyn Layer>);
    pub fn render(&self);
}
```
`Layer`

Trait for defining a renderable layer.

```rust

pub trait Layer {
    fn render(&self);
}
```
## Tasks Module
`GeoprocessingTask`

Executes geoprocessing tasks.

```rust

pub struct GeoprocessingTask {
    pub url: String,

    pub async fn execute(&self, parameters: &str) -> Result<String, reqwest::Error>;
}
```
`LocatorTask`

Executes geocoding tasks.

```rust

pub struct LocatorTask {
    pub url: String,

    pub async fn geocode(&self, address: &str) -> Result<Point<f64>, reqwest::Error>;
}
```
## Contributing

We welcome contributions to make this project even more awesome! To contribute, please follow these steps:

    Fork the repository.
    Create a new branch (git checkout -b feature-branch).
    Make your changes.
    Commit your changes (git commit -m 'Add new feature').
    Push to the branch (git push origin feature-branch).
    Open a pull request.

## License

This project is licensed under the MIT License. See the LICENSE file for details.


    Happy coding, and may your Rust adventures be filled with joy and discovery! 🦀❤️