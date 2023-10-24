use crate::gtfs_schedule::{Route, Trip, Stop};
use crate::{CarriageDetails, TripDescriptor, VehiclePosition};

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
