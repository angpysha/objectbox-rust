use example::{
    make_factory_map, make_model, new_entity_condition_factory, Entity, EntityConditionFactory,
};
use objectbox::{error, opt::Opt, store::Store};

use serial_test::serial;

#[test]
#[serial]
fn basic_query_tests() -> error::Result<()> {
    let mut model = make_model();
    let opt = Opt::from_model(&mut model)?;
    let trait_map = make_factory_map();
    let store = Store::new(opt, trait_map)?;

    let mut box1 = store.get_box::<Entity>()?;
    box1.remove_all()?;

    let EntityConditionFactory {
        // id,
        index_u32,
        t_bool,
        // t_u8,
        // t_i8,
        // t_i16,
        // t_u16,
        unique_i32,
        // t_i32,
        // t_u32,
        t_u64,
        t_i64,
        // t_f32,
        // t_f64,
        // t_string,
        t_char,
        t_vec_bytes,
        ..
    } = new_entity_condition_factory();

    let mut entity = Entity {
        id: 0,
        index_u32: 1,
        t_bool: false,
        t_u8: 2,
        t_i8: 3,
        t_i16: 4,
        t_u16: 5,
        unique_i32: 6,
        t_i32: 7,
        t_u32: 8,
        t_u64: 9,
        t_i64: 11,
        t_f32: 12.0,
        t_f64: 13.0,
        t_string: Option::from("14".to_string()),
        t_char: 'c',
        t_vec_string: vec!["str1".to_string(), "str2".to_string()],
        t_vec_bytes: vec![0x9, 0x8, 0x7, 0x6, 0x5],
    };

    box1.put(&mut entity)?;

    // pretend this is a new object
    entity.id = 0;

    // set new unique values
    entity.index_u32 = 555;
    entity.unique_i32 = 555;
    entity.t_i64 = 555;
    entity.t_u64 = 555;

    // store "two" items
    box1.put(&mut entity)?;

    // TODO investigate: doesn't seem to be supported
    // assert_eq!(
    //     2,
    //     box1.query(&mut t_bool.eq(0 as i64))?.count()?
    // );
    // assert_eq!(
    //     2,
    //     box1.query(&mut t_bool.ne(1 as i64))?.count()?
    // );
    assert_eq!(
        2,
        box1.query(&mut t_vec_bytes.eq(vec![0x9, 0x8, 0x7, 0x6, 0x5]))?
            .count()?
    );
    assert_eq!(2, box1.query(&mut t_vec_bytes.ne(vec![0x0]))?.count()?);
    assert_eq!(
        2,
        box1.query(&mut t_vec_bytes.le(vec![0xA, 0x8, 0x7, 0x6, 0x5]))?
            .count()?
    );
    assert_eq!(
        2,
        box1.query(&mut t_vec_bytes.ge(vec![0x8, 0x7, 0x6, 0x5, 0x4]))?
            .count()?
    );
    assert_eq!(
        2,
        box1.query(&mut t_vec_bytes.lt(vec![0xA, 0xA, 0xA, 0xA, 0xA]))?
            .count()?
    );
    assert_eq!(2, box1.query(&mut t_vec_bytes.gt(vec![0x0]))?.count()?);
    assert_eq!(2, box1.query(&mut t_char.eq('c' as i64))?.count()?);
    assert_eq!(2, box1.query(&mut t_char.ne('b' as i64))?.count()?);
    assert_eq!(2, box1.query(&mut t_char.le('d' as i64))?.count()?);
    assert_eq!(2, box1.query(&mut t_char.ge('b' as i64))?.count()?);
    assert_eq!(2, box1.query(&mut t_char.lt('d' as i64))?.count()?);
    assert_eq!(2, box1.query(&mut t_char.gt('b' as i64))?.count()?);
    assert_eq!(2, box1.query(&mut index_u32.ge(1))?.count()?);
    assert_eq!(1, box1.query(&mut index_u32.le(2))?.count()?);
    assert_eq!(2, box1.query(&mut index_u32.gt(0))?.count()?);
    assert_eq!(1, box1.query(&mut index_u32.lt(2))?.count()?);
    assert_eq!(1, box1.query(&mut index_u32.eq(1))?.count()?);
    assert_eq!(2, box1.query(&mut index_u32.ne(0))?.count()?);

    // TODO separate: not_member_of and member_of, because String does not support not_member_of aka not_in_strings
    // TODO lifetime of Vec could drop before the condition can be calculated, box?
    // assert_eq!(
    //     2,
    //     box1.query(&mut unique_i32.not_member_of(vec![6, 555]))?.count()?
    // );

    // assert_eq!(
    //     2,
    //     box1.query(&mut t_i64.not_member_of(vec![11, 555]))?.count()?
    // );

    // assert_eq!(
    //     2,
    //     box1.query(&mut t_u64.not_member_of(vec![11, 555]))?.count()?
    // );

    let r: Vec<Entity> = box1.query(&mut index_u32.ne(1))?.find()?;
    assert_eq!(r[0].index_u32, entity.index_u32);

    Ok(())
}
