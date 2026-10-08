use crate::{observation::{Observation, buffers::BufferKey}, transformation::Transformation};

impl Transformation {
    /// Closes a buffer.
    pub fn close(
        key: BufferKey,
        observation: &mut Observation
    ) {
        let buffers = observation.buffers_mut();

        // If at the limit, remove the buffer
        if buffers.current() == buffers.max() {
            buffers.map_mut().remove(key).expect("Buffer has already been closed");
        }
        else {
            let buffer = buffers.map_mut().get_mut(key).expect("Buffer has already been closed");
            assert!(buffer.0 == true, "Buffer has already been closed");
            
            // Clear the buffer, mark it out of use, and then increment the counter
            buffer.1.implementation_mut().clear();
            buffer.0 = false;
            *buffers.current_mut() += 1;
        }
    }
}