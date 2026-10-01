pub mod kind;
pub mod operations;

pub enum Buffer {
    Placeholder
}

impl Buffer {
    pub fn new() -> Self {
        Self::Placeholder
    }
}