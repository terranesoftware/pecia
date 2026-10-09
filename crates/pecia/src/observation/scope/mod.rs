mod kind;
pub use kind::ScopeKind;

use crate::observation::{Resource, resource::ResourceKind};

/// The persistence location of a `Shard`.
#[derive(Eq, Hash, PartialEq)]
pub struct Scope(ScopeKind);

impl Scope {
    pub fn directory(resource: Resource) -> Scope {
        match resource.kind() {
            ResourceKind::Directory(_) => Scope(ScopeKind::Directory(resource)),
            _ => panic!("Resource is not a Directory")
        }
    }

    pub fn ephemeral(resource: Resource) -> Scope {
        match resource.kind() {
            ResourceKind::Memory(_) => Scope(ScopeKind::Ephemeral(resource)),
            _ => panic!("Resource is not in Memory")
        }
    }

    pub fn kind(&self) -> &ScopeKind {
        &self.0
    }
}