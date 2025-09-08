use bevy::prelude::*;
use smallvec::SmallVec;
use std::collections::HashMap;


pub mod prelude {
    pub use crate::{
        TagPlugin,
        TagRegistry,
        TagId,
        TagList,
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

    pub fn matches(&self, descendent: TagId, ancestor: TagId) -> bool {
        let mut current = Some(descendent);
        while let Some(id) = current {
            if id == ancestor {
                return true;
            }
            current = self.nodes[id.0 as usize].parent;
        }
        false
    }
}

/// Generic over inline stack size for smallvec.  You can exceed this, but it will
/// involve heap allocations.  Try to match with the number of tags you would 
/// expect for your use case.
#[derive(Deref, DerefMut, Default, Clone)]
pub struct TagList<const N: usize>(SmallVec<[TagId; N]>);

#[allow(dead_code)]
impl<const N: usize> TagList<N> {
    pub fn any_matches(&self, tag: TagId, registry: &TagRegistry) -> bool {
        self.iter().any(|existing| registry.matches(*existing, tag))
    }

    pub fn all_matches<const M: usize>(&self, tags: &TagList<M>, registry: &TagRegistry) -> bool {
        tags.iter().all(|tag| self.any_matches(*tag, registry))
    }

    pub fn from_slice(slice: &[TagId]) -> Self {
        let mut list = SmallVec::new();
        list.extend_from_slice(slice);
        TagList(list)
    }

    pub fn from_iter<I: IntoIterator<Item = TagId>>(iter: I) -> Self {
        let mut list = SmallVec::new();
        list.extend(iter);
        TagList(list)
    }
}