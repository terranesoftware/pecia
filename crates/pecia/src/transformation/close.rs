use crate::{observation::{Observation, buffers::BufferKey}, transformation::Transformation};

impl Transformation {
    pub fn close(
        key: BufferKey,
        observation: &mut Observation
    ) {
        let buffers = observation.buffers_mut();

        // If at the limit, remove the buffer
        if buffers.current() == buffers.max() {
            buffers.map_mut().remove(key).expect("Buffer has already been closed");
        }
        // If not, mark it unused and increment the counter
        else {
            buffers.map_mut().get_mut(key).expect("Buffer has already been closed").0 = false;
            *buffers.current_mut() += 1;
        }
    }
}