mod kind;
use std::path::PathBuf;

pub use kind::ResourceKind;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Resource(ResourceKind);

impl Resource {
    pub fn directory(path: PathBuf) -> Resource {
        if !path.is_dir() {
            panic!("Is not a directory");
        }

        Resource(ResourceKind::Directory(path))
    }

    pub fn file(path: PathBuf) -> Resource {
        if !path.is_file() {
            panic!("Is not a file");
        }

        Resource(ResourceKind::File(path))
    }

    pub fn memory(id: usize) -> Resource {
        Resource(ResourceKind::Memory(id))
    }

    pub fn stdin() -> Resource {
        Resource(ResourceKind::Stdin)
    }
}