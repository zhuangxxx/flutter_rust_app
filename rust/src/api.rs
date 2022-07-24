use geojson::{GeoJson, GeoJson::FeatureCollection, Value};
use std::{fs::File, io::BufReader};
use tokio::runtime::Runtime;

pub struct Position {
    pub latitude: f64,
    pub longitude: f64,
}

pub struct GeoMap {
    pub name: String,
    pub coordinates: Vec<Position>,
}

pub fn load_geojson(path: String) -> anyhow::Result<Vec<GeoMap>> {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        // TODO: use tokio::fs
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let geojson = GeoJson::from_reader(reader)?;
        let mut geo_map_vec = Vec::new();
        if let FeatureCollection(collect) = geojson {
            for feature in &collect.features {
                let mut geo_map = GeoMap {
                    name: String::new(),
                    coordinates: Vec::new(),
                };
                if let Some(name) = feature.property("name") {
                    if let Some(name) = name.as_str() {
                        geo_map.name = String::from(name);
                    }
                }
                if let Some(geometry) = &feature.geometry {
                    if let Value::MultiPolygon(positions) = &geometry.value {
                        let mut coordinates = Vec::new();
                        for position in positions[0][0].iter() {
                            coordinates.push(Position {
                                longitude: position[0],
                                latitude: position[1],
                            });
                        }
                        geo_map.coordinates = coordinates;
                    }
                }

                geo_map_vec.push(geo_map);
            }
        }

        Ok(geo_map_vec)
    })
}
