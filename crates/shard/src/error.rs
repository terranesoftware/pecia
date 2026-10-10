#[derive(Debug)]
pub enum ShardError {
    InvalidHash,
    InvalidReference,
    NotARelative
}