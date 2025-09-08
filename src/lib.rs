use bevy::prelude::*;
use bitvec::prelude::*;
use smallvec::SmallVec;
use std::collections::HashMap;

pub mod prelude {
    pub use crate::{TagId, TagRegistry, TagList};
}

/// Tag identifier
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Deref)]
pub struct TagId(pub u16);

/// Node representing a tag and its ancestors
#[derive(Clone)]
pub struct TagNode {
    ancestors: BitVec,
}

/// Tag registry with a fixed inline array
#[derive(Resource)]
pub struct TagRegistry<const INLINE_NODES: usize> {
    nodes: [Option<TagNode>; INLINE_NODES],
    lookup: HashMap<String, TagId>,
    len: usize, // number of tags currently registered
}

impl<const INLINE_NODES: usize> TagRegistry<INLINE_NODES> {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self {
            nodes: std::array::from_fn(|_| None),
            lookup: HashMap::new(),
            len: 0,
        }
    }

    /// Register a tag by path (e.g., "Ability.Magic.Fireball")
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

            if self.len >= INLINE_NODES {
                panic!("Cannot add more tags. Increase INLINE_NODES generic parameter to TagRegistry.");
            }

            let id = TagId(self.len as u16);

            // Build ancestor mask
            let mut ancestors = if let Some(p) = parent {
                self.nodes[*p as usize].as_ref().unwrap().ancestors.clone()
            } else {
                bitvec![0; INLINE_NODES]
            };
            ancestors.set(*id as usize, true);

            self.nodes[self.len] = Some(TagNode { ancestors });
            self.lookup.insert(current_path.clone(), id);
            self.len += 1;
            parent = Some(id);
        }

        parent.unwrap()
    }

    /// Get TagId by path
    pub fn id_of(&self, tag: impl AsRef<str>) -> Option<TagId> {
        self.lookup.get(&tag.as_ref().to_lowercase()).copied()
    }

    /// Check if descendant has ancestor in its ancestor mask
    pub fn is_match(&self, descendant: TagId, ancestor: TagId) -> bool {
        self.nodes[*descendant as usize]
            .as_ref()
            .unwrap()
            .ancestors[*ancestor as usize]
    }
}

/// A list of tag ids. Uses SmallVec for compact storage of small lists.
#[derive(Deref, DerefMut, Clone)]
pub struct TagList<const N: usize>(SmallVec<[TagId; N]>);

impl<const N: usize> TagList<N> {
    pub fn any_match<const R: usize>(&self, tag: TagId, registry: &TagRegistry<R>) -> bool {
        self.iter().any(|existing| registry.is_match(*existing, tag))
    }

    pub fn none_match<const R: usize>(&self, tag: TagId, registry: &TagRegistry<R>) -> bool {
        !self.any_match(tag, registry)
    }

    pub fn none_match_from<const R: usize, const M: usize>(&self, tags: &TagList<M>, registry: &TagRegistry<R>) -> bool {
        tags.iter().all(|tag| !self.any_match(*tag, registry))
    }

    pub fn all_match_from<const R: usize, const M: usize>(&self, tags: &TagList<M>, registry: &TagRegistry<R>) -> bool {
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
