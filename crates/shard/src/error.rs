#[derive(Debug)]
pub enum ShardError {
    Cycle,
    InvalidKey,
    InvalidReference,
    NotARelative
}