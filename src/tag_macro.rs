#[macro_export]
macro_rules! tag {
    // Match 1 type
    ($t1:ty) => {
        Tag::from(TagArrayWrapper::<1>::new([std::any::TypeId::of::<$t1>()]))
    };
    // Match 2 types
    ($t1:ty, $t2:ty) => {
        Tag::from(TagArrayWrapper::<2>::new([std::any::TypeId::of::<$t1>(), std::any::TypeId::of::<$t2>()]))
    };
    // Match 3 types
    ($t1:ty, $t2:ty, $t3:ty) => {
        Tag::from(TagArrayWrapper::<3>::new([
            std::any::TypeId::of::<$t1>(),
            std::any::TypeId::of::<$t2>(),
            std::any::TypeId::of::<$t3>(),
        ]))
    };
    // Match 4 types
    ($t1:ty, $t2:ty, $t3:ty, $t4:ty) => {
        Tag::from(TagArrayWrapper::<4>::new([
            std::any::TypeId::of::<$t1>(),
            std::any::TypeId::of::<$t2>(),
            std::any::TypeId::of::<$t3>(),
            std::any::TypeId::of::<$t4>(),
        ]))
    };
    // Match 5 types
    ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty) => {
        Tag::from(TagArrayWrapper::<5>::new([
            std::any::TypeId::of::<$t1>(),
            std::any::TypeId::of::<$t2>(),
            std::any::TypeId::of::<$t3>(),
            std::any::TypeId::of::<$t4>(),
            std::any::TypeId::of::<$t5>(),
        ]))
    };
    // Match 6 types
    ($t1:ty, $t2:ty, $t3:ty, $t4:ty, $t5:ty, $t6:ty) => {
        Tag::from(TagArrayWrapper::<6>::new([
            std::any::TypeId::of::<$t1>(),
            std::any::TypeId::of::<$t2>(),
            std::any::TypeId::of::<$t3>(),
            std::any::TypeId::of::<$t4>(),
            std::any::TypeId::of::<$t5>(),
            std::any::TypeId::of::<$t6>(),
        ]))
    };
}
