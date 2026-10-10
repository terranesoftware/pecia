use std::{iter::Peekable, range::Range};

use crate::{change::Edit, operations::traverse::Delta};

pub(super) fn apply<'a>(
    edit: &Edit,
    end: usize,
    overlapped: &mut bool,
    start: usize,
    mut targeted: Peekable<impl Iterator<Item = &'a mut Delta>>
) {
    // Cursor to keep track without modifying the edit itself
    let mut cursor: usize = start;
    
    while let Some(delta) = targeted.next() {
        *overlapped = true;
        
        // Get the start of this intersection as its used in both branches
        let region = delta.region;
        let intersection_start = cursor.saturating_sub(region.start);
        
        // Prefix is always the part hanging off before the intersection
        // In the case that an edit overlaps multiple deltas, this would always be empty after the first delta
        let prefix = &delta.replacement[..intersection_start];
        
        let intersection = if let Some(next) = targeted.peek() {
            // If there is another delta after, suffix right up until its starting point
            // 
            // Example:
            // Delta 1 region: [3, 10)
            // Delta 2 region: [15, 30)
            // Edit region: [5, 25)
            // 
            // The unaccounted for region between the deltas are [10, 15)
            // prefix = [3, 5)
            // suffix = [5, 15)
            // cursor = 15

            // Subtract against the cursor since you have to normalize against the thing you're indexing into
            let next_start = next.region.start;
            let intersection_end = next_start.saturating_sub(cursor);
            let suffix = &edit.replacement[..intersection_end];

            // Two part intersection
            // 1. The prefix (possibly empty if the edit and the delta start at the same index)
            // 2. The suffix (extends from the prefix to the start of the next delta)
            let mut intersection: Vec<u8> = Vec::with_capacity(prefix.len() + suffix.len());
            intersection.extend(prefix);
            intersection.extend(suffix);

            // Calculate the resulting region for this delta
            // Just wherever it started at, but the end is always right before the start of the next delta 
            delta.region = Range {
                start: cursor.min(region.start),
                end: cursor
            };
            cursor = next_start;

            intersection
        }
        else {
            // Calculate the intersection end since this is the last delta
            let intersection_end = end.saturating_sub(region.start);
            
            // Handle the case where the intersection end would be out of bounds
            let suffix = if intersection_end < delta.replacement().len() {
                &delta.replacement()[intersection_end..]
            }
            else {
                &[]
            };
            
            // Three part intersection
            // 1. The prefix (possibly empty if the edit and delta start at the same index)
            // 2. The edit
            // 3. The suffix (any part leftover in the delta, possibly empty)
            let mut intersection: Vec<u8> = Vec::with_capacity(prefix.len() + edit.replacement.len() + suffix.len());
            intersection.extend(prefix);
            intersection.extend(&edit.replacement);
            intersection.extend(suffix);

            // Calculate the resulting region for this delta
            // Just wherever it started at and wherever it ended at
            delta.region = Range {
                start: cursor.min(region.start),
                end: end.max(region.end)
            };

            intersection
        };

        delta.replacement = intersection;
    }
}