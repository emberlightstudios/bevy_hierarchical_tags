use bevy::prelude::*;
use smallvec::SmallVec;
use std::collections::HashMap;

pub mod prelude {
    pub use crate::{TagId, TagRegistry, TagList};
}

/// Default maximum number of tags (512)
#[cfg(not(feature = "tagmax_1024"))]
pub const MAX_TAGS: usize = 512;

/// Optional extended maximum (1024)
#[cfg(feature = "tagmax_1024")]
pub const MAX_TAGS: usize = 1024;

const NUM_WORDS: usize = (MAX_TAGS + 63) / 64;


/// Tag identifier
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Deref)]
pub struct TagId(u16);

impl From<u16> for TagId {
    fn from(value: u16) -> Self {
        TagId(value)
    }
}

/// Node representing a tag and its ancestors
#[derive(Clone, Copy, Debug)]
pub struct TagNode {
    // Each bit represents an ancestor; TAGMAX bits max
    ancestors: [u64; NUM_WORDS],
}

impl TagNode {
    fn empty() -> Self {
        Self { ancestors: [0; NUM_WORDS] }
    }

    fn set_bit(&mut self, idx: usize) {
        let word = idx / 64;
        let bit = idx % 64;
        self.ancestors[word] |= 1 << bit;
    }

    fn get_bit(&self, idx: usize) -> bool {
        let word = idx / 64;
        let bit = idx % 64;
        (self.ancestors[word] & (1 << bit)) != 0
    }
}

/// Tag registry with fixed inline storage
#[derive(Resource, Clone)]
pub struct TagRegistry {
    nodes: [Option<TagNode>; MAX_TAGS],
    lookup: HashMap<String, TagId>,
    len: usize, // number of tags currently registered
}

impl TagRegistry {
    pub fn new() -> Self {
        Self {
            nodes: std::array::from_fn(|_| None),
            lookup: HashMap::new(),
            len: 0,
        }
    }

    /// Register a tag path like "Ability.Magic.Fireball"
    pub fn register(&mut self, path: impl AsRef<str>) -> TagId {
        let path = path.as_ref().to_lowercase();
        if let Some(&id) = self.lookup.get(&path) {
            return id;
        }

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

            if self.len >= MAX_TAGS {
                panic!("Exceeded TAGMAX in TagRegistry");
            }

            let id = TagId(self.len as u16);

            // Build ancestor mask
            let mut ancestors = if let Some(p) = parent {
                self.nodes[*p as usize].unwrap()
            } else {
                TagNode::empty()
            };
            ancestors.set_bit(*id as usize);

            self.nodes[self.len] = Some(ancestors);
            self.lookup.insert(current_path.clone(), id);
            self.len += 1;
            parent = Some(id);
        }

        parent.unwrap()
    }

    /// Get TagId by string
    pub fn id_of(&self, tag: impl AsRef<str>) -> Option<TagId> {
        self.lookup.get(&tag.as_ref().to_lowercase()).copied()
    }

    /// Check if descendant has ancestor in its mask
    #[inline(always)]
    pub fn is_match(&self, descendant: TagId, ancestor: TagId) -> bool {
        self.nodes[*descendant as usize]
            .unwrap()
            .get_bit(*ancestor as usize)
    }

    /// Returns true if `tag` matches *any* tag in `list`
    pub fn any_match<const N: usize>(&self, list: &TagList<N>, tag: TagId) -> bool {
        list.iter().any(|&existing| self.is_match(existing, tag))
    }

    /// Returns true if `tag` matches *none* of the tags in `list`
    pub fn none_match<const N: usize>(&self, list: &TagList<N>, tag: TagId) -> bool {
        !self.any_match(list, tag)
    }

    /// Returns true if *any* tag in `tags` matches any tag in `list`
    pub fn any_match_from<const N: usize, const M: usize>(
        &self,
        list: &TagList<N>,
        tags: &TagList<M>,
    ) -> bool {
        tags.iter().any(|&tag| self.any_match(list, tag))
    }

    /// Returns true if *none* of the tags in `tags` match any tag in `list`
    pub fn none_match_from<const N: usize, const M: usize>(
        &self,
        list: &TagList<N>,
        tags: &TagList<M>,
    ) -> bool {
        !self.any_match_from(list, tags)
    }

    /// Returns true if *all* tags in `tags` match some tag in `list`
    pub fn all_match_from<const N: usize, const M: usize>(
        &self,
        list: &TagList<N>,
        tags: &TagList<M>,
    ) -> bool {
        tags.iter().all(|&tag| self.any_match(list, tag))
    }
}

/// A list of tag ids. Uses SmallVec for small inline lists
#[derive(Deref, DerefMut, Clone, Default)]
pub struct TagList<const N: usize>(SmallVec<[TagId; N]>);

impl<const N: usize> TagList<N> {
    pub fn any_match(&self, tag: TagId, registry: &TagRegistry) -> bool {
        registry.any_match(self, tag)
    }

    pub fn none_match(&self, tag: TagId, registry: &TagRegistry) -> bool {
        registry.none_match(self, tag)
    }

    pub fn any_match_from<const M: usize>(&self, tags: &TagList<M>, registry: &TagRegistry) -> bool {
        registry.any_match_from(self, tags)
    }

    pub fn none_match_from<const M: usize>(&self, tags: &TagList<M>, registry: &TagRegistry) -> bool {
        registry.none_match_from(self, tags)
    }

    pub fn all_match_from<const M: usize>(&self, tags: &TagList<M>, registry: &TagRegistry) -> bool {
        registry.all_match_from(self, tags)
    }
}

impl<const N: usize, I: IntoIterator<Item = TagId>> From<I> for TagList<N> {
    fn from(iter: I) -> Self {
        let mut list = SmallVec::new();
        list.extend(iter);
        TagList(list)
    }
}
