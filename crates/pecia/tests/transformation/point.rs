use pecia::{Observation, Transformation, observation::{BufferKey, Resource}};

use crate::observation;

#[test]
fn test_point() {
    let mut observation = observation();

    // point(buffer, &mut observation, resource);
}

fn point(
    buffer: BufferKey,
    observation: &mut Observation,
    resource: Resource
) {
    Transformation::point(buffer, observation, resource);
}