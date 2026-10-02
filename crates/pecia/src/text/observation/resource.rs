use std::path::PathBuf;

#[derive(Debug)]
pub enum Resource {
    Directory(PathBuf),
    File(PathBuf),
    Memory,
    Stdin
}