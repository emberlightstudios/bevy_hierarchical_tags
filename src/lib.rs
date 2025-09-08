use bevy::prelude::*;
use smallbitvec::SmallBitVec;
use smallvec::SmallVec;
use std::collections::HashMap;

const N_TAGS: usize = 128;

pub mod prelude {
    pub use crate::{TagRegistry, TagId, TagList};
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Deref)]
pub struct TagId(pub u16);

pub struct TagNode {
    /// Bitmask of all ancestors (including itself)
    ancestors: SmallBitVec,
}

#[derive(Resource, Default)]
pub struct TagRegistry {
    nodes: Vec<TagNode>,
    lookup: HashMap<String, TagId>,
}

impl TagRegistry {
    pub fn register(&mut self, path: impl AsRef<str>) -> TagId {
        let path = path.as_ref();
        if let Some(&id) = self.lookup.get(path) {
            return id;
        }

        let path = path.to_lowercase();
        let parts: Vec<&str> = path.split('.').collect();
        let mut parent: Option<TagId> = None;
        let mut current_path = String::new();

        for part in parts {
            if !current_path.is_empty() {
                current_path.push('.');
            }
            current_path.push_str(part);

            if let Some(&id) = self.lookup.get(&current_path) {
                parent = Some(id);
                continue;
            }

            // assign new id
            let id = TagId(self.nodes.len() as u16);

            // build ancestor mask
            let mut ancestors: SmallBitVec;
            if let Some(p) = parent {
                ancestors = self.nodes[p.0 as usize].ancestors.clone();
            } else {
                ancestors = SmallBitVec::from_elem(N_TAGS, false);
            }
            ancestors.set(id.0 as usize, true); // mark itself

            self.nodes.push(TagNode { ancestors });
            self.lookup.insert(current_path.clone(), id);
            parent = Some(id);
        }

        parent.unwrap()
    }

    pub fn id_of(&self, tag: impl AsRef<str>) -> Option<TagId> {
        self.lookup.get(&tag.as_ref().to_lowercase()).copied()
    }

    pub fn is_match(&self, descendant: TagId, ancestor: TagId) -> bool {
        self.nodes[descendant.0 as usize].ancestors[ancestor.0 as usize]
    }
}

/// A list of tag ids. Uses SmallVec for compact storage of small lists.
#[derive(Deref, DerefMut, Default, Clone)]
pub struct TagList<const N: usize>(SmallVec<[TagId; N]>);

impl<const N: usize> TagList<N> {
    pub fn any_match(&self, tag: TagId, registry: &TagRegistry) -> bool {
        self.iter().any(|existing| registry.is_match(*existing, tag))
    }

    pub fn none_match(&self, tag: TagId, registry: &TagRegistry) -> bool {
        !self.iter().any(|existing| registry.is_match(*existing, tag))
    }

    pub fn none_match_from<const M: usize>(&self, tags: &TagList<M>, registry: &TagRegistry) -> bool {
        tags.iter().all(|tag| !self.any_match(*tag, registry))
    }

    pub fn all_match_from<const M: usize>(&self, tags: &TagList<M>, registry: &TagRegistry) -> bool {
        tags.iter().all(|tag| self.any_match(*tag, registry))
    }
}

impl<const N: usize, I: IntoIterator<Item = TagId>> From<I> for TagList<N> {
    fn from(iter: I) -> Self {
        let mut list = SmallVec::new();
        list.extend(iter);
        TagList(list)
    }
}
