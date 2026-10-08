use pecia::{Observation, observation::Buffers};

pub fn observation() -> Observation {
    Observation::new(Buffers::new(15))
}