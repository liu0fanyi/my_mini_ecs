use std::collections::HashMap;

type ComponentId = i32;
type EntityId = i32;
type Type = Vec<ComponentId>;

fn main() {
    let entity_index: HashMap<EntityId, Type> = HashMap::new();

    has_component(entity_index, 32, 48);
}

// we have to store a vector per entity
// 每个entity需要存储一个vector
// which can be expensive if we have lots of entities
// 当有很多entities的时候，会很昂贵
// we need an O(n) search to see if an entity has a component.
// 为了找到一个component需要O(n), 这个操作在ecs里很频繁，所以很昂贵
//  especially since we would also have to do this for almost any operation, like get, add, remove, and so on.
//  而且在get add remove操作里都需要进行这个查询
fn has_component(
    entity_index: HashMap<EntityId, Type>,
    entity: EntityId,
    component: ComponentId,
) -> bool {
    let types = &entity_index[&entity];

    for c in types {
        if *c == component {
            return true;
        }
    }

    true
}
