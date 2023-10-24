use crate::{
    gtfs_realtime::{CarriageDetails, TripDescriptor, VehiclePosition},
    gtfs_schedule::{Route, Stop, Trip},
};

#[non_exhaustive]
#[derive(Debug, Clone, trustfall::provider::TrustfallEnumVertex)]
pub enum Vertex<'a> {
    Route(&'a Route),
    Stop(&'a Stop),
    Trip(&'a Trip),
    TripDescriptor(&'a TripDescriptor),
    Vehicle(&'a VehiclePosition),
    CarriageDetails(&'a CarriageDetails),
}
