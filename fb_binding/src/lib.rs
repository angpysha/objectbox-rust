extern crate flatbuffers;
extern crate objectbox;

use objectbox::macros::entity;

// uncomment the next three lines
// when the mod hasn't been generated yet
mod objectbox_gen;
use objectbox_gen as ob;

use flatbuffers::Table;
use objectbox::traits;
use objectbox::traits::FBOBBridge;
use std::rc;

mod fb_gen;

use crate::fb_gen::entity;
use crate::fb_gen::entityArgs;
use crate::fb_gen::finish_entity_buffer;
use flatbuffers::FlatBufferBuilder;

/*
table entity {
    id:  ulong;

    t_i8:   byte;
    t_u8:   ubyte;
    t_bool: bool;

    t_i16:  short;
    t_u16:  ushort;

    t_i32:  int;
    t_u32:  uint;
    t_f32:  float;

    t_u64:  ulong;
    t_i64:  long;
    t_f64:  double;

    t_string: string;

    t_vec_u8: [ubyte];
    t_vec_string: [string];
}
*/

#[derive(Debug)]
#[entity]
pub struct Entity {
    #[id]
    id: u64,
    t_i8: i8,
    t_u8: u8,
    t_bool: bool,

    t_i16: i16,
    t_u16: u16,

    t_i32: i32,
    t_u32: u32,
    t_f32: f32,

    t_u64: u64,
    t_i64: i64,
    t_f64: f64,

    t_string: String,

    t_vec_u8: Vec<u8>,
    t_vec_string: Vec<String>,
}

fn fb_make_entity<'a>(builder: &'a mut FlatBufferBuilder<'a>, dest: &'a mut Vec<u8>) -> entity<'a> {
    dest.clear();
    builder.reset();
    let args = entityArgs {
        id: 1,
        t_i8: 1,
        t_u8: 2,
        t_bool: false,
        t_i16: 3,
        t_u16: 4,
        t_i32: 5,
        t_u32: 6,
        t_f32: 7.0,
        t_u64: 8,
        t_i64: 9,
        t_f64: 10.0,
        t_string: None,
        t_vec_u8: None,
        t_vec_string: None,
    };

    let entity_offset = entity::create(builder, &args);
    finish_entity_buffer(builder, entity_offset);
    dest.extend_from_slice(builder.finished_data());

    let dest_slice = dest.as_slice();

    unsafe {
        // root_as_entity_unchecked(dest.as_slice()) // works
        // flatbuffers::root_unchecked::<entity>(dest.as_slice()) // works
        entity::init_from_table(Table::new(dest_slice, dest_slice[0].into())) //
    }
}

/*fn ob_make_entity(builder: &mut objectbox::flatbuffers::FlatBufferBuilder, dest: &mut Vec<u8>) -> crate::Entity {
    dest.clear();
    builder.reset();
    let e1 = Entity {
        id: 1,
        t_i8: 1,
        t_u8: 2,
        t_bool: false,
        t_i16: 3,
        t_u16: 4,
        t_i32: 5,
        t_u32: 6,
        t_f32: 7.0,
        t_u64: 8,
        t_i64: 9,
        t_f64: 10.0,
        t_string: "7".to_string(),
        t_vec_u8: vec![1, 2, 3, 4, 5, 6, 7],
        t_vec_string: vec![
            "1".to_string(),
            "1".to_string(),
            "1".to_string(),
            "1".to_string(),
            "1".to_string(),
            "1".to_string(),
            "1".to_string(),
        ],
    };

    e1.flatten(builder);
    dest.extend_from_slice(builder.finished_data());

    // Q: As I got this method converts bytes to Entity using generated code?
    // A: Yes, it uses the generated factory to create an Entity from the byte data


    let trait_map = ob::make_factory_map();

    // let f1 = trait_map
    //     .get::<rc::Rc<dyn >>
    // let f1 = trait_map
    //     .get::<rc::Rc<dyn traits::FactoryHelper<crate::Entity>>>()
    //     .unwrap()
    //     .clone();
    //
    let dest_slice = dest.as_slice();
    unsafe {
        let mut table = Table::new(dest_slice, dest_slice[0].into());
        table.
    }
}*/

#[cfg(test)]
mod tests {
    use super::*;
    use flatbuffers::FlatBufferBuilder;

    #[test]
    fn compare_ob_vs_fb_created_entities() {
        let mut fbb1 = FlatBufferBuilder::new();
        let mut fbb2 = FlatBufferBuilder::new();
        let mut fb_out = Vec::<u8>::new();
        let mut ob_out = Vec::<u8>::new();

        let fb_e = fb_make_entity(&mut fbb1, &mut fb_out);
       // let ob_e = ob_make_entity(&mut fbb2, &mut ob_out);

        assert_eq!(fb_e.id(), 1);
        assert_eq!(fb_e.t_f64(), 10.0);

        // assert_eq!(fb_e.id(), ob_e.id);
        // assert_eq!(fb_e.t_f64(), ob_e.t_f64);
        //
        // assert_eq!("7", ob_e.t_string.as_str());
        // assert_eq!(7, ob_e.t_vec_u8.len());
        // assert_eq!(7, ob_e.t_vec_string.len());
    }
}
