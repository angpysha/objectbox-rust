//! Relations module for ObjectBox
//!
//! This module provides ToOne and ToMany relation types for creating
//! relationships between entities in ObjectBox.
//!
//! # Example
//!
//! ```rust,ignore
//! use objectbox::relations::{ToOne, ToMany};
//!
//! #[entity]
//! struct Order {
//!     #[id]
//!     id: u64,
//!     customer: ToOne<Customer>,
//! }
//!
//! #[entity]
//! struct Student {
//!     #[id]
//!     id: u64,
//!     teachers: ToMany<Teacher>,
//! }
//! ```

mod to_one;
mod to_many;
mod info;

pub use to_one::ToOne;
pub use to_many::ToMany;
pub use info::{RelInfo, RelType};
