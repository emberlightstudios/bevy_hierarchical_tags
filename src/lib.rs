mod tag_macro;
use std::any::TypeId;
use smallvec::SmallVec;

const TAG_LIST_SIZE: usize = 4;
const TAG_SIZE: usize = 4;

pub mod prelude {
    pub use crate::{tag, TagList, Tag, TagArrayWrapper};
}

#[derive(PartialEq, Copy, Clone)]
pub struct TagArrayWrapper<const N: usize>([TypeId; N]);

impl<const N: usize> TagArrayWrapper<N> {
    pub fn new(ids: [TypeId; N]) -> Self {
        Self(ids)
    }

    #[allow(dead_code)]
    pub(crate) fn matches<const M: usize>(&self, other: &TagArrayWrapper<M>) -> bool {
        if M > N { return false; }
        self.0[..M] == other.0[..M]
    }

    #[allow(dead_code)]
    pub(crate) fn len(&self) -> usize { N }
}

// So we can aggregate tags in a list, wrap in an enum
#[derive(PartialEq, Copy, Clone)]
pub enum Tag {
    Len1(TagArrayWrapper<1>),
    Len2(TagArrayWrapper<2>),
    Len3(TagArrayWrapper<3>),
    Len4(TagArrayWrapper<4>),
    //Len5(TagArrayWrapper<5>),
    //Len6(TagArrayWrapper<6>),
}


impl From<TagArrayWrapper<1>> for Tag {
    fn from(value: TagArrayWrapper<1>) -> Self {
        Tag::Len1(value)
    }
}
impl From<TagArrayWrapper<2>> for Tag {
    fn from(value: TagArrayWrapper<2>) -> Self {
        Tag::Len2(value)
    }
}
impl From<TagArrayWrapper<3>> for Tag {
    fn from(value: TagArrayWrapper<3>) -> Self {
        Tag::Len3(value)
    }
}
impl From<TagArrayWrapper<4>> for Tag {
    fn from(value: TagArrayWrapper<4>) -> Self {
        Tag::Len4(value)
    }
}
//impl From<TagArrayWrapper<5>> for Tag {
//    fn from(value: TagArrayWrapper<5>) -> Self {
//        Tag::Len5(value)
//    }
//}
//impl From<TagArrayWrapper<6>> for Tag {
//    fn from(value: TagArrayWrapper<6>) -> Self {
//        Tag::Len6(value)
//    }
//}

impl Tag {
    fn as_slice(&self) -> &[TypeId] {
        match self {
            Tag::Len1(tag) => &tag.0,
            Tag::Len2(tag) => &tag.0,
            Tag::Len3(tag) => &tag.0,
            Tag::Len4(tag) => &tag.0,
            //Tag::Len5(tag) => &tag.0,
            //Tag::Len6(tag) => &tag.0,
        }
    }

    pub fn matches(&self, other: &Tag) -> bool {
        let self_slice = self.as_slice();
        let other_slice = other.as_slice();
        if other_slice.len() > self_slice.len() { return false; }
        self_slice[..other_slice.len()] == *other_slice
    }

    pub fn len(&self) -> usize { self.as_slice().len() }

    pub fn join<T: 'static>(&self) -> Option<Tag> {
        let mut new_ids = [TypeId::of::<()>(); TAG_SIZE];
        let len = self.len();

        if len >= TAG_SIZE {
            return None; 
        }

        new_ids[..len].copy_from_slice(self.as_slice());

        new_ids[len] = TypeId::of::<T>();
        let new_len = len + 1;

        let result = match new_len {
            1 => Tag::Len1(TagArrayWrapper::new([new_ids[0]])),
            2 => Tag::Len2(TagArrayWrapper::new([new_ids[0], new_ids[1]])),
            3 => Tag::Len3(TagArrayWrapper::new([new_ids[0], new_ids[1], new_ids[2]])),
            4 => Tag::Len4(TagArrayWrapper::new([new_ids[0], new_ids[1], new_ids[2], new_ids[3]])),
            //5 => Tag::Len5(TagArrayWrapper::new([new_ids[0], new_ids[1], new_ids[2], new_ids[3], new_ids[4]])),
            //6 => Tag::Len6(TagArrayWrapper::new([new_ids[0], new_ids[1], new_ids[2], new_ids[3], new_ids[4], new_ids[5]])),
            _ => unreachable!(),
        };

        Some(result)
    }

    pub fn parent(&self) -> Option<Tag> {
        if self.len() == 1 {
            None
        } else {
            let mut new_ids = [TypeId::of::<()>(); TAG_SIZE];
            new_ids[..self.len()].copy_from_slice(&self.as_slice());
            new_ids[self.len()] = TypeId::of::<()>();
            let new_len = self.len() - 1;
            let result = match new_len {
                1 => Tag::Len1(TagArrayWrapper::new([new_ids[0]])),
                2 => Tag::Len2(TagArrayWrapper::new([new_ids[0], new_ids[1]])),
                3 => Tag::Len3(TagArrayWrapper::new([new_ids[0], new_ids[1], new_ids[2]])),
                //4 => Tag::Len4(TagArrayWrapper::new([new_ids[0], new_ids[1], new_ids[2], new_ids[3]])),
                //5 => Tag::Len5(TagArrayWrapper::new([new_ids[0], new_ids[1], new_ids[2], new_ids[3], new_ids[4]])),
                _ => unreachable!(),
            };
            Some(result)
        }
    }
}

#[derive(Default, Clone)]
pub struct TagList(SmallVec<[Tag; TAG_LIST_SIZE]>);

impl TagList {
    pub fn new() -> Self { Self::default() }
    
    pub fn has_tag(&self, tag: &Tag) -> bool {
        self.0.contains(tag)
    }

    pub fn add_tag(&mut self, tag: &Tag) -> bool {
        if self.has_tag(tag) {
            false
        } else {
            self.0.push(*tag);
            true
        }
    }

    pub fn remove_tag(&mut self, tag: &Tag) -> bool {
        if let Some(idx) = self.0.iter().position(|x| x == tag) {
            self.0.remove(idx);
            true
        } else { false }
    }

    pub fn any_matches(&self, tag: &Tag) -> bool {
        self.0.iter().any(|stored_tag| stored_tag.matches(&tag))
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Tag> {
        self.0.iter()
    }

    pub fn len(&self) -> usize { self.0.len() }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Input;
    struct Ability;
    struct Magic;
    struct Fire;

    #[test]
    fn test_tag_enum() {
        let tag1: Tag = tag!(Ability);
        let tag2: Tag = tag!(Ability, Magic);
        let tag3: Tag = tag!(Ability, Magic, Fire);

        assert!(tag3.matches(&tag2));
        assert!(tag3.matches(&tag1));
        assert!(!tag2.matches(&tag3));
        assert!(!tag1.matches(&tag2));
        assert!(tag2.join::<Fire>().unwrap().matches(&tag3));

        assert_eq!(tag1.len(), 1);
        assert_eq!(tag2.len(), 2);
        assert_eq!(tag3.len(), 3);

        let mut tags = TagList::new();
        tags.add_tag(&tag!(Input));
        tags.add_tag(&tag!(Fire));
        assert!(!tags.any_matches(&tag!(Magic)));
    }
}
