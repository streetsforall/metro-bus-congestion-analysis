use geojson::{FeatureCollection, GeoJson, JsonValue};
use std::error::Error;

struct CleanSegment {
    route_short_name: String,
    p20_kmh: f64,
}

fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    let raw_data = std::fs::read_to_string("./182_PM_Peak_speeds.geojson")?;

    let geojson: GeoJson = raw_data.parse::<GeoJson>().unwrap();
    let feature_collection: FeatureCollection = FeatureCollection::try_from(geojson).unwrap();

    let new_bus_lane_stats = feature_collection
        .features
        .into_iter()
        .filter_map(|feature| {
            let properties = feature.properties.as_ref().unwrap().clone();

            match (
                properties.get("p20_mph").unwrap(),
                properties.get("route_short_name").unwrap(),
            ) {
                (JsonValue::String(p20_mph), JsonValue::String(route_short_name)) => {
                    let p20_kmh = p20_mph.parse::<f64>().unwrap() * 1.609344;

                    Some(CleanSegment {
                        route_short_name: route_short_name.to_string(),
                        p20_kmh,
                    })
                }
                _ => None,
            }
        })
        //segments eligable for bus lanes are under 30 kmh
        .filter(|x| x.p20_kmh < 30.)
        .collect::<Vec<CleanSegment>>();

    Ok(())
}
