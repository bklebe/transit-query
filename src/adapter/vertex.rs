use crate::{VehiclePosition, TripDescriptor};

#[non_exhaustive]
#[derive(Debug, Clone, trustfall::provider::TrustfallEnumVertex)]
pub enum Vertex<'a> {
    Route(()),
    Stop(()),
    Trip(&'a TripDescriptor),
    Vehicle(&'a VehiclePosition),
}
