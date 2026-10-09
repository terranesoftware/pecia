use std::path::PathBuf;

use indexmap::map::MutableKeys;

use crate::{Observation, Transformation, observation::{Resource, Scope, resource::ResourceKind, scope::ScopeKind}};

impl Transformation {
    pub fn persist(
        directory: Option<PathBuf>,
        observation: &mut Observation,
        shard: &Scope
    ) {
        let (_, scope, _shard) = observation.shards_mut().get_full_mut2(shard).expect("Shard doesn't exist");
        
        if let Some(path) = directory {
            *scope = Scope::directory(Resource::directory(path));
        }
        
        match scope.kind() {
            ScopeKind::Directory(resource) => match resource.kind() {
                ResourceKind::Directory(_path) => {
                    
                }
                _ => unreachable!()
            }
            ScopeKind::Ephemeral(_) => panic!("Cannot persist an ephemeral shard")
        }

        todo!()
    }
}