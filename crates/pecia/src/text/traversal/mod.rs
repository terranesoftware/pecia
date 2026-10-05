pub mod diff;
pub mod direction;

use std::range::Range;

use blake3::Hash;

use crate::text::{observation::{Observation, buffers::buffer::region::Region, resource::Resource, scope::Scope, shard::change::Change}, traversal::{diff::Diff, direction::Direction}};

/// The entity that contribute to the navigation of text.
pub struct Traversal;

impl Traversal {
    pub fn route(
        deltas: &mut Vec<(Resource, Vec<Diff>)>,
        destination: (Scope, Hash),
        direction: Direction,
        observation: &Observation
    ) {
        let change = change(destination, direction, observation);
        
        for edit in change.edits() {
            // Find if there are already diffs for the edit's resource
            if let Some((_, diffs)) = deltas.iter_mut().find(|delta| &delta.0 == edit.resource()) {
                let start = edit.at();
                let end = edit.at() + edit.replaced().len();
                
                // Flag to check if an edit's region overlapped with the region of an existing diff
                let mut overlapped = false;
                
                // Collect all the diffs and then sort by their starting range
                // They're guaranteed not to overlap due to the coalesce at the end
                let mut targeted: Vec<&mut Diff> = diffs.iter_mut().filter(|diff| {
                    let range = diff.region().range();

                    start < range.end && end > range.start
                }).collect();
                targeted.sort_by_key(|diff| diff.region().range().start);

                // Turn it into a peekable iterator in case this edit spans multiple diffs
                let mut targeted = targeted.into_iter().peekable();
                
                // STILL BROKEN, HAVE TO FIX EDITS SPANNING THE DIFFS, PEEK TO THE NEXT DIFF TO CHECK ITS STARTING POINT
                while let Some(diff) = targeted.next() {
                    overlapped = true;
                    
                    let range = diff.region().range();
                    let intersection_start = start.saturating_sub(range.start);
                    let intersection_end = end.saturating_sub(range.start);
                    
                    let prefix = &diff.replacement()[..intersection_start];
                    let suffix = if intersection_end < diff.replacement().len() {
                        &diff.replacement()[intersection_end..]
                    }
                    else {
                        &[]
                    };
                    let mut intersection: Vec<u8> = Vec::with_capacity(prefix.len() + edit.replacement().len() + suffix.len());
                    intersection.extend(prefix);
                    intersection.extend(edit.replacement());
                    intersection.extend(suffix);

                    *diff.region_mut().range_mut() = Range {
                        start: start.min(range.start),
                        end: end.max(range.end)
                    };
                    *diff.replacement_mut() = intersection;
                }
                
                if !overlapped {
                    let diff = Diff::new(
                        Region::new(edit.at(), edit.at() + edit.replaced().len()),
                        edit.replacement().to_vec()
                    );

                    diffs.push(diff);
                }
            }
            else {
                let diff = Diff::new(
                    Region::new(edit.at(), edit.at() + edit.replaced().len()),
                    edit.replacement().to_vec()
                );
                
                deltas.push((edit.resource().clone(), vec![diff]));
            }
        }

        coalesce(deltas);
    }
}

// Helper for getting the desired destination change
fn change(
    destination: (Scope, Hash),
    direction: Direction,
    observation: &Observation
) -> &Change {
    let shard = observation.shards().get(&destination.0).expect("Shard doesn't exist");
    let head = shard.head();
    
    let relatives = match direction {
        Direction::Down => shard.changes().get(&head).unwrap().parents(),
        Direction::Up => shard.children().get(&head).unwrap()
    };
    let destination = relatives.iter().find(|hash| **hash == destination.1).unwrap();
    
    shard.changes().get(destination).unwrap()
}

// Helper for coalescing diffs
fn coalesce(deltas: &mut Vec<(Resource, Vec<Diff>)>) {
    todo!()
}