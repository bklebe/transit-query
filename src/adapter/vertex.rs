use crate::gtfs_schedule::{Route, ScheduledTrip, Stop};
use crate::{CarriageDetails, TripDescriptor, VehiclePosition};

#[non_exhaustive]
#[derive(Debug, Clone, trustfall::provider::TrustfallEnumVertex)]
pub enum Vertex<'a> {
    Route(&'a Route),
    Stop(&'a Stop),
    Trip(&'a ScheduledTrip),
    TripDescriptor(&'a TripDescriptor),
    Vehicle(&'a VehiclePosition),
    CarriageDetails(&'a CarriageDetails),
}
