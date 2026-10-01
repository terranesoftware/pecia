use std::path::PathBuf;

pub enum Scope {
    Directory(PathBuf),
    // Decide how to identify this
    Ephemeral
}