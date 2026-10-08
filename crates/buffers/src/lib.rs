pub mod implementation;
pub mod operations;

use crate::implementation::vector::Vector;

pub enum Buffer {
    Vector(Vector)
}

impl Buffer {
    pub fn vector() -> Self {
        Self::Vector(Vector {  })
    }
}