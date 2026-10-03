use std::assert_matches;

use blake3::Hash;
use buffers::kind::BufferKind;

use crate::text::{observation::{Observation, buffers::{BufferKey, buffer::{encoding::Encoding, region::Region}}, resource::Resource}, traversal::diff::Diff};

/// The entity that contributes to the manipulation of text.
pub struct Transformation;

impl Transformation {
    pub fn checkout(
        delta: Vec<(Resource, Vec<Diff>)>,
        destination: Hash,
        observation: &mut Observation
    ) {
        
    }
    
    pub fn close(key: BufferKey, observation: &mut Observation) {
        observation.buffers_mut().close(key);
    }

    pub fn edit(
        delta: Vec<(Resource, Vec<Diff>)>,
        observation: &mut Observation
    ) {
        
    }

    pub fn open(
        observation: &mut Observation,
        encoding: Encoding,
        implementation: BufferKind,
        region: Region,
        resource: Resource
    ) -> BufferKey {
        observation.buffers_mut().open(encoding, implementation, region, resource)
    }

    pub fn persist(
        key: BufferKey,
        observation: &mut Observation,
        resource: Resource
    ) {
        assert_matches!(resource, Resource::Memory(_) | Resource::Stdin, "Cannot persist to an ephemeral resource");
        
        let buffer = observation.buffers_mut().map_mut().get_mut(key).expect("Buffer has already been closed");
        assert_matches!(buffer.resource(), Resource::Memory(_) | Resource::Stdin, "Buffer is already persisted");
        
        *buffer.resource_mut() = resource;
    }

    pub fn reset(
        delta: Vec<(Resource, Vec<Diff>)>,
        destination: Hash,
        observation: &mut Observation
    ) {
        
    }

    pub fn save(
        key: BufferKey,
        observation: &mut Observation
    ) {
        let buffer = observation.buffers().map().get(key).expect("Buffer has already been closed");
        // Need some type of save operation
    }

    pub fn squash(observation: &mut Observation) {
        for (_, shard) in observation.shards_mut() {
            let head = shard.head();
            shard.changes_mut().retain(|key, _| *key == head);
        }
    }
}