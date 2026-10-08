use crate::Buffer;

pub(crate) mod vector;

#[derive(PartialEq)]
pub enum Implementation {
    Vector
}

impl PartialEq<Buffer> for Implementation {
    fn eq(&self, other: &Buffer) -> bool {
        match self {
            Implementation::Vector => matches!(other, Buffer::Vector(_))
        }
    }
}