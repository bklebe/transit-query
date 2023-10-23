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
        "id" => todo!("implement property 'id' in fn `resolve_route_property()`"),
        "route_short_name" => {
            todo!("implement property 'route_short_name' in fn `resolve_route_property()`")
        }
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
        "name" => todo!("implement property 'name' in fn `resolve_stop_property()`"),
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
