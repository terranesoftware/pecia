use std::path::PathBuf;

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum Resource {
    Directory(PathBuf),
    File(PathBuf),
    Memory(usize),
    Stdin
}