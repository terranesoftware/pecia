use std::path::PathBuf;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Resource {
    Directory(PathBuf),
    File(PathBuf),
    Memory(usize),
    Stdin
}