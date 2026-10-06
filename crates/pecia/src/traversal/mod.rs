pub mod delta;
pub mod direction;
pub mod helpers;

use blake3::Hash;

use crate::{observation::{Observation, scope::Scope}, traversal::{delta::Delta, direction::Direction, helpers::{change, coalesce, create}}};

/// The entity that contribute to the navigation of text.
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

        // Coalesce diffs within each resource
        coalesce(deltas);
    }
}