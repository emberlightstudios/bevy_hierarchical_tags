use std::any::TypeId;
use hierarchical_tags::prelude::*;

fn main() {
    struct Input;
    struct Attack;

    struct Ability;
    struct Magic;
    struct Fireball;
    struct Lightning;

    // Tags are hierarchical, you can have up to 6 levels
    let input_tag = tag!(Input);
    let attack_tag = tag!(Input, Attack);

    let ability_tag = tag!(Ability);
    let fireball_tag = tag!(Ability, Magic, Fireball);
    let lightning_tag = tag!(Ability, Magic, Lightning);

    // This is a parent of the fireball and lightning tags
    // It has the first 2 levels in common.
    let magic_tag = tag!(Ability, Magic);

    
    // tags match on themselves, obviously
    assert!(lightning_tag.matches(&lightning_tag));
    // or up through their parents
    assert!(attack_tag.matches(&input_tag));
    assert!(lightning_tag.matches(&magic_tag));
    assert!(fireball_tag.matches(&magic_tag));
    assert!(fireball_tag.matches(&ability_tag));

    // but they won't match anything else, including siblings
    assert!(!magic_tag.matches(&input_tag));
    assert!(!fireball_tag.matches(&lightning_tag));

    // TagLists let you aggregate tags.
    let mut abilities = TagList::new();
    abilities.add_tag(&lightning_tag);
    abilities.add_tag(&fireball_tag);

    // check if we have at least 1 magic tag
    assert!(abilities.any_matches(&magic_tag));
    // but we shouldn't have any input tags
    assert!(!abilities.any_matches(&input_tag));

    // You can check existence per tag
    assert!(abilities.has_tag(&lightning_tag));
    assert!(abilities.has_tag(&fireball_tag));
    
    // and iterate
    for tag in abilities.iter() {
        println!("{}", tag.matches(&fireball_tag));
    }

    // and remove tags
    abilities.remove_tag(&lightning_tag);
    abilities.remove_tag(&fireball_tag);

    // Now we have no more magic tags
    assert!(!abilities.any_matches(&magic_tag));
    // or any tags at all
    assert_eq!(abilities.len(), 0);
}
