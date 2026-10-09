mod implementations;
pub use implementations::Vector;

mod operations;

/// A container of text.
pub enum Buffer {
    Vector(Vector)
}

impl Buffer {
    /// Constructs a `Buffer::Vector`.
    pub fn vector() -> Self {
        Self::Vector(Vector::new())
    }
}