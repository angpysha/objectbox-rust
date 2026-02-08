extern crate objectbox;

use objectbox::macros::entity;
use objectbox::relations::{ToOne, ToMany};
use objectbox::datetime::{DateTime, DateTimeNano};

#[derive(Debug)]
#[entity(id = 4, uid = 12469918787009386704)]
pub struct Entity3 {
    #[id(uid = 15234086132445654169)]
    pub id: u64,
    #[property(id = 2, uid = 17547540590170066926)]
    pub hello: String,
}

#[derive(Debug)]
#[entity]
pub struct Entity2 {
    #[id]
    pub id: u64,
    #[index]
    pub index_u64: u64,
}

#[derive(Debug)]
#[entity]
pub struct Entity {
    #[id]
    pub id: u64,
    #[index]
    pub index_u32: u32,
    pub t_bool: bool,
    pub t_u8: u8,
    pub t_i8: i8,
    pub t_i16: i16,
    pub t_u16: u16,
    #[unique]
    pub unique_i32: i32,
    pub t_i32: i32,
    pub t_u32: u32,
    pub t_u64: u64,
    pub t_i64: i64,
    pub t_f32: f32,
    pub t_f64: f64,
    pub t_string: Option<String>,
    pub t_char: char,
    pub t_vec_string: Vec<String>,
    pub t_vec_bytes: Vec<u8>,
}

// Тестова сутність з Option полями
#[derive(Debug)]
#[entity]
pub struct EntityWithOptionals {
    #[id]
    pub id: u64,
    pub required_name: String,
    pub required_age: i32,
    pub optional_nickname: Option<String>,
    pub optional_score: Option<f64>,
    pub optional_count: Option<i32>,
    pub optional_active: Option<bool>,
    pub optional_flag: Option<u8>,
}

// ==================== Relation Test Entities ====================

// NOTE: Target entities must be defined BEFORE source entities that reference them

/// Customer entity for testing ToOne relations
/// (must be defined before Order which references it)
#[derive(Debug)]
#[entity(id = 1, uid = 8476576275622108635)]
pub struct Customer {
    #[id(uid = 16031606834929606574)]
    pub id: u64,
    #[property(id = 2, uid = 10366521659228310164)]
    pub name: String,
    #[property(id = 3, uid = 17074053082311592179)]
    pub email: String,
}

/// Teacher entity for testing ToMany relations  
/// (must be defined before Student which references it)
#[derive(Debug)]
#[entity]
pub struct Teacher {
    #[id]
    pub id: u64,
    pub name: String,
    pub subject: String,
}

/// Order entity with a ToOne relation to Customer
#[derive(Debug)]
#[entity]
pub struct Order {
    #[id]
    pub id: u64,
    pub description: String,
    pub amount: f64,
    /// ToOne relation: an order belongs to one customer
    pub customer: ToOne<Customer>,
}

/// Student entity with a ToMany relation to Teacher
#[derive(Debug)]
#[entity]
pub struct Student {
    #[id]
    pub id: u64,
    pub name: String,
    pub grade: i32,
    /// ToMany relation: a student can have many teachers
    pub teachers: ToMany<Teacher>,
}

/// Entity with renamed properties to test #[property(name = "...")]
/// Rust uses snake_case but DB/model uses camelCase (like Dart's @Property(uid: ...) + name)
#[derive(Debug)]
#[entity]
pub struct RenamedFieldsEntity {
    #[id]
    pub id: u64,
    /// In DB stored as "itemName" (camelCase), in Rust accessed as "item_name" (snake_case)
    /// Index with manually specified id/uid (matching a Dart model)
    #[index(id = 10, uid = 3650410481451669802)]
    #[property(name = "itemName")]
    pub item_name: String,
    /// In DB stored as "itemCount", in Rust accessed as "item_count"
    #[property(name = "itemCount")]
    pub item_count: i32,
    /// In DB stored as "isActive", in Rust accessed as "is_active"
    #[property(name = "isActive")]
    pub is_active: bool,
    /// Property without rename — name is the same in Rust and DB
    pub regular_field: f64,
    /// Unique with manually specified index id/uid
    #[unique(id = 11, uid = 2230256660868146630)]
    pub unique_code: String,
}

/// Entity with DateTime fields to test all date type variations
/// Mirrors Dart's PropertyType.date / dateNano / dateUtc / dateNanoUtc
#[derive(Debug)]
#[entity]
pub struct DateTimeEntity {
    #[id]
    pub id: u64,
    /// UTC DateTime with millisecond precision (like Dart's PropertyType.dateUtc)
    pub created_at: DateTime,
    /// UTC DateTime with nanosecond precision (like Dart's PropertyType.dateNanoUtc)
    pub updated_at: DateTimeNano,
    /// Optional DateTime
    pub deleted_at: Option<DateTime>,
    /// Raw i64 milliseconds with explicit type annotation (manual approach, like Dart's @Transient pattern)
    #[property(type = "date")]
    pub raw_timestamp_ms: i64,
    /// Raw i64 nanoseconds with explicit type annotation
    #[property(type = "dateNano")]
    pub raw_timestamp_ns: i64,
    /// A regular field to ensure mixing with dates works
    pub label: String,
}