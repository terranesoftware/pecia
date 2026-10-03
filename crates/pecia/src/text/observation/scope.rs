use crate::text::observation::resource::Resource;

/// The persistence location of a `Shard`.
#[derive(Hash, PartialEq, Eq)]
pub enum Scope {
    Directory(Resource),
    Ephemeral(Resource)
}