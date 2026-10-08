pub mod implementation;
pub mod operations;

use crate::implementation::{Implementation, vector::Vector};

pub enum Buffer {
    Vector(Vector)
}

impl Buffer {
    pub fn vector() -> Self {
        Self::Vector(Vector {  })
    }
}

impl PartialEq<Implementation> for Buffer {
    fn eq(&self, other: &Implementation) -> bool {
        match self {
            Self::Vector(_) => matches!(other, Implementation::Vector)
        }
    }
}