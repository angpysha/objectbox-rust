//! Relation metadata types

use crate::c::obx_schema_id;

/// Specifies the type of a relation field.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelType {
    /// Many-to-many relation (standalone ToMany)
    ToMany,
    /// Many-to-many backlink
    ToManyBacklink,
    /// One-to-many backlink (from ToOne)
    ToOneBacklink,
}

/// Holds relation information for a field.
///
/// This is used internally to track relation metadata during
/// serialization and deserialization.
#[derive(Debug, Clone)]
pub struct RelInfo {
    /// The type of this relation
    pub rel_type: RelType,
    /// Relation ID (for ToMany) or Property ID (for ToOne backlink)
    pub id: obx_schema_id,
    /// Source object ID (or target for backlinks)
    pub object_id: u64,
}

impl RelInfo {
    /// Create info for a ToMany relation field.
    pub fn to_many(id: obx_schema_id, object_id: u64) -> Self {
        RelInfo {
            rel_type: RelType::ToMany,
            id,
            object_id,
        }
    }

    /// Create info for a ToOne backlink relation field.
    pub fn to_one_backlink(id: obx_schema_id, object_id: u64) -> Self {
        RelInfo {
            rel_type: RelType::ToOneBacklink,
            id,
            object_id,
        }
    }

    /// Create info for a ToMany backlink relation field.
    pub fn to_many_backlink(id: obx_schema_id, object_id: u64) -> Self {
        RelInfo {
            rel_type: RelType::ToManyBacklink,
            id,
            object_id,
        }
    }
}
