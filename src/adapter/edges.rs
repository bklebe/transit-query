use trustfall::provider::{
    AsVertex, ContextIterator, ContextOutcomeIterator, EdgeParameters, ResolveEdgeInfo,
    VertexIterator,
};

use super::vertex::Vertex;

pub(super) fn resolve_trip_edge<'a, V: AsVertex<Vertex<'a>> + 'a>(
    contexts: ContextIterator<'a, V>,
    edge_name: &str,
    parameters: &EdgeParameters,
    resolve_info: &ResolveEdgeInfo,
) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex<'a>>> {
    match edge_name {
        "route" => trip::route(contexts, resolve_info),
        "vehicle" => trip::vehicle(contexts, resolve_info),
        _ => {
            unreachable!("attempted to resolve unexpected edge '{edge_name}' on type 'Trip'")
        }
    }
}

mod trip {
    use trustfall::provider::{
        resolve_neighbors_with, AsVertex, ContextIterator, ContextOutcomeIterator, ResolveEdgeInfo,
        VertexIterator,
    };

    use super::super::vertex::Vertex;

    pub(super) fn route<'a, V: AsVertex<Vertex<'a>> + 'a>(
        contexts: ContextIterator<'a, V>,
        _resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex<'a>>> {
        resolve_neighbors_with(contexts, |vertex| {
            let vertex = vertex
                .as_trip()
                .expect("conversion failed, vertex was not a Trip");
            todo!("get neighbors along edge 'route' for type 'Trip'")
        })
    }

    pub(super) fn vehicle<'a, V: AsVertex<Vertex<'a>> + 'a>(
        contexts: ContextIterator<'a, V>,
        _resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex<'a>>> {
        resolve_neighbors_with(contexts, |vertex| {
            let vertex = vertex
                .as_trip()
                .expect("conversion failed, vertex was not a Trip");
            todo!("get neighbors along edge 'vehicle' for type 'Trip'")
        })
    }
}

pub(super) fn resolve_vehicle_edge<'a, V: AsVertex<Vertex<'a>> + 'a>(
    contexts: ContextIterator<'a, V>,
    edge_name: &str,
    parameters: &EdgeParameters,
    resolve_info: &ResolveEdgeInfo,
) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex<'a>>> {
    match edge_name {
        "stop" => vehicle::stop(contexts, resolve_info),
        "trip" => vehicle::trip(contexts, resolve_info),
        _ => {
            unreachable!("attempted to resolve unexpected edge '{edge_name}' on type 'Vehicle'")
        }
    }
}

mod vehicle {
    use trustfall::provider::{
        resolve_neighbors_with, AsVertex, ContextIterator, ContextOutcomeIterator, ResolveEdgeInfo,
        VertexIterator,
    };

    use crate::{VehicleDescriptor, VehiclePosition};

    use super::super::vertex::Vertex;

    pub(super) fn stop<'a, V: AsVertex<Vertex<'a>> + 'a>(
        contexts: ContextIterator<'a, V>,
        _resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex<'a>>> {
        resolve_neighbors_with(contexts, |vertex| {
            let vertex = vertex
                .as_vehicle()
                .expect("conversion failed, vertex was not a Vehicle");
            todo!("get neighbors along edge 'stop' for type 'Vehicle'")
        })
    }

    pub(super) fn trip<'a, V: AsVertex<Vertex<'a>> + 'a>(
        contexts: ContextIterator<'a, V>,
        _resolve_info: &ResolveEdgeInfo,
    ) -> ContextOutcomeIterator<'a, V, VertexIterator<'a, Vertex<'a>>> {
        resolve_neighbors_with(contexts, |vertex| {
            let vertex: &VehiclePosition = vertex
                .as_vehicle()
                .expect("conversion failed, vertex was not a Vehicle");
            Box::new(
                vertex.trip.as_ref().map(Vertex::Trip).into_iter()
            )
        })
    }
}
