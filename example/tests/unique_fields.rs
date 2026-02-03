use example::{make_factory_map, make_model, Entity};
use objectbox::{error, opt::Opt, store::Store};

use serial_test::serial;

#[test]
#[serial]
fn uniqueness_tests() -> error::Result<()> {
    let mut model = make_model();
    let opt = Opt::from_model(&mut model)?;
    let trait_map = make_factory_map();
    let store = Store::new(opt, trait_map)?;

    let mut box1 = store.get_box::<Entity>()?;
    box1.remove_all()?;

    let mut entity = Entity {
        id: 0,
        index_u32: 333,
        t_bool: false,
        t_u8: 2,
        t_i8: 3,
        t_i16: 4,
        t_u16: 5,
        unique_i32: 555,
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

    assert!(box1.put(&mut entity).is_err());

    Ok(())
}
