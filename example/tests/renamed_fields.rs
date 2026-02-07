use example::{make_factory_map, make_model, RenamedFieldsEntity};
use objectbox::error;
use objectbox::traits::{self, IdExt};
use objectbox::{opt::Opt, store::Store};
use std::rc;

use serial_test::serial;

#[test]
#[serial]
fn test_renamed_fields_put_and_get() -> error::Result<()> {
    let mut model = make_model();
    let opt = Opt::from_model(&mut model)?;
    let trait_map = make_factory_map();
    let store = Store::new(opt, trait_map)?;
    let mut box_: objectbox::r#box::Box<'_, RenamedFieldsEntity> = store.get_box::<RenamedFieldsEntity>()?;
    box_.remove_all()?;

    let trait_map = make_factory_map();
    let factory = trait_map
        .get::<rc::Rc<dyn traits::EntityFactoryExt<RenamedFieldsEntity>>>()
        .unwrap()
        .clone();

    // Create entity using Rust snake_case field names
    let mut entity = factory.new_entity();
    entity.item_name = "Test Item".to_string();
    entity.item_count = 42;
    entity.is_active = true;
    entity.regular_field = 3.14;

    // Put and get back
    let id = box_.put(&mut entity)?;
    assert!(id > 0, "Should have assigned an ID");
    assert_eq!(entity.get_id(), id);

    let retrieved = box_.get(id)?.expect("Should find entity by id");

    // Verify all renamed fields round-trip correctly
    assert_eq!(retrieved.item_name, "Test Item");
    assert_eq!(retrieved.item_count, 42);
    assert_eq!(retrieved.is_active, true);
    assert_eq!(retrieved.regular_field, 3.14);

    box_.remove_all()?;
    Ok(())
}

#[test]
#[serial]
fn test_renamed_fields_put_many_and_get_all() -> error::Result<()> {
    let mut model = make_model();
    let opt = Opt::from_model(&mut model)?;
    let trait_map = make_factory_map();
    let store = Store::new(opt, trait_map)?;
    let mut box_: objectbox::r#box::Box<'_, RenamedFieldsEntity> = store.get_box::<RenamedFieldsEntity>()?;
    box_.remove_all()?;

    let trait_map = make_factory_map();
    let factory = trait_map
        .get::<rc::Rc<dyn traits::EntityFactoryExt<RenamedFieldsEntity>>>()
        .unwrap()
        .clone();

    let mut e1 = factory.new_entity();
    e1.item_name = "Alpha".to_string();
    e1.item_count = 10;
    e1.is_active = true;
    e1.regular_field = 1.1;

    let mut e2 = factory.new_entity();
    e2.item_name = "Beta".to_string();
    e2.item_count = 20;
    e2.is_active = false;
    e2.regular_field = 2.2;

    let ids = box_.put_many(vec![&mut e1, &mut e2])?;
    assert_eq!(ids.len(), 2);

    let all = box_.get_all()?;
    assert_eq!(all.len(), 2);

    // Verify first entity
    let first = &all[0];
    assert_eq!(first.item_name, "Alpha");
    assert_eq!(first.item_count, 10);
    assert_eq!(first.is_active, true);
    assert_eq!(first.regular_field, 1.1);

    // Verify second entity
    let second = &all[1];
    assert_eq!(second.item_name, "Beta");
    assert_eq!(second.item_count, 20);
    assert_eq!(second.is_active, false);
    assert_eq!(second.regular_field, 2.2);

    box_.remove_all()?;
    Ok(())
}

#[test]
#[serial]
fn test_renamed_fields_update() -> error::Result<()> {
    let mut model = make_model();
    let opt = Opt::from_model(&mut model)?;
    let trait_map = make_factory_map();
    let store = Store::new(opt, trait_map)?;
    let mut box_: objectbox::r#box::Box<'_, RenamedFieldsEntity> = store.get_box::<RenamedFieldsEntity>()?;
    box_.remove_all()?;

    let trait_map = make_factory_map();
    let factory = trait_map
        .get::<rc::Rc<dyn traits::EntityFactoryExt<RenamedFieldsEntity>>>()
        .unwrap()
        .clone();

    // Create and put
    let mut entity = factory.new_entity();
    entity.item_name = "Original".to_string();
    entity.item_count = 1;
    entity.is_active = false;
    entity.regular_field = 0.0;

    let id = box_.put(&mut entity)?;

    // Update fields using Rust names
    entity.item_name = "Updated".to_string();
    entity.item_count = 99;
    entity.is_active = true;
    entity.regular_field = 9.99;

    let id2 = box_.put(&mut entity)?;
    assert_eq!(id, id2, "Should update same entity");
    assert_eq!(box_.count()?, 1, "Should still be one entity");

    let retrieved = box_.get(id)?.expect("Should find updated entity");
    assert_eq!(retrieved.item_name, "Updated");
    assert_eq!(retrieved.item_count, 99);
    assert_eq!(retrieved.is_active, true);
    assert_eq!(retrieved.regular_field, 9.99);

    box_.remove_all()?;
    Ok(())
}
