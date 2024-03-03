/*
 * 需要理解下archetype类型
 * 对于每一个component，都有单独的array用来保存
 * 对应下面的component: Vec<Column>, Column就是一种对应的Vec<component类型>
// Type [A]
A a[2];
// Type [A, B]
A a[2];
B b[2];
// Type [A, C]
A a[2];
C c[2];*/
use std::{
    any::Any,
    collections::{HashMap, HashSet},
    path::Component,
};

type EntityId = i32;
type ComponentId = i32;
type Type = Vec<ComponentId>;
type ArcheTypeId = i32;
type ArcheTypeSet = HashSet<ArcheTypeId>;
// 这个是关键，这里是当前所有包含某component的ArcheType, 然后可以通过ArcheTypeid索引到对应的component在当前ArcheType里的column
type ArcheTypeMap = HashMap<ArcheTypeId, ArchetypeRecord>;

const POSITION: &ComponentId = &32;
const VELOCITY: &ComponentId = &45;

// 对于每个entity，都对应一个record
// 这个record，保存了一种archetype的引用
//
/*
 * 需要理解当前entities的样子
 * 对于每一种ArchType，entities有自己的row，现在存在了record里面
0: [A]
1: [A]

2: [A B]
3: [A B]

4: [A C]
5: [A C]*/
// 这个row，代表着这个entity用到了archetype里的第row个component
struct Record {
    arche_type: Box<ArcheType>,
    row: usize,
}

struct ArchetypeRecord {
    column: usize,
}

// 应该是Vec<T>, 但目前没实现对T的处理，暂时用String代替
type Column = Vec<String>;
// archetype现在比较辛苦，存了所有的components的id，和components的本体, 以及自己的id
struct ArcheType {
    id: ArcheTypeId,
    // 而这里保存着那些Component的ids
    type_c: Type,
    // 这里存着多个components
    // 每种components都有各自的Vec负责保存
    // 这里的Column是行的概念，实际上是个图
    components: Vec<Column>,
}

// 现在要通过componentId获得entity里面的component
fn get_component(
    entity_index: HashMap<EntityId, &Record>,
    component_index: HashMap<ComponentId, ArcheTypeSet>,
    entity: EntityId,
    component: ComponentId,
) -> Option<&String> {
    let record = entity_index[&entity];
    let arche_type = &record.arche_type;

    // TODO: 下俩可以和一块，减少查找
    // First check if archetype has component
    let arche_types: &ArcheTypeSet = &component_index[&component];
    if !arche_types.contains(&arche_type.id) {
        return Option::None;
    }

    // 这里的搜索仍然不理想，会比较慢，而get操作非常常见
    for (index, type_id) in arche_type.type_c.iter().enumerate() {
        if *type_id == component {
            return Option::Some(&arche_type.components[index][record.row as usize]);
        }
    }

    Option::None
}

// 上边的改良
// 现在要通过componentId获得entity里面的component
fn get_component2(
    entity_index: HashMap<EntityId, &Record>,
    component_index: HashMap<ComponentId, ArcheTypeMap>,
    entity: EntityId,
    component: ComponentId,
) -> Option<&String> {
    // record, 通过entity_index+entity，索引到的
    // 里面存了这个entity存储的archetype
    let record = entity_index[&entity];
    // 拿到record里面的archetype，这就是当前entity的所有components组合
    let arche_type = &record.arche_type;

    // First check if archetype has component
    // 首先通过component_index获得一个map
    // 这个map,通过archetype的id，索引到Record，这个Record里面有个column
    let arche_types = &component_index[&component];
    if !arche_types.contains_key(&arche_type.id) {
        return Option::None;
    }

    // arche_types本来应该是通过component_index索引到的一组archeTypes的set, 现在变成了一个map，可以通过index，索引到ArchetypeRecord
    let a_record: &ArchetypeRecord = &arche_types[&arche_type.id];
    
    // 现在通过arche_type，获得对应的components
    // a_record.column里面保存了components的索引index
    // 得到components之后，可以通过record.row来获得对应位置的component
    Option::Some(&arche_type.components[a_record.column][record.row])
}

fn main() {
    let entity_index: HashMap<EntityId, &Record> = HashMap::new();
    // Find the archetype for an entity
    // let component_index: HashMap<ComponentId, ArcheTypeSet> = HashMap::new();
    
    let component_index: HashMap<ComponentId, ArcheTypeMap> = HashMap::new();

    println!("Hello, world!");
}