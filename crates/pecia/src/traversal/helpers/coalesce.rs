use crate::traversal::delta::Delta;

pub(crate) fn coalesce(deltas: &mut Vec<Delta>) {
    for delta in deltas {
        // TODO: Have to create a whole new Vec<Diff>, just append non-coalesced diffs in again
        let mut replacement: Vec<u8> = Vec::new();
        let mut diffs = delta.diffs().iter().peekable();

        while let Some(diff) = diffs.next() {
            let end = diff.region().range().end;
            
            if let Some(next) = diffs.peek() {
                let start = next.region().range().start;

                if start == end {
                    replacement.extend(diff.replacement());
                }
            }
            else {
                if !replacement.is_empty() {
                    
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