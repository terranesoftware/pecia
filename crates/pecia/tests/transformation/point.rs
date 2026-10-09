use pecia::{Observation, Transformation, observation::{BufferKey, Resource}};

pub(crate) fn point(
    buffer: BufferKey,
    observation: &mut Observation,
    resource: Resource
) {
    Transformation::point(buffer, observation, resource);
}