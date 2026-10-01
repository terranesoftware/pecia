use std::path::PathBuf;

/// The persistence location of a `Shard`.
pub enum Scope {
    Directory(PathBuf),
    // Decide how to identify this
    Ephemeral
}