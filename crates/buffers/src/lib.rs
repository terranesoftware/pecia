mod implementations;
pub use implementations::Vector;

mod operations;

pub enum Buffer {
    Vector(Vector)
}

impl Buffer {
    pub fn vector() -> Self {
        Self::Vector(Vector::new())
    }
}