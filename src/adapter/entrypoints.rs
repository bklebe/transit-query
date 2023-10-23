use trustfall::provider::{ResolveInfo, VertexIterator};

use crate::{gtfs_schedule::GtfsSchedule, Message};

use super::vertex::Vertex;

pub(super) fn vehicle<'a>(
    message: &'a Message,
    _resolve_info: &ResolveInfo,
) -> VertexIterator<'a, Vertex<'a>> {
    Box::new(
        message
            .entity
            .iter()
            .map(|entity| Vertex::Vehicle(&entity.vehicle)),
    )
}
