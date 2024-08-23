use geojson::{Feature, FeatureCollection, GeoJson, Geometry, JsonValue, Value};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    let raw_data = std::fs::read_to_string("./182_PM_Peak_speeds.geojson")?;

    let geojson: GeoJson = raw_data.parse::<GeoJson>().unwrap();
    let feature_collection: FeatureCollection = FeatureCollection::try_from(geojson).unwrap();

    let new_bus_lane_stats = feature_collection
        .features
        .into_iter()
        .map(|feature| {
            let mut feature = feature;

            let mut properties = feature.properties.as_ref().unwrap().clone();

            if let JsonValue::String(p20_mph) = properties.get("p20_mph").unwrap() {
                let p20_kmh = p20_mph.parse::<f64>().unwrap() * 1.609344;

                properties.insert("p20_kmh".to_string(), p20_kmh.into());
            }

            feature
        })
        .collect::<Vec<Feature>>();

    Ok(())
}
