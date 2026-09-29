use std::path::PathBuf;

pub enum Resource {
    Directory(PathBuf),
    File(PathBuf),
    Memory,
    Stdin
}