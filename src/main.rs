use std::{collections::BTreeMap, path::Path, sync::Arc};

use gtfs_schedule::GtfsSchedule;
use maplit::btreemap;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use trustfall::{execute_query, TransparentValue};

use crate::adapter::Adapter;

mod adapter;
mod gtfs_schedule;

fn main() {
    let contents = get_feed("https://cdn.mbta.com/realtime/VehiclePositions.json");
    let trip_updates = get_feed("https://cdn.mbta.com/realtime/TripUpdates.json");
    let schedule = GtfsSchedule::from_path(Path::new("../MBTA_GTFS"));
    let adapter: Adapter = Adapter::new(&contents, &trip_updates, &schedule);
    let variables: BTreeMap<Arc<str>, Arc<str>> = BTreeMap::new(); // btreemap! {Arc::from("minLabel") => Arc::from("3900")};
    execute_query(
        Adapter::schema(),
        adapter.into(),
        include_str!("../query.gql"),
        variables,
    )
    .expect("query failed to parse")
    .map(|v| {
        v.into_iter()
            .map(|(k, v)| (k, TransparentValue::from(v)))
            .collect::<BTreeMap<_, _>>()
    })
    .for_each(|result| {
        println!(
            "{}",
            serde_json::to_string_pretty(&result).expect("failed to serialize result")
        )
    });
}

fn get_feed<T>(url: &str) -> T
where
    T: DeserializeOwned,
{
    let body = reqwest::blocking::get(url)
        .unwrap_or_else(|_| panic!("couldn't pull {}", url))
        .text()
        .expect("invalid response encoding");
    deserialize_feed(&body)
}

pub(crate) fn deserialize_feed<T>(text: &str) -> T
where
    T: DeserializeOwned,
{
    serde_json::from_str(text).expect("couldn't deserialize")
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Position {
    bearing: i64,
    latitude: f64,
    longitude: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TripDescriptor {
    direction_id: i64,
    route_id: String,
    schedule_relationship: Option<String>,
    start_date: Option<String>,
    start_time: Option<String>,
    trip_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CarriageDetails {
    carriage_sequence: i64,
    id: Option<String>,
    label: String,
    occupancy_percentage: i64,
    occupancy_status: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct VehicleDescriptor {
    id: String,
    label: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VehiclePosition {
    current_status: Option<String>,
    current_stop_sequence: Option<i64>,
    occupancy_percentage: Option<i64>,
    occupancy_status: Option<String>,
    multi_carriage_details: Option<Vec<CarriageDetails>>,
    position: Position,
    stop_id: Option<String>,
    timestamp: i64,
    trip: Option<TripDescriptor>,
    vehicle: VehicleDescriptor,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct VehicleEntity {
    id: String,
    vehicle: VehiclePosition,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct VehiclePositions {
    entity: Vec<VehicleEntity>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct TripEntity {
    id: String,
    trip_update: TripUpdate,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct StopTimeEvent {
    time: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
struct StopTimeUpdate {
    arrival: Option<StopTimeEvent>,
    departure: Option<StopTimeEvent>,
    stop_id: String,
    stop_sequence: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
struct TripUpdate {
    stop_time_update: Option<Vec<StopTimeUpdate>>,
    timestamp: Option<i64>,
    trip: TripDescriptor,
    vehicle: Option<VehicleDescriptor>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct TripUpdates {
    entity: Vec<TripEntity>,
}
