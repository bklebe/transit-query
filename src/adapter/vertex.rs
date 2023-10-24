use crate::gtfs_schedule::{Route, Stop};
use crate::{CarriageDetails, TripDescriptor, VehiclePosition};

#[non_exhaustive]
#[derive(Debug, Clone, trustfall::provider::TrustfallEnumVertex)]
pub enum Vertex<'a> {
    Route(&'a Route),
    Stop(&'a Stop),
    Trip(&'a TripDescriptor),
    Vehicle(&'a VehiclePosition),
    CarriageDetails(&'a CarriageDetails),
}
