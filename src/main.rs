use std::{collections::BTreeMap, sync::Arc};

use maplit::btreemap;
use serde::{Deserialize, Serialize};
use trustfall::execute_query;

use crate::adapter::Adapter;

mod adapter;

fn main() {
    let body = reqwest::blocking::get("https://cdn.mbta.com/realtime/VehiclePositions.json")
        .expect("couldn't pull VehiclePositions.json")
        .text()
        .expect("invalid response encoding");
    let contents: Message = serde_json::from_str(&body).expect("couldn't deserialize");
    let adapter: Adapter = Adapter::new(&contents);
    execute_query(
        Adapter::schema(),
        adapter.into(),
        include_str!("../query.graphql"),
        btreemap! {
            Arc::from("routeId") => Arc::from("Red"),
        },
    )
    .expect("query failed to parse")
    .map(|v| {
        v.into_iter()
            // .map(|(k, v)| (k, TransparentValue::from(v)))
            .collect::<BTreeMap<_, _>>()
    })
    .for_each(|result| {
        println!(
            "{}",
            serde_json::to_string_pretty(&result).expect("failed to serialize result")
        )
    });
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
    schedule_relationship: String,
    start_date: String,
    start_time: Option<String>,
    trip_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct VehicleDescriptor {
    id: String,
    label: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VehiclePosition {
    current_status: Option<String>,
    current_stop_sequence: Option<i64>,
    occupancy_percentage: Option<i64>,
    occupancy_status: Option<String>,
    position: Position,
    stop_id: Option<String>,
    timestamp: i64,
    trip: Option<TripDescriptor>,
    vehicle: VehicleDescriptor,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Entity {
    id: String,
    vehicle: VehiclePosition,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Message {
    entity: Vec<Entity>,
}
