use pecia::{Observation, Transformation, observation::BufferKey};

pub(crate) fn close(
    key: BufferKey,
    observation: &mut Observation
) {
    Transformation::close(key, observation);
}