use bevy::prelude::*;
use bitvec::prelude::*;
use smallvec::SmallVec;
use std::collections::HashMap;

/// This can be tuned to increase performance
/// If it's larger than the number of tags you avoid
/// heap allocations.  If it's smaller then you waste ram.
/// Ideally it's equal or only slightly larger than max_tags,
/// but it needs to be known at compile time.
const INLINE_NODES: usize = 128;

/// Module prelude
pub mod prelude {
    pub use crate::{TagRegistry, TagId, TagList};
}

/// Tag identifier
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Deref)]
pub struct TagId(pub u16);

/// Node representing a tag and its ancestors
pub struct TagNode {
    /// Ancestor mask (includes self)
    ancestors: BitVec,
}

/// Tag registry resource 
#[derive(Resource)]
pub struct TagRegistry {
    nodes: SmallVec<[TagNode; INLINE_NODES]>, 
    lookup: HashMap<String, TagId>,
    max_tags: usize,
}

impl TagRegistry {
    /// Create a new registry with a specified max number of tags
    pub fn new(max_tags: usize) -> Self {
        Self {
            nodes: SmallVec::new(),
            lookup: HashMap::new(),
            max_tags,
        }
    }

    /// Register a tag by its path (e.g., "Ability.Magic.Fireball")
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

            let id = TagId(self.nodes.len() as u16);
            if *id > self.max_tags as u16 {
                panic!("Cannot add more tags.  Increase max_tags parameter for TagRegistry");
            }

            // Build ancestor mask
            let mut ancestors = if let Some(p) = parent {
                self.nodes[p.0 as usize].ancestors.clone()
            } else {
                bitvec![0; self.max_tags]
            };
            ancestors.set(id.0 as usize, true);

            // Create node
            let node = TagNode {
                ancestors,
            };

            self.nodes.push(node);
            self.lookup.insert(current_path.clone(), id);
            parent = Some(id);
        }

        parent.unwrap()
    }

    /// Get the TagId for a given path
    pub fn id_of(&self, tag: impl AsRef<str>) -> Option<TagId> {
        self.lookup.get(&tag.as_ref().to_lowercase()).copied()
    }

    /// Check if descendant has ancestor in its ancestor mask
    pub fn is_match(&self, descendant: TagId, ancestor: TagId) -> bool {
        self.nodes[descendant.0 as usize].ancestors[ancestor.0 as usize]
    }

    /// Maximum number of tags
    pub fn max_tags(&self) -> usize {
        self.max_tags
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
