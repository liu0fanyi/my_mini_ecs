use std::collections::{HashMap, HashSet};

type EntityId = i32;
type ComponentId = i32;
type Type = Vec<ComponentId>;
type ArcheTypeId = i32;
// 这里应该说明下
// 存储的是set of archetypeid
// 经过下面的那种查询archetypes的操作，可以大概猜测这里存储的Archetype被认为是拥有某种component的archetype集合, 比如Position的component与其他的component组合出来的一套archtype,需要用一个这样的set来存储
// 缺少这个存储过程，因而有点懵逼
type ArcheTypeSet = HashSet<ArcheTypeId>;
const POSITION: &ComponentId = &32;
const VELOCITY: &ComponentId = &45;

// instead of having a set per archetype with component ids, we’ll have a set per component that has archetype ids.
// 相对于每个archetype有一个component ids的set，换成每个component有一个archetype ids的set, 因为components会比archetype少很多，archetype相当于components的排列组合

// ArcheType因此, 改为存储Id和vec<cIds>
struct ArcheType {
    id: ArcheTypeId,
    type_c: Type,
}

fn main() {
    // Find an archetype by its list of component ids
    let archetype_index: HashMap<Type, ArcheType> = HashMap::new();

    // Find the archetype for an entity
    let entity_index: HashMap<EntityId, &ArcheType> = HashMap::new();
    // Find the archetype for an entity
    let component_index: HashMap<ComponentId, ArcheTypeSet> = HashMap::new();

    hash_component(entity_index, &component_index, 35, 45);

    // 现在可以通过component找到所有的archetypes
    let position_archetypes: &ArcheTypeSet = &component_index[POSITION];
    let velocity_archetypes: &ArcheTypeSet = &component_index[VELOCITY];

    for archetype in position_archetypes {
        if velocity_archetypes.contains(archetype) {
            // archetype has Position and Velocity
        }
    }
}

fn hash_component(
    entity_index: HashMap<EntityId, &ArcheType>,
    component_index: &HashMap<ComponentId, ArcheTypeSet>,
    entity: EntityId,
    component: ComponentId,
) -> bool {
    // 现在需要先通过Entity来找到ArcheType
    let arche_type = &entity_index[&entity];
    // 然后通过component来找到archetype_set
    let archetype_set = &component_index[&component];
    // 之后判断set是否包含archetype
    archetype_set.contains(&arche_type.id)
}
