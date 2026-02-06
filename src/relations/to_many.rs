//! ToMany relation implementation
//!
//! A ToMany relation references multiple objects of a target entity.
//! Uses lazy initialization - the target objects are only read from
//! the database when first accessed.

use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::marker::PhantomData;

use crate::c::obx_id;
use crate::traits::OBBlanket;

use super::info::RelInfo;

/// A to-many relation that references multiple objects of a target entity.
///
/// # Example
///
/// ```rust,ignore
/// use objectbox::relations::ToMany;
///
/// #[entity]
/// struct Student {
///     #[id]
///     id: u64,
///     teachers: ToMany<Teacher>,
/// }
///
/// // Add to relation
/// student.teachers.add(teacher1);
/// student.teachers.add(teacher2);
/// box_.put(&mut student)?;
///
/// // Remove from relation
/// student.teachers.remove(&teacher1);
/// box_.put(&mut student)?;
/// ```
///
/// The target objects are referenced by their IDs, which are persisted
/// as part of the standalone relation in a separate internal table.
///
/// Note: The generic parameter T does NOT require OBBlanket on the struct itself
/// to avoid circular dependency issues during entity definition. The bound is
/// only required on methods that actually need it.
pub struct ToMany<T> {
    /// Target entity type marker
    _marker: PhantomData<T>,
    /// Relation info (set when attached to store)
    rel_info: RefCell<Option<RelInfo>>,
    /// Loaded items (lazy loaded)
    items: RefCell<Option<Vec<T>>>,
    /// Track changes: positive count = added, negative = removed
    changes: RefCell<HashMap<obx_id, i32>>,
    /// Items added before lazy loading
    added_before_load: RefCell<Vec<T>>,
}

// Debug doesn't require OBBlanket
impl<T> fmt::Debug for ToMany<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let items_loaded = self.items.borrow().is_some();
        let pending_changes = !self.changes.borrow().is_empty();
        f.debug_struct("ToMany")
            .field("items_loaded", &items_loaded)
            .field("pending_changes", &pending_changes)
            .finish()
    }
}

impl<T> Default for ToMany<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Clone for ToMany<T> {
    fn clone(&self) -> Self {
        ToMany {
            _marker: PhantomData,
            rel_info: RefCell::new(self.rel_info.borrow().clone()),
            items: RefCell::new(None), // Don't clone items
            changes: RefCell::new(HashMap::new()),
            added_before_load: RefCell::new(Vec::new()),
        }
    }
}

// Safety: ToMany is Send if T is Send
unsafe impl<T: Send> Send for ToMany<T> {}

/// Core methods that don't require OBBlanket
impl<T> ToMany<T> {
    /// Create a new empty ToMany relation.
    pub fn new() -> Self {
        ToMany {
            _marker: PhantomData,
            rel_info: RefCell::new(None),
            items: RefCell::new(None),
            changes: RefCell::new(HashMap::new()),
            added_before_load: RefCell::new(Vec::new()),
        }
    }

    /// Track add/remove operations.
    /// Increment = 1 for add, -1 for remove.
    fn track(&self, id: obx_id, increment: i32) {
        if id == 0 {
            return; // Don't track unsaved objects by ID
        }
        
        let mut changes = self.changes.borrow_mut();
        *changes.entry(id).or_insert(0) += increment;
    }

    /// Check if there are pending changes to save to the database.
    pub fn has_pending_changes(&self) -> bool {
        self.changes.borrow().values().any(|&count| count != 0)
    }

    /// Get the pending changes (added and removed IDs).
    ///
    /// Returns (added_ids, removed_ids)
    pub fn get_pending_changes(&self) -> (Vec<obx_id>, Vec<obx_id>) {
        let changes = self.changes.borrow();
        let mut added = Vec::new();
        let mut removed = Vec::new();
        
        for (&id, &count) in changes.iter() {
            if count > 0 {
                added.push(id);
            } else if count < 0 {
                removed.push(id);
            }
        }
        
        (added, removed)
    }

    /// Clear pending changes after they have been applied.
    pub(crate) fn clear_pending_changes(&self) {
        self.changes.borrow_mut().clear();
        self.added_before_load.borrow_mut().clear();
    }

    /// Set the relation info (called when attached to store).
    pub(crate) fn set_rel_info(&self, info: RelInfo) {
        *self.rel_info.borrow_mut() = Some(info);
    }

    /// Get the relation info.
    pub(crate) fn get_rel_info(&self) -> Option<RelInfo> {
        self.rel_info.borrow().clone()
    }
}

/// Methods that require OBBlanket (entity operations)
impl<T: OBBlanket> ToMany<T> {
    /// Create a ToMany with initial items.
    ///
    /// This bypasses lazy loading - useful when creating objects
    /// from external sources (e.g., JSON).
    pub fn with_items(items: Vec<T>) -> Self {
        let changes: HashMap<obx_id, i32> = items
            .iter()
            .map(|item| (item.get_id(), 1))
            .collect();
        
        ToMany {
            _marker: PhantomData,
            rel_info: RefCell::new(None),
            items: RefCell::new(Some(items)),
            changes: RefCell::new(changes),
            added_before_load: RefCell::new(Vec::new()),
        }
    }

    /// Get the number of items in this relation.
    ///
    /// Note: This triggers lazy loading if items haven't been loaded yet.
    pub fn len(&self) -> usize {
        if let Some(ref items) = *self.items.borrow() {
            items.len()
        } else {
            self.added_before_load.borrow().len()
        }
    }

    /// Check if the relation is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Add a target object to this relation.
    ///
    /// If the object is new (ID = 0), it will be put when the
    /// owning object is put.
    pub fn add(&self, item: T) {
        let id = item.get_id();
        self.track(id, 1);
        
        if self.items.borrow().is_none() {
            // Don't load from DB just to add
            self.added_before_load.borrow_mut().push(item);
        } else {
            self.items.borrow_mut().as_mut().unwrap().push(item);
        }
    }

    /// Add multiple target objects to this relation.
    pub fn add_all(&self, items: impl IntoIterator<Item = T>) {
        for item in items {
            self.add(item);
        }
    }

    /// Remove a target object from this relation by ID.
    ///
    /// Returns true if the item was found and removed.
    pub fn remove_by_id(&self, id: obx_id) -> bool {
        if id == 0 {
            return false;
        }
        
        let mut found = false;
        
        if let Some(ref mut items) = *self.items.borrow_mut() {
            if let Some(pos) = items.iter().position(|item| item.get_id() == id) {
                items.remove(pos);
                found = true;
            }
        }
        
        // Also check added_before_load
        {
            let mut added = self.added_before_load.borrow_mut();
            if let Some(pos) = added.iter().position(|item| item.get_id() == id) {
                added.remove(pos);
                found = true;
            }
        }
        
        if found {
            self.track(id, -1);
        }
        
        found
    }

    /// Clear all items from this relation.
    pub fn clear(&self) {
        // Track removals for all current items
        if let Some(ref items) = *self.items.borrow() {
            for item in items {
                self.track(item.get_id(), -1);
            }
        }
        for item in self.added_before_load.borrow().iter() {
            self.track(item.get_id(), -1);
        }
        
        *self.items.borrow_mut() = Some(Vec::new());
        self.added_before_load.borrow_mut().clear();
    }

    /// Get items that need to be put (new objects with ID = 0).
    pub(crate) fn get_items_to_put(&self) -> Vec<&T> {
        let mut to_put = Vec::new();
        
        if let Some(ref items) = *self.items.borrow() {
            for item in items {
                if item.get_id() == 0 {
                    // Safety: We're returning references that will be valid
                    // as long as the ToMany is borrowed
                    to_put.push(unsafe { &*(item as *const T) });
                }
            }
        }
        
        for item in self.added_before_load.borrow().iter() {
            if item.get_id() == 0 {
                to_put.push(unsafe { &*(item as *const T) });
            }
        }
        
        to_put
    }

    /// Set loaded items (called after lazy loading).
    pub(crate) fn set_items(&self, mut items: Vec<T>) {
        // Merge with items added before load
        let mut added = self.added_before_load.borrow_mut();
        items.append(&mut added);
        *self.items.borrow_mut() = Some(items);
    }

    /// Get all item IDs (for items that have been stored).
    pub fn get_ids(&self) -> Vec<obx_id> {
        let mut ids = Vec::new();
        
        if let Some(ref items) = *self.items.borrow() {
            for item in items {
                let id = item.get_id();
                if id != 0 {
                    ids.push(id);
                }
            }
        }
        
        for item in self.added_before_load.borrow().iter() {
            let id = item.get_id();
            if id != 0 {
                ids.push(id);
            }
        }
        
        ids
    }

    /// Iterate over items in this relation.
    ///
    /// Note: This requires items to be loaded.
    pub fn iter(&self) -> impl Iterator<Item = &T> + '_ {
        ToManyIter {
            relation: self,
            index: 0,
        }
    }
}

struct ToManyIter<'a, T: OBBlanket> {
    relation: &'a ToMany<T>,
    index: usize,
}

impl<'a, T: OBBlanket> Iterator for ToManyIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        // First iterate over loaded items
        let items_ref = self.relation.items.borrow();
        if let Some(ref items) = *items_ref {
            if self.index < items.len() {
                let item = &items[self.index];
                self.index += 1;
                // Safety: The item lives as long as the ToMany
                return Some(unsafe { &*(item as *const T) });
            }
        }
        
        // Then iterate over added_before_load
        let added_ref = self.relation.added_before_load.borrow();
        let items_len = items_ref.as_ref().map(|v| v.len()).unwrap_or(0);
        let added_index = self.index - items_len;
        
        if added_index < added_ref.len() {
            let item = &added_ref[added_index];
            self.index += 1;
            return Some(unsafe { &*(item as *const T) });
        }
        
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::c;
    use crate::traits::{FBOBBridge, IdExt};
    use flatbuffers::FlatBufferBuilder;

    // Test entity for ToMany
    #[derive(Clone)]
    struct TestEntity {
        id: obx_id,
    }

    impl FBOBBridge for TestEntity {
        fn flatten(&self, _builder: &mut FlatBufferBuilder<'_>) {}
    }

    impl IdExt for TestEntity {
        fn get_id(&self) -> c::obx_id {
            self.id
        }
        fn set_id(&mut self, id: c::obx_id) {
            self.id = id;
        }
    }

    #[test]
    fn test_to_many_new() {
        let rel: ToMany<TestEntity> = ToMany::new();
        assert!(rel.is_empty());
        assert!(!rel.has_pending_changes());
    }

    #[test]
    fn test_to_many_add() {
        let rel: ToMany<TestEntity> = ToMany::new();
        let entity = TestEntity { id: 42 };
        rel.add(entity);
        
        assert!(!rel.is_empty());
        assert_eq!(rel.len(), 1);
        assert!(rel.has_pending_changes());
        
        let (added, removed) = rel.get_pending_changes();
        assert_eq!(added, vec![42]);
        assert!(removed.is_empty());
    }

    #[test]
    fn test_to_many_remove() {
        let rel: ToMany<TestEntity> = ToMany::new();
        let entity = TestEntity { id: 42 };
        rel.add(entity);
        
        assert!(rel.remove_by_id(42));
        assert!(rel.is_empty());
        
        // After add and remove, the net change should be 0
        assert!(!rel.has_pending_changes());
    }

    #[test]
    fn test_to_many_with_items() {
        let items = vec![
            TestEntity { id: 1 },
            TestEntity { id: 2 },
            TestEntity { id: 3 },
        ];
        let rel: ToMany<TestEntity> = ToMany::with_items(items);
        
        assert_eq!(rel.len(), 3);
        assert!(rel.has_pending_changes());
        
        let ids = rel.get_ids();
        assert_eq!(ids.len(), 3);
    }

    #[test]
    fn test_to_many_clear_pending() {
        let rel: ToMany<TestEntity> = ToMany::new();
        rel.add(TestEntity { id: 1 });
        rel.add(TestEntity { id: 2 });
        
        assert!(rel.has_pending_changes());
        rel.clear_pending_changes();
        assert!(!rel.has_pending_changes());
    }
}
