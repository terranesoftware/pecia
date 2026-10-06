use std::mem::take;

use crate::{observation::buffers::buffer::region::Region, traversal::delta::{Delta, diff::Diff}};

pub(crate) fn coalesce(deltas: &mut Vec<Delta>) {
    for delta in deltas {
        // New coalesced replacement for the diff
        // Could technically reuse an existing diff, might do it, but doing this now for simplicity
        let mut replacement: Vec<u8> = Vec::new();
        
        // Splice bound
        let mut start: usize;
        
        let mut diffs = delta.diffs_mut();
        let mut iterator = delta.diffs_mut().iter().enumerate().peekable();

        while let Some((first, diff)) = iterator.next() {
            // Check if replacement has been added to
            // If not, then set this element to where to start the splice
            if replacement.is_empty() {
                start = first;
            }
            
            if let Some((second, next)) = iterator.peek() {
                if diff.region().range().end == next.region().range().start {
                    replacement.extend(diff.replacement());
                }
            }
            else {
                if !replacement.is_empty() {
                    let start = diffs[start].region().range().start;
                    let diff = Diff::new(
                        Region::new(start, diff.region().range().end),
                        take(&mut replacement)
                    );
                    
                    diffs.splice(start..=first, [diff]);
                }
            }
        }

        // See if the current diff's end matches the start of the next one
        // If it does, move on and repeat the check until it fails
        // After it fails, check if the current replacement is empty
        // If it is, then this was an isolated diff and there is no need to do anything
        // If it isn't, then this was the last diff in its coalesce streak and you have to add the new diff
    }
}