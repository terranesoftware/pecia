#[derive(Debug)]
pub enum ShardError {
    InvalidKey,
    InvalidReference,
    NotARelative
}