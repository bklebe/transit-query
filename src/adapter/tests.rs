use std::path::Path;

use trustfall::provider::check_adapter_invariants;

use crate::{deserialize_feed, gtfs_schedule::GtfsSchedule};

use super::Adapter;

#[test]
fn adapter_satisfies_trustfall_invariants() {
    let vehicles = deserialize_feed(include_str!("../../test_data/VehiclePositions.json"));
    let trips = deserialize_feed(include_str!("../../test_data/TripUpdates.json"));
    let schedule = GtfsSchedule::from_path(Path::new("../MBTA_GTFS"));

    let adapter = Adapter::new(&vehicles, &trips, &schedule);
    let schema = Adapter::schema();
    check_adapter_invariants(schema, adapter);
}
