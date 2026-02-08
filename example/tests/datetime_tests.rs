use example::{make_factory_map, make_model, DateTimeEntity};
use objectbox::datetime::{DateTime, DateTimeNano};
use objectbox::error;
// objectbox traits available if needed for query operations
use objectbox::{opt::Opt, store::Store};

use serial_test::serial;

#[test]
#[serial]
fn test_datetime_put_and_get() -> error::Result<()> {
    let mut model = make_model();
    let opt = Opt::from_model(&mut model)?;
    let trait_map = make_factory_map();
    let store = Store::new(opt, trait_map)?;
    let mut box_: objectbox::r#box::Box<'_, DateTimeEntity> =
        store.get_box::<DateTimeEntity>()?;
    box_.remove_all()?;

    // Create entity with specific timestamps
    let created = DateTime::from_millis(1706745600000); // 2024-02-01 00:00:00 UTC
    let updated = DateTimeNano::from_nanos(1706745600_123_456_789);
    let deleted = DateTime::from_millis(1706832000000); // 2024-02-02 00:00:00 UTC

    let mut entity = DateTimeEntity {
        id: 0,
        created_at: created,
        updated_at: updated,
        deleted_at: Some(deleted),
        raw_timestamp_ms: 1706745600000,
        raw_timestamp_ns: 1706745600_000_000_000,
        label: "test_datetime".to_string(),
    };

    let id = box_.put(&mut entity)?;
    assert!(id > 0);
    assert_eq!(entity.id, id);

    // Read back and verify
    let read_entity = box_.get(id)?.expect("Entity should exist");
    assert_eq!(read_entity.created_at, created);
    assert_eq!(read_entity.created_at.to_millis(), 1706745600000);
    assert_eq!(read_entity.updated_at, updated);
    assert_eq!(read_entity.updated_at.to_nanos(), 1706745600_123_456_789);
    assert_eq!(read_entity.deleted_at, Some(deleted));
    assert_eq!(read_entity.deleted_at.unwrap().to_millis(), 1706832000000);
    assert_eq!(read_entity.raw_timestamp_ms, 1706745600000);
    assert_eq!(read_entity.raw_timestamp_ns, 1706745600_000_000_000);
    assert_eq!(read_entity.label, "test_datetime");

    Ok(())
}

#[test]
#[serial]
fn test_datetime_now() -> error::Result<()> {
    let mut model = make_model();
    let opt = Opt::from_model(&mut model)?;
    let trait_map = make_factory_map();
    let store = Store::new(opt, trait_map)?;
    let mut box_: objectbox::r#box::Box<'_, DateTimeEntity> =
        store.get_box::<DateTimeEntity>()?;
    box_.remove_all()?;

    // Use ::now() constructors
    let before = DateTime::now();
    let mut entity = DateTimeEntity {
        id: 0,
        created_at: DateTime::now(),
        updated_at: DateTimeNano::now(),
        deleted_at: None,
        raw_timestamp_ms: 0,
        raw_timestamp_ns: 0,
        label: "now_test".to_string(),
    };

    let id = box_.put(&mut entity)?;
    let read = box_.get(id)?.expect("Entity should exist");

    // created_at should be close to "before" (within 1 second)
    assert!(read.created_at.to_millis() >= before.to_millis());
    assert!(read.created_at.to_millis() - before.to_millis() < 1000);

    // updated_at should be non-zero
    assert!(!read.updated_at.is_zero());

    // deleted_at should be None
    assert_eq!(read.deleted_at, None);

    Ok(())
}

#[test]
#[serial]
fn test_datetime_zero_and_default() -> error::Result<()> {
    let mut model = make_model();
    let opt = Opt::from_model(&mut model)?;
    let trait_map = make_factory_map();
    let store = Store::new(opt, trait_map)?;
    let mut box_: objectbox::r#box::Box<'_, DateTimeEntity> =
        store.get_box::<DateTimeEntity>()?;
    box_.remove_all()?;

    // Test default/zero values (epoch)
    let mut entity = DateTimeEntity {
        id: 0,
        created_at: DateTime::default(),
        updated_at: DateTimeNano::default(),
        deleted_at: None,
        raw_timestamp_ms: 0,
        raw_timestamp_ns: 0,
        label: "zero_test".to_string(),
    };

    assert!(entity.created_at.is_zero());
    assert!(entity.updated_at.is_zero());

    let id = box_.put(&mut entity)?;
    let read = box_.get(id)?.expect("Entity should exist");

    assert_eq!(read.created_at.to_millis(), 0);
    assert_eq!(read.updated_at.to_nanos(), 0);
    assert_eq!(read.deleted_at, None);
    assert!(read.created_at.is_zero());
    assert!(read.updated_at.is_zero());

    Ok(())
}

#[test]
#[serial]
fn test_datetime_raw_i64_type_annotation() -> error::Result<()> {
    let mut model = make_model();
    let opt = Opt::from_model(&mut model)?;
    let trait_map = make_factory_map();
    let store = Store::new(opt, trait_map)?;
    let mut box_: objectbox::r#box::Box<'_, DateTimeEntity> =
        store.get_box::<DateTimeEntity>()?;
    box_.remove_all()?;

    // Test raw i64 fields with #[property(type = "date")] and #[property(type = "dateNano")]
    // This is the manual approach similar to Dart's @Transient + getter/setter pattern
    let ms_value: i64 = 1706745600000;
    let ns_value: i64 = 1706745600_000_000_000;

    let mut entity = DateTimeEntity {
        id: 0,
        created_at: DateTime::from_millis(0),
        updated_at: DateTimeNano::from_nanos(0),
        deleted_at: None,
        raw_timestamp_ms: ms_value,
        raw_timestamp_ns: ns_value,
        label: "raw_test".to_string(),
    };

    let id = box_.put(&mut entity)?;
    let read = box_.get(id)?.expect("Entity should exist");

    assert_eq!(read.raw_timestamp_ms, ms_value);
    assert_eq!(read.raw_timestamp_ns, ns_value);

    Ok(())
}

#[test]
#[serial]
fn test_datetime_put_many_and_get_all() -> error::Result<()> {
    let mut model = make_model();
    let opt = Opt::from_model(&mut model)?;
    let trait_map = make_factory_map();
    let store = Store::new(opt, trait_map)?;
    let mut box_: objectbox::r#box::Box<'_, DateTimeEntity> =
        store.get_box::<DateTimeEntity>()?;
    box_.remove_all()?;

    let mut entities: Vec<DateTimeEntity> = (0..5).map(|i| {
        DateTimeEntity {
            id: 0,
            created_at: DateTime::from_millis(1706745600000 + i * 86400000), // each day later
            updated_at: DateTimeNano::from_nanos(1706745600_000_000_000 + i * 86400_000_000_000),
            deleted_at: if i % 2 == 0 { Some(DateTime::from_millis(1706745600000 + i * 86400000)) } else { None },
            raw_timestamp_ms: 1000 + i,
            raw_timestamp_ns: 1_000_000 + i,
            label: format!("entity_{}", i),
        }
    }).collect();

    let mut_refs: Vec<&mut DateTimeEntity> = entities.iter_mut().collect();
    let ids = box_.put_many(mut_refs)?;
    assert_eq!(ids.len(), 5);

    let all = box_.get_all()?;
    assert_eq!(all.len(), 5);

    // Check ordering and values
    for (i, e) in all.iter().enumerate() {
        let i = i as i64;
        assert_eq!(e.created_at.to_millis(), 1706745600000 + i * 86400000);
        assert_eq!(e.updated_at.to_nanos(), 1706745600_000_000_000 + i * 86400_000_000_000);
        assert_eq!(e.label, format!("entity_{}", i));
        if i % 2 == 0 {
            assert!(e.deleted_at.is_some());
        } else {
            assert!(e.deleted_at.is_none());
        }
    }

    Ok(())
}

#[test]
#[serial]
fn test_datetime_conversion_helpers() {
    // Test the conversion helpers (no DB needed)
    let dt = DateTime::from_secs(1706745600);
    assert_eq!(dt.to_millis(), 1706745600000);
    assert_eq!(dt.to_secs(), 1706745600);

    let nano = DateTimeNano::from_millis(1706745600000);
    assert_eq!(nano.to_millis(), 1706745600000);
    assert_eq!(nano.to_nanos(), 1706745600_000_000_000);

    let nano_from_micros = DateTimeNano::from_micros(1706745600_000_000);
    assert_eq!(nano_from_micros.to_micros(), 1706745600_000_000);

    // DateTime → DateTimeNano conversion
    let dt2 = DateTime::from_millis(1706745600123);
    let nano2: DateTimeNano = dt2.into();
    assert_eq!(nano2.to_millis(), 1706745600123);

    // DateTimeNano → DateTime conversion (truncating)
    let nano3 = DateTimeNano::from_nanos(1706745600_123_456_789);
    let dt3 = nano3.to_datetime();
    assert_eq!(dt3.to_millis(), 1706745600_123);
}
