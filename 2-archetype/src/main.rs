use std::collections::{HashMap, HashSet};

type EntityId = i32;
type ComponentId = i32;
// type vector is sorted
// 这个type vector需要排序，保证无论怎么插入，position 和health的顺序是不变的
type Type = Vec<ComponentId>;

// it does add quite a bit of overhead to each archetype
// 由于添加了hashset导致archetype更加臃肿
// because in our ECS we will end up with an archetype for each unique combination of components. Entities with components Position, Health will end up in a different archetype than entities with Position, Velocity, Health
// ArcheType会非常多，Potision, Health和Position, Health, Volocity是两种不同的ArcheType
struct ArcheType {
    type_c: Type,
    // 添加一个hash_set, 判断has_component的时候使用
    type_set: HashSet<ComponentId>,
}

fn main() {
    let entity_index: HashMap<EntityId, &ArcheType> = HashMap::new();

    // 为了通过Type找到ArcheType, 嗯嗯嗯
    //
    // Whenever we’d want to find an archetype we would create a type vector which has the component ids we want, in sorted order, and then use this vector with this map which would then hash the vector
    // 如果要找一个archetype，创建一个type vector，里面装满了component ids，并且排序，然后hash map这个vector到一个新的archetype
    let archetype_index: HashMap<Type, ArcheType> = HashMap::new();
}

fn has_component(
    entity_index: HashMap<EntityId, ArcheType>,
    entity: EntityId,
    component: ComponentId,
) -> bool {
    let arche_type: &ArcheType = &entity_index[&entity];

    arche_type.type_set.contains(&component)
}


