use crate::text::observation::resource::Resource;

/// The persistence location of a `Shard`.
pub enum Scope {
    Directory(Resource),
    Ephemeral(Resource)
}