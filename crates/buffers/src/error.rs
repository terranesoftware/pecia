/// Possible failures when using a `Buffer`.
#[derive(Debug)]
pub enum BufferError {
    OutOfBounds,
    ReversedBounds
}