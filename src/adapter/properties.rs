use trustfall::{
    provider::{
        field_property, resolve_property_with, AsVertex, ContextIterator, ContextOutcomeIterator,
        ResolveInfo,
    },
    FieldValue,
};

use super::vertex::Vertex;

pub(super) fn resolve_route_property<'a, V: AsVertex<Vertex<'a>> + 'a>(
    contexts: ContextIterator<'a, V>,
    property_name: &str,
    _resolve_info: &ResolveInfo,
) -> ContextOutcomeIterator<'a, V, FieldValue> {
    match property_name {
        "id" => resolve_property_with(contexts, field_property!(as_route, route_id)),
        "short_name" => {
            resolve_property_with(contexts, field_property!(as_route, route_short_name))
        }
        "long_name" => resolve_property_with(contexts, field_property!(as_route, route_long_name)),
        _ => {
            unreachable!("attempted to read unexpected property '{property_name}' on type 'Route'")
        }
    }
}

pub(super) fn resolve_stop_property<'a, V: AsVertex<Vertex<'a>> + 'a>(
    contexts: ContextIterator<'a, V>,
    property_name: &str,
    _resolve_info: &ResolveInfo,
) -> ContextOutcomeIterator<'a, V, FieldValue> {
    match property_name {
        "id" => todo!("implement property 'id' in fn `resolve_stop_property()`"),
        "name" => resolve_property_with(contexts, field_property!(as_stop, stop_name)),
        _ => {
            unreachable!("attempted to read unexpected property '{property_name}' on type 'Stop'")
        }
    }
}

pub(super) fn resolve_trip_property<'a, V: AsVertex<Vertex<'a>> + 'a>(
    contexts: ContextIterator<'a, V>,
    property_name: &str,
    _resolve_info: &ResolveInfo,
) -> ContextOutcomeIterator<'a, V, FieldValue> {
    match property_name {
        "id" => resolve_property_with(contexts, field_property!(as_trip, trip_id)),
        "route_id" => resolve_property_with(contexts, field_property!(as_trip, route_id)),
        _ => {
            unreachable!("attempted to read unexpected property '{property_name}' on type 'Trip'")
        }
    }
}

pub(super) fn resolve_vehicle_property<'a, V: AsVertex<Vertex<'a>> + 'a>(
    contexts: ContextIterator<'a, V>,
    property_name: &str,
    _resolve_info: &ResolveInfo,
) -> ContextOutcomeIterator<'a, V, FieldValue> {
    match property_name {
        "id" => resolve_property_with(
            contexts,
            field_property!(as_vehicle, vehicle, { vehicle.id.clone().into() }),
        ),
        "label" => resolve_property_with(
            contexts,
            field_property!(as_vehicle, vehicle, { vehicle.label.clone().into() }),
        ),
        "latitude" => resolve_property_with(
            contexts,
            field_property!(as_vehicle, position, {
                FieldValue::Float64(position.latitude)
            }),
        ),
        "longitude" => resolve_property_with(
            contexts,
            field_property!(as_vehicle, position, {
                FieldValue::Float64(position.longitude)
            }),
        ),
        _ => {
            unreachable!(
                "attempted to read unexpected property '{property_name}' on type 'Vehicle'"
            )
        }
    }
}

pub(super) fn resolve_carriage_details_property<'a, V: AsVertex<Vertex<'a>> + 'a>(
    contexts: ContextIterator<'a, V>,
    property_name: &str,
    _resolve_info: &ResolveInfo,
) -> ContextOutcomeIterator<'a, V, FieldValue> {
    match property_name {
        "id" => resolve_property_with(contexts, field_property!(as_carriage_details, id)),
        "label" => resolve_property_with(contexts, field_property!(as_carriage_details, label)),
        "occupancy_percentage" => resolve_property_with(
            contexts,
            field_property!(as_carriage_details, occupancy_percentage),
        ),
        "occupancy_status" => resolve_property_with(
            contexts,
            field_property!(as_carriage_details, occupancy_status),
        ),

        _ => {
            unreachable!(
                "attempted to read unexpected property '{property_name}' on type 'CarriageDetails'"
            )
        }
    }
}
