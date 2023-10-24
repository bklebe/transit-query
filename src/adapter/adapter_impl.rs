use std::sync::{Arc, OnceLock};

use trustfall::{
    provider::{
        resolve_coercion_using_schema, resolve_property_with, AsVertex,
        ContextIterator, ContextOutcomeIterator, EdgeParameters, ResolveEdgeInfo, ResolveInfo,
        Typename, VertexIterator,
    },
    FieldValue, Schema,
};

use crate::{gtfs_schedule::GtfsSchedule, TripUpdates, VehiclePositions};

use super::vertex::Vertex;

static SCHEMA: OnceLock<Schema> = OnceLock::new();

#[non_exhaustive]
#[derive(Debug)]
pub struct Adapter<'a> {
    vehicle_positions: &'a VehiclePositions,
    trip_updates: &'a TripUpdates,
    gtfs_schedule: &'a GtfsSchedule,
}

impl<'a> Adapter<'a> {
    pub const SCHEMA_TEXT: &'static str = include_str!("./schema.gql");

    pub fn schema() -> &'static Schema {
        SCHEMA.get_or_init(|| Schema::parse(Self::SCHEMA_TEXT).expect("not a valid schema"))
    }

    pub(crate) fn new(
        vehicle_positions: &'a VehiclePositions,
        trip_updates: &'a TripUpdates,
        gtfs_schedule: &'a GtfsSchedule,
    ) -> Self {
        Self {
            vehicle_positions,
            trip_updates,
            gtfs_schedule,
        }
    }
}

impl<'a> trustfall::provider::Adapter<'a> for Adapter<'a> {
    type Vertex = Vertex<'a>;

    fn resolve_starting_vertices(
        &self,
        edge_name: &Arc<str>,
        _parameters: &EdgeParameters,
        resolve_info: &ResolveInfo,
    ) -> VertexIterator<'a, Self::Vertex> {
        match edge_name.as_ref() {
            "Vehicle" => super::entrypoints::vehicle(self.vehicle_positions, resolve_info),
            // "Trip" => super::entrypoints::trip(self.trip_updates, resolve_info),
            _ => {
                unreachable!(
                    "attempted to resolve starting vertices for unexpected edge name: {edge_name}"
                )
            }
        }
    }

    fn resolve_property<V: AsVertex<Self::Vertex> + 'a>(
        &self,
        contexts: ContextIterator<'a, V>,
        type_name: &Arc<str>,
        property_name: &Arc<str>,
        resolve_info: &ResolveInfo,
    ) -> ContextOutcomeIterator<'a, V, FieldValue> {
        if property_name.as_ref() == "__typename" {
            return resolve_property_with(contexts, |vertex| vertex.typename().into());
        }

        match type_name.as_ref() {
            "CarriageDetails" => super::properties::resolve_carriage_details_property(
                contexts,
                property_name.as_ref(),
                resolve_info,
            ),
            "Route" => super::properties::resolve_route_property(
                contexts,
                property_name.as_ref(),
                resolve_info,
            ),
            "Stop" => super::properties::resolve_stop_property(
                contexts,
                property_name.as_ref(),
                resolve_info,
            ),
            "Trip" => super::properties::resolve_trip_property(
                contexts,
                property_name.as_ref(),
                resolve_info,
            ),
            "Vehicle" => super::properties::resolve_vehicle_property(
                contexts,
                property_name.as_ref(),
                resolve_info,
            ),
            _ => {
                unreachable!(
                    "attempted to read property '{property_name}' on unexpected type: {type_name}"
                )
            }
        }
    }

    fn resolve_neighbors<V: AsVertex<Self::Vertex> + 'a>(
        &self,
        contexts: ContextIterator<'a, V>,
        type_name: &Arc<str>,
        edge_name: &Arc<str>,
        parameters: &EdgeParameters,
        resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Self::Vertex>> {
        match type_name.as_ref() {
            "Trip" => super::edges::resolve_trip_edge(
                self.gtfs_schedule,
                contexts,
                edge_name,
                parameters,
                resolve_info,
            ),
            "TripDescriptor" => super::edges::resolve_trip_edge(
                self.gtfs_schedule,
                contexts,
                edge_name,
                parameters,
                resolve_info,
            ),
            "Vehicle" => super::edges::resolve_vehicle_edge(
                self.gtfs_schedule,
                contexts,
                edge_name.as_ref(),
                parameters,
                resolve_info,
            ),
            _ => {
                unreachable!(
                    "attempted to resolve edge '{edge_name}' on unexpected type: {type_name}"
                )
            }
        }
    }

    fn resolve_coercion<V: AsVertex<Self::Vertex> + 'a>(
        &self,
        contexts: ContextIterator<'a, V>,
        _type_name: &Arc<str>,
        coerce_to_type: &Arc<str>,
        _resolve_info: &ResolveInfo,
    ) -> ContextOutcomeIterator<'a, V, bool> {
        resolve_coercion_using_schema(contexts, Self::schema(), coerce_to_type.as_ref())
    }
}
