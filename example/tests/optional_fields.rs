use example::{
    make_factory_map, make_model, new_entitywithoptionals_condition_factory, EntityWithOptionals,
    EntityWithOptionalsConditionFactory,
};
use objectbox::{error, opt::Opt, store::Store};

use serial_test::serial;

/// Helper: create a store for tests
fn setup_store() -> error::Result<Store> {
    let mut model = make_model();
    let opt = Opt::from_model(&mut model)?;
    let trait_map = make_factory_map();
    Store::new(opt, trait_map)
}

// =============================================================================
// 1. Save None values
// =============================================================================

#[test]
#[serial]
fn test_save_entity_with_all_none() -> error::Result<()> {
    let store = setup_store()?;
    let mut ob_box = store.get_box::<EntityWithOptionals>()?;
    ob_box.remove_all()?;

    let mut entity = EntityWithOptionals {
        id: 0,
        required_name: "Alice".to_string(),
        required_age: 30,
        optional_nickname: None,
        optional_score: None,
        optional_count: None,
        optional_active: None,
        optional_flag: None,
    };

    let new_id = ob_box.put(&mut entity)?;
    assert!(new_id > 0, "Entity should be assigned a valid ID");
    assert_eq!(entity.id, new_id, "Entity ID should be updated in place");

    // Verify it's stored
    assert_eq!(1, ob_box.count()?);

    ob_box.remove_all()?;
    Ok(())
}

// =============================================================================
// 2. Save Some values
// =============================================================================

#[test]
#[serial]
fn test_save_entity_with_all_some() -> error::Result<()> {
    let store = setup_store()?;
    let mut ob_box = store.get_box::<EntityWithOptionals>()?;
    ob_box.remove_all()?;

    let mut entity = EntityWithOptionals {
        id: 0,
        required_name: "Bob".to_string(),
        required_age: 25,
        optional_nickname: Some("Bobby".to_string()),
        optional_score: Some(99.5),
        optional_count: Some(42),
        optional_active: Some(true),
        optional_flag: Some(7),
    };

    let new_id = ob_box.put(&mut entity)?;
    assert!(new_id > 0);

    ob_box.remove_all()?;
    Ok(())
}

// =============================================================================
// 3. Read None values
// =============================================================================

#[test]
#[serial]
fn test_read_none_values() -> error::Result<()> {
    let store = setup_store()?;
    let mut ob_box = store.get_box::<EntityWithOptionals>()?;
    ob_box.remove_all()?;

    let mut entity = EntityWithOptionals {
        id: 0,
        required_name: "Charlie".to_string(),
        required_age: 40,
        optional_nickname: None,
        optional_score: None,
        optional_count: None,
        optional_active: None,
        optional_flag: None,
    };

    let new_id = ob_box.put(&mut entity)?;
    let loaded = ob_box.get(new_id)?.expect("Entity should exist");

    assert_eq!(loaded.required_name, "Charlie");
    assert_eq!(loaded.required_age, 40);
    assert_eq!(loaded.optional_nickname, None, "optional_nickname should be None");
    assert_eq!(loaded.optional_score, None, "optional_score should be None");
    assert_eq!(loaded.optional_count, None, "optional_count should be None");
    assert_eq!(loaded.optional_active, None, "optional_active should be None");
    assert_eq!(loaded.optional_flag, None, "optional_flag should be None");

    ob_box.remove_all()?;
    Ok(())
}

// =============================================================================
// 4. Read Some values
// =============================================================================

#[test]
#[serial]
fn test_read_some_values() -> error::Result<()> {
    let store = setup_store()?;
    let mut ob_box = store.get_box::<EntityWithOptionals>()?;
    ob_box.remove_all()?;

    let mut entity = EntityWithOptionals {
        id: 0,
        required_name: "Diana".to_string(),
        required_age: 35,
        optional_nickname: Some("Di".to_string()),
        optional_score: Some(88.3),
        optional_count: Some(10),
        optional_active: Some(false),
        optional_flag: Some(255),
    };

    let new_id = ob_box.put(&mut entity)?;
    let loaded = ob_box.get(new_id)?.expect("Entity should exist");

    assert_eq!(loaded.required_name, "Diana");
    assert_eq!(loaded.required_age, 35);
    assert_eq!(loaded.optional_nickname, Some("Di".to_string()));
    assert_eq!(loaded.optional_score, Some(88.3));
    assert_eq!(loaded.optional_count, Some(10));
    assert_eq!(loaded.optional_active, Some(false));
    assert_eq!(loaded.optional_flag, Some(255));

    ob_box.remove_all()?;
    Ok(())
}

// =============================================================================
// 5. Update Some → None
// =============================================================================

#[test]
#[serial]
fn test_update_some_to_none() -> error::Result<()> {
    let store = setup_store()?;
    let mut ob_box = store.get_box::<EntityWithOptionals>()?;
    ob_box.remove_all()?;

    // First, save with all Some values
    let mut entity = EntityWithOptionals {
        id: 0,
        required_name: "Eve".to_string(),
        required_age: 28,
        optional_nickname: Some("Evie".to_string()),
        optional_score: Some(75.0),
        optional_count: Some(5),
        optional_active: Some(true),
        optional_flag: Some(128),
    };

    let new_id = ob_box.put(&mut entity)?;

    // Verify the Some values were saved
    let loaded = ob_box.get(new_id)?.expect("Entity should exist");
    assert_eq!(loaded.optional_nickname, Some("Evie".to_string()));
    assert_eq!(loaded.optional_score, Some(75.0));
    assert_eq!(loaded.optional_count, Some(5));
    assert_eq!(loaded.optional_active, Some(true));
    assert_eq!(loaded.optional_flag, Some(128));

    // Now update all optional fields to None
    let mut updated_entity = EntityWithOptionals {
        id: new_id, // same ID = update
        required_name: "Eve".to_string(),
        required_age: 28,
        optional_nickname: None,
        optional_score: None,
        optional_count: None,
        optional_active: None,
        optional_flag: None,
    };

    ob_box.put(&mut updated_entity)?;

    // Read back and verify all are None
    let reloaded = ob_box.get(new_id)?.expect("Entity should still exist");
    assert_eq!(reloaded.required_name, "Eve");
    assert_eq!(reloaded.optional_nickname, None, "nickname should be None after update");
    assert_eq!(reloaded.optional_score, None, "score should be None after update");
    assert_eq!(reloaded.optional_count, None, "count should be None after update");
    assert_eq!(reloaded.optional_active, None, "active should be None after update");
    assert_eq!(reloaded.optional_flag, None, "flag should be None after update");

    ob_box.remove_all()?;
    Ok(())
}

// =============================================================================
// 6. Update None → Some
// =============================================================================

#[test]
#[serial]
fn test_update_none_to_some() -> error::Result<()> {
    let store = setup_store()?;
    let mut ob_box = store.get_box::<EntityWithOptionals>()?;
    ob_box.remove_all()?;

    // First, save with all None values
    let mut entity = EntityWithOptionals {
        id: 0,
        required_name: "Frank".to_string(),
        required_age: 50,
        optional_nickname: None,
        optional_score: None,
        optional_count: None,
        optional_active: None,
        optional_flag: None,
    };

    let new_id = ob_box.put(&mut entity)?;

    // Verify None values
    let loaded = ob_box.get(new_id)?.expect("Entity should exist");
    assert_eq!(loaded.optional_nickname, None);
    assert_eq!(loaded.optional_score, None);

    // Now update all optional fields to Some
    let mut updated_entity = EntityWithOptionals {
        id: new_id,
        required_name: "Frank".to_string(),
        required_age: 50,
        optional_nickname: Some("Franky".to_string()),
        optional_score: Some(100.0),
        optional_count: Some(999),
        optional_active: Some(false),
        optional_flag: Some(1),
    };

    ob_box.put(&mut updated_entity)?;

    // Read back and verify all are Some
    let reloaded = ob_box.get(new_id)?.expect("Entity should still exist");
    assert_eq!(reloaded.optional_nickname, Some("Franky".to_string()));
    assert_eq!(reloaded.optional_score, Some(100.0));
    assert_eq!(reloaded.optional_count, Some(999));
    assert_eq!(reloaded.optional_active, Some(false));
    assert_eq!(reloaded.optional_flag, Some(1));

    ob_box.remove_all()?;
    Ok(())
}

// =============================================================================
// 7. Mixed Optional patterns
// =============================================================================

#[test]
#[serial]
fn test_mixed_some_and_none() -> error::Result<()> {
    let store = setup_store()?;
    let mut ob_box = store.get_box::<EntityWithOptionals>()?;
    ob_box.remove_all()?;

    // Some fields are Some, others are None
    let mut entity = EntityWithOptionals {
        id: 0,
        required_name: "Grace".to_string(),
        required_age: 22,
        optional_nickname: Some("Gracie".to_string()),
        optional_score: None,
        optional_count: Some(3),
        optional_active: None,
        optional_flag: Some(42),
    };

    let new_id = ob_box.put(&mut entity)?;
    let loaded = ob_box.get(new_id)?.expect("Entity should exist");

    assert_eq!(loaded.optional_nickname, Some("Gracie".to_string()));
    assert_eq!(loaded.optional_score, None);
    assert_eq!(loaded.optional_count, Some(3));
    assert_eq!(loaded.optional_active, None);
    assert_eq!(loaded.optional_flag, Some(42));

    ob_box.remove_all()?;
    Ok(())
}

// =============================================================================
// 8. put_many with mixed optional values
// =============================================================================

#[test]
#[serial]
fn test_put_many_with_optionals() -> error::Result<()> {
    let store = setup_store()?;
    let mut ob_box = store.get_box::<EntityWithOptionals>()?;
    ob_box.remove_all()?;

    let mut e1 = EntityWithOptionals {
        id: 0,
        required_name: "One".to_string(),
        required_age: 1,
        optional_nickname: None,
        optional_score: None,
        optional_count: None,
        optional_active: None,
        optional_flag: None,
    };
    let mut e2 = EntityWithOptionals {
        id: 0,
        required_name: "Two".to_string(),
        required_age: 2,
        optional_nickname: Some("Second".to_string()),
        optional_score: Some(2.0),
        optional_count: Some(2),
        optional_active: Some(true),
        optional_flag: Some(2),
    };
    let mut e3 = EntityWithOptionals {
        id: 0,
        required_name: "Three".to_string(),
        required_age: 3,
        optional_nickname: Some("Third".to_string()),
        optional_score: None,
        optional_count: Some(33),
        optional_active: None,
        optional_flag: None,
    };

    let ids = ob_box.put_many(vec![&mut e1, &mut e2, &mut e3])?;
    assert_eq!(3, ids.len());
    assert_eq!(3, ob_box.count()?);

    // Verify each one
    let loaded1 = ob_box.get(ids[0])?.expect("Entity 1 should exist");
    assert_eq!(loaded1.optional_nickname, None);
    assert_eq!(loaded1.optional_score, None);

    let loaded2 = ob_box.get(ids[1])?.expect("Entity 2 should exist");
    assert_eq!(loaded2.optional_nickname, Some("Second".to_string()));
    assert_eq!(loaded2.optional_score, Some(2.0));
    assert_eq!(loaded2.optional_active, Some(true));

    let loaded3 = ob_box.get(ids[2])?.expect("Entity 3 should exist");
    assert_eq!(loaded3.optional_nickname, Some("Third".to_string()));
    assert_eq!(loaded3.optional_score, None);
    assert_eq!(loaded3.optional_count, Some(33));
    assert_eq!(loaded3.optional_active, None);

    ob_box.remove_all()?;
    Ok(())
}

// =============================================================================
// 9. get_all with optional fields
// =============================================================================

#[test]
#[serial]
fn test_get_all_with_optionals() -> error::Result<()> {
    let store = setup_store()?;
    let mut ob_box = store.get_box::<EntityWithOptionals>()?;
    ob_box.remove_all()?;

    let mut e1 = EntityWithOptionals {
        id: 0,
        required_name: "A".to_string(),
        required_age: 10,
        optional_nickname: None,
        optional_score: Some(1.0),
        optional_count: None,
        optional_active: None,
        optional_flag: None,
    };
    let mut e2 = EntityWithOptionals {
        id: 0,
        required_name: "B".to_string(),
        required_age: 20,
        optional_nickname: Some("Bee".to_string()),
        optional_score: None,
        optional_count: Some(7),
        optional_active: Some(true),
        optional_flag: Some(99),
    };

    ob_box.put_many(vec![&mut e1, &mut e2])?;

    let all = ob_box.get_all()?;
    assert_eq!(2, all.len());

    // Find the entity named "A"
    let a = all.iter().find(|e| e.required_name == "A").expect("Should find A");
    assert_eq!(a.optional_nickname, None);
    assert_eq!(a.optional_score, Some(1.0));

    // Find the entity named "B"
    let b = all.iter().find(|e| e.required_name == "B").expect("Should find B");
    assert_eq!(b.optional_nickname, Some("Bee".to_string()));
    assert_eq!(b.optional_score, None);
    assert_eq!(b.optional_count, Some(7));
    assert_eq!(b.optional_active, Some(true));
    assert_eq!(b.optional_flag, Some(99));

    ob_box.remove_all()?;
    Ok(())
}

// =============================================================================
// 10. Query: is_null / is_not_null on optional fields
// =============================================================================

#[test]
#[serial]
fn test_query_is_null_and_is_not_null() -> error::Result<()> {
    let store = setup_store()?;
    let mut ob_box = store.get_box::<EntityWithOptionals>()?;
    ob_box.remove_all()?;

    let EntityWithOptionalsConditionFactory {
        optional_nickname,
        optional_score,
        optional_count,
        ..
    } = new_entitywithoptionals_condition_factory();

    // Entity with all None
    let mut e_none = EntityWithOptionals {
        id: 0,
        required_name: "NoneEntity".to_string(),
        required_age: 10,
        optional_nickname: None,
        optional_score: None,
        optional_count: None,
        optional_active: None,
        optional_flag: None,
    };
    // Entity with all Some
    let mut e_some = EntityWithOptionals {
        id: 0,
        required_name: "SomeEntity".to_string(),
        required_age: 20,
        optional_nickname: Some("Nick".to_string()),
        optional_score: Some(95.5),
        optional_count: Some(42),
        optional_active: Some(true),
        optional_flag: Some(7),
    };
    // Entity with mixed
    let mut e_mixed = EntityWithOptionals {
        id: 0,
        required_name: "MixedEntity".to_string(),
        required_age: 30,
        optional_nickname: Some("Mixed".to_string()),
        optional_score: None,
        optional_count: Some(10),
        optional_active: None,
        optional_flag: None,
    };

    ob_box.put_many(vec![&mut e_none, &mut e_some, &mut e_mixed])?;
    assert_eq!(3, ob_box.count()?);

    // Query: optional_nickname IS NULL → should find 1 (NoneEntity)
    assert_eq!(
        1,
        ob_box.query(&mut optional_nickname.is_null())?.count()?,
        "1 entity should have optional_nickname = NULL"
    );

    // Query: optional_nickname IS NOT NULL → should find 2 (SomeEntity + MixedEntity)
    assert_eq!(
        2,
        ob_box.query(&mut optional_nickname.is_not_null())?.count()?,
        "2 entities should have optional_nickname != NULL"
    );

    // Query: optional_score IS NULL → 2 (NoneEntity + MixedEntity)
    assert_eq!(
        2,
        ob_box.query(&mut optional_score.is_null())?.count()?,
        "2 entities should have optional_score = NULL"
    );

    // Query: optional_score IS NOT NULL → 1 (SomeEntity)
    assert_eq!(
        1,
        ob_box.query(&mut optional_score.is_not_null())?.count()?,
        "1 entity should have optional_score != NULL"
    );

    // Query: optional_count IS NULL → 1 (NoneEntity)
    assert_eq!(
        1,
        ob_box.query(&mut optional_count.is_null())?.count()?,
        "1 entity should have optional_count = NULL"
    );

    // Query: optional_count IS NOT NULL → 2 (SomeEntity + MixedEntity)
    assert_eq!(
        2,
        ob_box.query(&mut optional_count.is_not_null())?.count()?,
        "2 entities should have optional_count != NULL"
    );

    ob_box.remove_all()?;
    Ok(())
}

// =============================================================================
// 11. Query with eq/ne on optional String field
// =============================================================================

#[test]
#[serial]
fn test_query_eq_on_optional_string() -> error::Result<()> {
    let store = setup_store()?;
    let mut ob_box = store.get_box::<EntityWithOptionals>()?;
    ob_box.remove_all()?;

    let EntityWithOptionalsConditionFactory {
        optional_nickname,
        ..
    } = new_entitywithoptionals_condition_factory();

    let mut e1 = EntityWithOptionals {
        id: 0,
        required_name: "A".to_string(),
        required_age: 1,
        optional_nickname: Some("Alpha".to_string()),
        optional_score: None,
        optional_count: None,
        optional_active: None,
        optional_flag: None,
    };
    let mut e2 = EntityWithOptionals {
        id: 0,
        required_name: "B".to_string(),
        required_age: 2,
        optional_nickname: Some("Beta".to_string()),
        optional_score: None,
        optional_count: None,
        optional_active: None,
        optional_flag: None,
    };
    let mut e3 = EntityWithOptionals {
        id: 0,
        required_name: "C".to_string(),
        required_age: 3,
        optional_nickname: None,
        optional_score: None,
        optional_count: None,
        optional_active: None,
        optional_flag: None,
    };

    ob_box.put_many(vec![&mut e1, &mut e2, &mut e3])?;

    // is_not_null on optional_nickname should find 2 (Alpha + Beta)
    let not_null_count = ob_box
        .query(&mut optional_nickname.is_not_null())?
        .count()?;
    assert_eq!(2, not_null_count, "Two entities have optional_nickname set");

    // is_null on optional_nickname should find 1 (entity C)
    let null_count = ob_box
        .query(&mut optional_nickname.is_null())?
        .count()?;
    assert_eq!(1, null_count, "One entity has optional_nickname = NULL");

    ob_box.remove_all()?;
    Ok(())
}

// =============================================================================
// 12. Query with comparison on optional numeric fields
// =============================================================================

#[test]
#[serial]
fn test_query_comparison_on_optional_i32() -> error::Result<()> {
    let store = setup_store()?;
    let mut ob_box = store.get_box::<EntityWithOptionals>()?;
    ob_box.remove_all()?;

    let EntityWithOptionalsConditionFactory {
        optional_count,
        ..
    } = new_entitywithoptionals_condition_factory();

    let mut e1 = EntityWithOptionals {
        id: 0,
        required_name: "Low".to_string(),
        required_age: 1,
        optional_nickname: None,
        optional_score: None,
        optional_count: Some(10),
        optional_active: None,
        optional_flag: None,
    };
    let mut e2 = EntityWithOptionals {
        id: 0,
        required_name: "High".to_string(),
        required_age: 2,
        optional_nickname: None,
        optional_score: None,
        optional_count: Some(100),
        optional_active: None,
        optional_flag: None,
    };
    let mut e3 = EntityWithOptionals {
        id: 0,
        required_name: "Null".to_string(),
        required_age: 3,
        optional_nickname: None,
        optional_score: None,
        optional_count: None,
        optional_active: None,
        optional_flag: None,
    };

    ob_box.put_many(vec![&mut e1, &mut e2, &mut e3])?;

    // eq on optional i32
    assert_eq!(
        1,
        ob_box.query(&mut optional_count.eq(10))?.count()?,
        "Exactly one entity with optional_count = 10"
    );

    // gt on optional i32
    assert_eq!(
        1,
        ob_box.query(&mut optional_count.gt(50))?.count()?,
        "One entity with optional_count > 50"
    );

    // ge on optional i32
    assert_eq!(
        2,
        ob_box.query(&mut optional_count.ge(10))?.count()?,
        "Two entities with optional_count >= 10"
    );

    // lt on optional i32
    assert_eq!(
        1,
        ob_box.query(&mut optional_count.lt(50))?.count()?,
        "One entity with optional_count < 50"
    );

    ob_box.remove_all()?;
    Ok(())
}

// =============================================================================
// 13. Edge case: empty string vs None
// =============================================================================

#[test]
#[serial]
fn test_empty_string_vs_none() -> error::Result<()> {
    let store = setup_store()?;
    let mut ob_box = store.get_box::<EntityWithOptionals>()?;
    ob_box.remove_all()?;

    let EntityWithOptionalsConditionFactory {
        optional_nickname,
        ..
    } = new_entitywithoptionals_condition_factory();

    let mut e_none = EntityWithOptionals {
        id: 0,
        required_name: "WithNone".to_string(),
        required_age: 1,
        optional_nickname: None,
        optional_score: None,
        optional_count: None,
        optional_active: None,
        optional_flag: None,
    };
    let mut e_empty = EntityWithOptionals {
        id: 0,
        required_name: "WithEmpty".to_string(),
        required_age: 2,
        optional_nickname: Some("".to_string()),
        optional_score: None,
        optional_count: None,
        optional_active: None,
        optional_flag: None,
    };

    let ids = ob_box.put_many(vec![&mut e_none, &mut e_empty])?;

    // Read back
    let loaded_none = ob_box.get(ids[0])?.expect("Should exist");
    let loaded_empty = ob_box.get(ids[1])?.expect("Should exist");

    // None should remain None
    assert_eq!(loaded_none.optional_nickname, None, "None should not become empty string");

    // Empty string should remain Some("")
    assert_eq!(loaded_empty.optional_nickname, Some("".to_string()), "Empty string should persist");

    // is_null should find only the None entry
    assert_eq!(
        1,
        ob_box.query(&mut optional_nickname.is_null())?.count()?,
        "Only None entry should be NULL"
    );

    // is_not_null should find the empty string entry
    assert_eq!(
        1,
        ob_box.query(&mut optional_nickname.is_not_null())?.count()?,
        "Empty string entry should be NOT NULL"
    );

    ob_box.remove_all()?;
    Ok(())
}

// =============================================================================
// 14. Edge case: zero values vs None for numeric optional
// =============================================================================

#[test]
#[serial]
fn test_zero_vs_none_for_optional_numeric() -> error::Result<()> {
    let store = setup_store()?;
    let mut ob_box = store.get_box::<EntityWithOptionals>()?;
    ob_box.remove_all()?;

    let EntityWithOptionalsConditionFactory {
        optional_count,
        ..
    } = new_entitywithoptionals_condition_factory();

    let mut e_none = EntityWithOptionals {
        id: 0,
        required_name: "NoneCount".to_string(),
        required_age: 1,
        optional_nickname: None,
        optional_score: None,
        optional_count: None,
        optional_active: None,
        optional_flag: None,
    };
    let mut e_zero = EntityWithOptionals {
        id: 0,
        required_name: "ZeroCount".to_string(),
        required_age: 2,
        optional_nickname: None,
        optional_score: None,
        optional_count: Some(0),
        optional_active: None,
        optional_flag: None,
    };

    let ids = ob_box.put_many(vec![&mut e_none, &mut e_zero])?;

    // Read back
    let loaded_none = ob_box.get(ids[0])?.expect("Should exist");
    let loaded_zero = ob_box.get(ids[1])?.expect("Should exist");

    assert_eq!(loaded_none.optional_count, None, "None should remain None");
    assert_eq!(loaded_zero.optional_count, Some(0), "Some(0) should remain Some(0)");

    // is_null should find only None
    assert_eq!(
        1,
        ob_box.query(&mut optional_count.is_null())?.count()?,
        "Only None entry should be NULL"
    );

    // is_not_null should find Some(0)
    assert_eq!(
        1,
        ob_box.query(&mut optional_count.is_not_null())?.count()?,
        "Some(0) should be NOT NULL"
    );

    ob_box.remove_all()?;
    Ok(())
}
