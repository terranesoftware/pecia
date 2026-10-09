use std::path::PathBuf;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ResourceKind {
    Directory(PathBuf),
    File(PathBuf),
    Memory(usize),
    Stdin
}