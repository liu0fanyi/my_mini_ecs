use std::{alloc::Layout, any::TypeId, collections::HashMap, hash::BuildHasherDefault, ptr::NonNull};

use crate::borrow::AtomicBorrow;

/// from hecs
/// 

/// Metadata required to store a component.
///
/// All told, this means a [`TypeId`], to be able to dynamically name/check the component type; a
/// [`Layout`], so that we know how to allocate memory for this component type; and a drop function
/// which internally calls [`core::ptr::drop_in_place`] with the correct type parameter.
#[derive(Debug, Copy, Clone)]
pub struct TypeInfo {
    id: TypeId,
    layout: Layout,
    drop: unsafe fn(*mut u8),
    #[cfg(debug_assertions)]
    type_name: &'static str,
}

struct Data {
    state: AtomicBorrow,
    storage: NonNull<u8>,
}

/// 
/// A collection of entities having the same component types
///
/// Accessing `Archetype`s is only required in niche cases. Typical use should go through the
/// [`World`](crate::World).
pub struct Archetype {
    types: Vec<TypeInfo>,
    type_ids: Box<[TypeId]>,
    index: OrderedTypeIdMap<usize>,
    len: u32,
    entities: Box<[u32]>,
    /// One allocation per type, in the same order as `types`
    data: Box<[Data]>,
}

/// A hasher optimized for hashing a single TypeId.
///
/// TypeId is already thoroughly hashed, so there's no reason to hash it again.
/// Just leave the bits unchanged.
#[derive(Default)]
pub(crate) struct TypeIdHasher {
    hash: u64,
}


/// A HashMap with TypeId keys
///
/// Because TypeId is already a fully-hashed u64 (including data in the high seven bits,
/// which hashbrown needs), there is no need to hash it again. Instead, this uses the much
/// faster no-op hash.
pub(crate) type TypeIdMap<V> = HashMap<TypeId, V, BuildHasherDefault<TypeIdHasher>>;

struct OrderedTypeIdMap<V>(Box<[(TypeId, V)]>);

impl<V> OrderedTypeIdMap<V> {
    fn new(iter: impl Iterator<Item = (TypeId, V)>) -> Self {
        let mut vals = iter.collect::<Box<[_]>>();
        vals.sort_unstable_by_key(|(id, _)| *id);
        Self(vals)
    }

    fn search(&self, id: &TypeId) -> Option<usize> {
        self.0.binary_search_by_key(id, |(id, _)| *id).ok()
    }

    fn contains_key(&self, id: &TypeId) -> bool {
        self.search(id).is_some()
    }

    fn get(&self, id: &TypeId) -> Option<&V> {
        self.search(id).map(move |idx| &self.0[idx].1)
    }
}

