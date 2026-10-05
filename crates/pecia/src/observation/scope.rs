use crate::observation::resource::Resource;

/// The persistence location of a `Shard`.
#[derive(Eq, Hash, PartialEq)]
pub enum Scope {
    Directory(Resource),
    Ephemeral(Resource)
}