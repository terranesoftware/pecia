pub mod direction;
pub mod helpers;

use blake3::Hash;

use crate::{delta::Delta, observation::{Observation, Scope}, traversal::{direction::Direction, helpers::{change, create}}};

/// The entity that contribute to the navigation of text.
#[derive(Clone, Copy)]
pub struct Traversal;

impl Traversal {
    pub fn route(
        deltas: &mut Vec<Delta>,
        destination: (Scope, Hash),
        direction: Direction,
        observation: &Observation
    ) {
        // Get the destination change
        let change = change(destination, direction, observation);
        
        // Loop through edits and add to/modify deltas
        for edit in change.edits() {
            create(deltas, edit);
        }

        // TODO: Coalesce diffs within each resource
        // Deferred due to it being an optimization, not a necessity
    }
}