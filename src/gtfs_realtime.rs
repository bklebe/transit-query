use serde::{Deserialize, Serialize};

use crate::gtfs_schedule::Trip;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Position {
    bearing: i64,
    pub(crate) latitude: f64,
    pub(crate) longitude: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TripDescriptor {
    pub(crate) direction_id: i64,
    route_id: String,
    pub(crate) schedule_relationship: Option<String>,
    pub(crate) start_date: Option<String>,
    pub(crate) start_time: Option<String>,
    pub(crate) trip_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CarriageDetails {
    carriage_sequence: i64,
    pub(crate) id: Option<String>,
    pub(crate) label: String,
    pub(crate) occupancy_percentage: i64,
    pub(crate) occupancy_status: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct VehicleDescriptor {
    pub(crate) id: String,
    pub(crate) label: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VehiclePosition {
    pub(crate) current_status: Option<String>,
    pub(crate) current_stop_sequence: Option<i64>,
    pub(crate) occupancy_percentage: Option<i64>,
    pub(crate) occupancy_status: Option<String>,
    pub(crate) multi_carriage_details: Option<Vec<CarriageDetails>>,
    pub(crate) position: Position,
    pub(crate) stop_id: Option<String>,
    pub(crate) timestamp: i64,
    pub(crate) trip: Option<Trip>,
    pub(crate) vehicle: VehicleDescriptor,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct VehicleEntity {
    id: String,
    pub(crate) vehicle: VehiclePosition,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VehiclePositions {
    pub(crate) entity: Vec<VehicleEntity>,
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
    trip: Trip,
    vehicle: Option<VehicleDescriptor>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TripUpdates {
    pub(crate) entity: Vec<TripEntity>,
}
