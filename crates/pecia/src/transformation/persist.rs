use indexmap::map::MutableKeys;

use crate::{Observation, Transformation, observation::{Resource, Scope}};

impl Transformation {
    pub fn persist(
        directory: Option<Resource>,
        observation: &mut Observation,
        shard: &Scope
    ) {
        let (_, scope, _shard) = observation.shards_mut().get_full_mut2(shard).expect("Shard doesn't exist");
        
        if let Some(directory) = directory {
            *scope = Scope::Directory(directory);
        }
        
        match scope {
            Scope::Directory(resource) => match resource {
                Resource::Directory(_path) => {
                    
                }
                _ => unreachable!()
            }
            Scope::Ephemeral(_) => panic!("Cannot persist an ephemeral shard")
        }

        todo!()
    }
}