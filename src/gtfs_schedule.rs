use std::{fs, path::Path};

use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Debug)]
pub(crate) struct GtfsSchedule {
    pub routes: Vec<Route>,
    pub stops: Vec<Stop>,
    pub trips: Vec<ScheduledTrip>,
}

impl GtfsSchedule {
    pub(crate) fn from_path(path: &Path) -> Self {
        let routes = deserialize_file(path, "routes.txt");
        let stops = deserialize_file(path, "stops.txt");
        let trips = deserialize_file(path, "trips.txt");
        Self {
            routes,
            stops,
            trips,
        }
    }

    pub(crate) fn new(routes: String, stops: String, trips: String) -> Self {
        Self {
            routes: deserialize("routes.txt", routes),
            stops: deserialize("stops.txt", stops),
            trips: deserialize("trips.txt", trips),
        }
    }
}

fn deserialize_file<T>(path: &Path, file: &str) -> Vec<T>
where
    T: DeserializeOwned,
{
    let text = fs::read_to_string(path.join(Path::new(file)))
        .unwrap_or_else(|_| panic!("couldn't read {}", file));
    deserialize(file, text)
}

fn deserialize<T>(name: &str, text: String) -> Vec<T>
where
    T: DeserializeOwned,
{
    csv::Reader::from_reader(text.as_bytes())
        .deserialize()
        .collect::<Result<Vec<T>, _>>()
        .unwrap_or_else(|_| panic!("couldn't deserialize {}", name))
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Route {
    pub(super) route_id: String,
    pub(super) route_long_name: String,
    pub(super) route_short_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Stop {
    pub(super) stop_id: String,
    pub(super) stop_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScheduledTrip {
    pub(super) route_id: String,
    pub(super) service_id: String,
    pub(super) trip_id: String,
    pub(super) trip_headsign: String,
    pub(super) direction_id: i64,
    pub(super) shape_id: String,
    pub(super) block_id: String,
}
