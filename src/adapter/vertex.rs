use crate::gtfs_schedule::Route;
use crate::{TripDescriptor, VehiclePosition};

#[non_exhaustive]
#[derive(Debug, Clone, trustfall::provider::TrustfallEnumVertex)]
pub enum Vertex<'a> {
    Route(&'a Route),
    Stop(()),
    Trip(&'a TripDescriptor),
    Vehicle(&'a VehiclePosition),
}
