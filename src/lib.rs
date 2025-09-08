use bevy::prelude::*;
use std::collections::HashMap;


pub mod prelude {
    pub use crate::{
        TagPlugin,
        TagRegistry,
        TagId
    };
}

pub struct TagPlugin;

impl Plugin for TagPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TagRegistry::default());
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Deref)]
pub struct TagId(u16);

pub struct TagNode {
    //id: TagId,
    //name: String,
    parent: Option<TagId>,
}

#[derive(Resource, Default)]
pub struct TagRegistry {
    nodes: Vec<TagNode>,
    lookup: HashMap<String, TagId>,
}

impl TagRegistry {
    pub fn register(&mut self, path: &str) -> TagId {
        if let Some(&id) = self.lookup.get(path) {
            return id;
        }

        let path = path.to_lowercase();
        // Split "Ability.Magic.Fire" into ["Ability", "Magic", "Fire"]
        let parts: Vec<&str> = path.split('.').collect();
        let mut parent: Option<TagId> = None;
        let mut current_path = String::new();

        for part in parts {
            if !current_path.is_empty() {
                current_path.push('.');
            }
            current_path.push_str(part);

            // Already exists?
            if let Some(&id) = self.lookup.get(&current_path) {
                parent = Some(id);
                continue;
            }

            // Insert new node
            let id = TagId(self.nodes.len() as u16);
            self.nodes.push(TagNode {
                //id,
                //name: part.to_string(),
                parent,
            });

            self.lookup.insert(current_path.clone(), id);
            parent = Some(id);
        }

        parent.unwrap()
    }

    pub fn id_of(&self, tag: &str) -> Option<TagId> {
        self.lookup.get(&tag.to_lowercase()).copied()
    }

    pub fn matches(&self, a: TagId, b: TagId) -> bool {
        let mut current = Some(a);
        while let Some(id) = current {
            if id == b {
                return true;
            }
            current = self.nodes[id.0 as usize].parent;
        }
        false
    }
}
