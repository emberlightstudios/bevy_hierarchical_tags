use bevy::prelude::*;
use bevy_hierarchical_tags::prelude::*;

#[derive(Resource)]
struct MyTagIds {
    fireball: TagId,
    lightning: TagId,
    attack: TagId,
}

fn main() {
    let mut app = App::new();
    let mut registry = TagRegistry::default();
    app.add_systems(Startup, test);

    let fireball = registry.register("Ability.Magic.Fireball");
    let lightning = registry.register("Ability.Magic.Lightning");
    let attack = registry.register("Input.Attack");

    app.insert_resource(registry);
    app.insert_resource(MyTagIds {
        fireball, lightning, attack
    });
    app.run();
}

fn test(registry: Res<TagRegistry>, tags: Res<MyTagIds>) {
    // This method should be used sparingly since hashing strings isn't exactly the cheapest. 
    // Ideally all needed tag ids would be stored in a resource somewhere
    // This is just for testing
    let magic = registry.id_of("Ability.Magic").unwrap();
    let abilities = registry.id_of("Ability").unwrap();

    assert!(registry.is_match(tags.fireball, magic));
    assert!(registry.is_match(tags.lightning, magic));
    assert!(registry.is_match(tags.fireball, abilities));
    assert!(registry.is_match(tags.lightning, abilities));
    assert!(!registry.is_match(tags.lightning, tags.fireball));
    assert!(!registry.is_match(tags.lightning, tags.attack));

    let abilities: TagList<2> = TagList::from([tags.fireball, tags.lightning]);
    let magic_tags: TagList<1> = TagList::from([magic]);
    let input_tags: TagList<2> = TagList::from([tags.attack]);

    assert!(abilities.any_match(magic, &registry));
    assert!(abilities.all_match(&magic_tags, &registry));
    assert!(abilities.none_match(tags.attack, &registry));
    assert!(abilities.none_match_from(&input_tags, &registry));

    println!("SUCCESS");
}