//! ToOne relation implementation
//!
//! A ToOne relation references one object of a target entity.
//! Uses lazy initialization - the target object is only read from
//! the database when first accessed.

use std::marker::PhantomData;
use std::cell::Cell;
use std::fmt;

use crate::c::obx_id;
use crate::traits::OBBlanket;

/// Internal state of a ToOne relation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ToOneState {
    /// No target (null reference)
    None,
    /// Target set but not yet stored in DB (new object)
    Unstored,
    /// Target ID set, object not yet loaded (lazy)
    Lazy,
    /// Target loaded and stored
    Stored,
    /// ID was set but object not found in DB
    #[allow(dead_code)]
    Unresolvable,
}

/// A to-one relation that references one object of a target entity.
///
/// # Example
///
/// ```rust,ignore
/// use objectbox::relations::ToOne;
///
/// #[entity]
/// struct Order {
///     #[id]
///     id: u64,
///     customer: ToOne<Customer>,
/// }
///
/// // Create relation
/// let mut order = Order::default();
/// order.customer.set_target(customer);
/// box_.put(&mut order)?;
///
/// // Or set by ID
/// order.customer.set_target_id(customer_id);
/// box_.put(&mut order)?;
///
/// // Remove relation
/// order.customer.clear();
/// box_.put(&mut order)?;
/// ```
///
/// The target object is referenced by its ID. This `target_id` is persisted
/// as part of the owning object in a special property (e.g., "customerId").
/// 
/// Note: The generic parameter T does NOT require OBBlanket on the struct itself
/// to avoid circular dependency issues during entity definition. The bound is
/// only required on methods that actually need it.
pub struct ToOne<T> {
    /// The target entity type marker
    _marker: PhantomData<T>,
    /// The target ID (0 means no relation)
    target_id: Cell<obx_id>,
    /// Current state
    state: Cell<ToOneState>,
    /// Cached target object (only used when state is Stored or Unstored)
    target: Cell<Option<*const T>>,
}

// Debug doesn't require OBBlanket
impl<T> fmt::Debug for ToOne<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ToOne")
            .field("target_id", &self.target_id.get())
            .field("state", &self.state.get())
            .finish()
    }
}

impl<T> Default for ToOne<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Clone for ToOne<T> {
    fn clone(&self) -> Self {
        ToOne {
            _marker: PhantomData,
            target_id: Cell::new(self.target_id.get()),
            state: Cell::new(self.state.get()),
            target: Cell::new(None), // Don't clone the cached object
        }
    }
}

// Safety: ToOne is Send if T is Send
unsafe impl<T: Send> Send for ToOne<T> {}

/// Core methods that don't require OBBlanket
impl<T> ToOne<T> {
    /// Create a new empty ToOne relation.
    pub fn new() -> Self {
        ToOne {
            _marker: PhantomData,
            target_id: Cell::new(0),
            state: Cell::new(ToOneState::None),
            target: Cell::new(None),
        }
    }

    /// Create a ToOne with a specific target ID.
    ///
    /// The target object will be lazily loaded when accessed.
    pub fn with_id(id: obx_id) -> Self {
        if id == 0 {
            Self::new()
        } else {
            ToOne {
                _marker: PhantomData,
                target_id: Cell::new(id),
                state: Cell::new(ToOneState::Lazy),
                target: Cell::new(None),
            }
        }
    }

    /// Get the target ID.
    ///
    /// Returns 0 if no target is set.
    pub fn get_target_id(&self) -> obx_id {
        self.target_id.get()
    }

    /// Set the target by ID.
    ///
    /// Pass 0 to remove the relation.
    /// Changes are applied when the owning object is put.
    pub fn set_target_id(&self, id: obx_id) {
        if id == 0 {
            self.clear();
        } else {
            self.target_id.set(id);
            self.state.set(ToOneState::Lazy);
            self.target.set(None);
        }
    }

    /// Check if the relation has a target set.
    pub fn has_value(&self) -> bool {
        self.state.get() != ToOneState::None
    }

    /// Check if the relation is empty (no target).
    pub fn is_empty(&self) -> bool {
        self.state.get() == ToOneState::None
    }

    /// Clear the relation (remove target).
    ///
    /// Changes are applied when the owning object is put.
    pub fn clear(&self) {
        self.target_id.set(0);
        self.state.set(ToOneState::None);
        self.target.set(None);
    }

    /// Mark the target as stored with the given ID.
    ///
    /// This is called internally after putting a new target object.
    pub(crate) fn mark_stored(&self, id: obx_id) {
        self.target_id.set(id);
        if self.state.get() == ToOneState::Unstored {
            self.state.set(ToOneState::Stored);
        }
    }

    /// Check if the target needs to be put (is unstored).
    pub(crate) fn needs_put(&self) -> bool {
        self.state.get() == ToOneState::Unstored
    }

    /// Get the relation property ID for serialization.
    /// This is set by generated code.
    pub(crate) fn get_property_id(&self) -> obx_id {
        // This will be set by generated code
        0
    }
}

/// Methods that require OBBlanket (entity operations)
impl<T: OBBlanket> ToOne<T> {
    /// Set a target object that is already stored in the database.
    ///
    /// The object must have a valid (non-zero) ID.
    pub fn set_target_stored(&self, target: &T) {
        let id = target.get_id();
        if id == 0 {
            // Object not stored yet - mark as unstored
            self.target_id.set(0);
            self.state.set(ToOneState::Unstored);
        } else {
            self.target_id.set(id);
            self.state.set(ToOneState::Stored);
        }
        self.target.set(Some(target as *const T));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::c;
    use crate::traits::{FBOBBridge, IdExt};
    use flatbuffers::FlatBufferBuilder;

    // Test entity for ToOne
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
    fn test_to_one_new() {
        let rel: ToOne<TestEntity> = ToOne::new();
        assert!(rel.is_empty());
        assert!(!rel.has_value());
        assert_eq!(rel.get_target_id(), 0);
    }

    #[test]
    fn test_to_one_with_id() {
        let rel: ToOne<TestEntity> = ToOne::with_id(42);
        assert!(!rel.is_empty());
        assert!(rel.has_value());
        assert_eq!(rel.get_target_id(), 42);
    }

    #[test]
    fn test_to_one_set_target_id() {
        let rel: ToOne<TestEntity> = ToOne::new();
        rel.set_target_id(123);
        assert_eq!(rel.get_target_id(), 123);
        assert!(rel.has_value());
    }

    #[test]
    fn test_to_one_clear() {
        let rel: ToOne<TestEntity> = ToOne::with_id(42);
        assert!(rel.has_value());
        rel.clear();
        assert!(rel.is_empty());
        assert_eq!(rel.get_target_id(), 0);
    }

    #[test]
    fn test_to_one_set_target_stored() {
        let rel: ToOne<TestEntity> = ToOne::new();
        let target = TestEntity { id: 99 };
        rel.set_target_stored(&target);
        assert_eq!(rel.get_target_id(), 99);
        assert!(rel.has_value());
    }
}
