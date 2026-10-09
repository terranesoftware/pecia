use crate::observation::Resource;

#[derive(Eq, Hash, PartialEq)]
pub enum ScopeKind {
    Directory(Resource),
    Ephemeral(Resource)
}