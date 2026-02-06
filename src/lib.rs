#![allow(dead_code)]
// pub extern crate predicates;
pub extern crate anymap as map;
pub extern crate flatbuffers as flatbuffers;
pub extern crate objectbox_generator as generator;
pub extern crate objectbox_macros as macros;

pub mod r#box;
pub mod c;
pub mod error;
pub mod model;
pub mod opt;
pub mod relations;
pub mod store;
pub mod util;
pub mod version;

pub mod query;
pub mod traits;

mod r#async;
mod cursor;
mod txn;

// TODO do the prelude thing, in the generated objectbox_gen.rs
// use objectbox::prelude::*;
