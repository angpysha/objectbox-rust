use example::{
    make_factory_map, make_model, new_entity3_condition_factory, Entity3,
    Entity3ConditionFactory,
};
use objectbox::{error, opt::Opt, query::condition::Condition, store::Store};

use serial_test::serial;

trait TesterExt {
    fn given_condition_count(
        &mut self,
        c: &mut Condition<Entity3>,
        expected: usize,
        label: &str,
    ) -> error::Result<()>;
}

impl TesterExt for objectbox::r#box::Box<'_, Entity3> {
    fn given_condition_count(
        &mut self,
        c: &mut Condition<Entity3>,
        expected: usize,
        label: &str,
    ) -> error::Result<()> {
        let q = self.query(c)?;
        let count = q.count()?;
        let found_list = q.find()?;
        assert_eq!(
            expected,
            found_list.len(),
            "Failed for: {label} (count={count}, find={})",
            found_list.len()
        );
        Ok(())
    }
}

#[test]
#[serial]
fn string_contains_tests() -> error::Result<()> {
    let mut model = make_model();
    let opt = Opt::from_model(&mut model)?;
    let trait_map = make_factory_map();
    let store = Store::new(opt, trait_map)?;

    let mut box3 = store.get_box::<Entity3>()?;
    box3.remove_all()?;

    let mut first = Entity3 {
        id: 0,
        hello: "world".to_string(),
    };
    let mut second = Entity3 {
        id: 0,
        hello: "real world".to_string(),
    };
    let mut third = Entity3 {
        id: 0,
        hello: "REAL world".to_string(),
    };
    box3.put(&mut first)?;
    box3.put(&mut second)?;
    box3.put(&mut third)?;

    let Entity3ConditionFactory { hello, .. } = new_entity3_condition_factory();

    // contains "world" - all 3 have "world" (case insensitive by default)
    let mut c = hello.contains("world");
    box3.given_condition_count(&mut c, 3, "contains(world)")?;

    // contains "real" case-insensitive (default) - "real world" and "REAL world"
    let mut c = hello.contains("real");
    box3.given_condition_count(&mut c, 2, "contains(real)")?;

    // contains "xyz" - no match
    let mut c = hello.contains("xyz");
    box3.given_condition_count(&mut c, 0, "contains(xyz)")?;

    Ok(())
}

#[test]
#[serial]
fn string_starts_with_tests() -> error::Result<()> {
    let mut model = make_model();
    let opt = Opt::from_model(&mut model)?;
    let trait_map = make_factory_map();
    let store = Store::new(opt, trait_map)?;

    let mut box3 = store.get_box::<Entity3>()?;
    box3.remove_all()?;

    let mut first = Entity3 {
        id: 0,
        hello: "world".to_string(),
    };
    let mut second = Entity3 {
        id: 0,
        hello: "real world".to_string(),
    };
    let mut third = Entity3 {
        id: 0,
        hello: "REAL world".to_string(),
    };
    box3.put(&mut first)?;
    box3.put(&mut second)?;
    box3.put(&mut third)?;

    let Entity3ConditionFactory { hello, .. } = new_entity3_condition_factory();

    // starts_with "w" - only "world"
    let mut c = hello.starts_with("w");
    box3.given_condition_count(&mut c, 1, "starts_with(w)")?;

    // starts_with "real" - "real world" and "REAL world" (case insensitive)
    let mut c = hello.starts_with("real");
    box3.given_condition_count(&mut c, 2, "starts_with(real)")?;

    // starts_with "xyz" - no match
    let mut c = hello.starts_with("xyz");
    box3.given_condition_count(&mut c, 0, "starts_with(xyz)")?;

    Ok(())
}

#[test]
#[serial]
fn string_ends_with_tests() -> error::Result<()> {
    let mut model = make_model();
    let opt = Opt::from_model(&mut model)?;
    let trait_map = make_factory_map();
    let store = Store::new(opt, trait_map)?;

    let mut box3 = store.get_box::<Entity3>()?;
    box3.remove_all()?;

    let mut first = Entity3 {
        id: 0,
        hello: "world".to_string(),
    };
    let mut second = Entity3 {
        id: 0,
        hello: "real world".to_string(),
    };
    let mut third = Entity3 {
        id: 0,
        hello: "REAL world".to_string(),
    };
    box3.put(&mut first)?;
    box3.put(&mut second)?;
    box3.put(&mut third)?;

    let Entity3ConditionFactory { hello, .. } = new_entity3_condition_factory();

    // ends_with "d" - all 3 end with "d"
    let mut c = hello.ends_with("d");
    box3.given_condition_count(&mut c, 3, "ends_with(d)")?;

    // ends_with " world" - "real world" and "REAL world"
    let mut c = hello.ends_with(" world");
    box3.given_condition_count(&mut c, 2, "ends_with( world)")?;

    // ends_with "xyz" - no match
    let mut c = hello.ends_with("xyz");
    box3.given_condition_count(&mut c, 0, "ends_with(xyz)")?;

    Ok(())
}

#[test]
#[serial]
fn string_eq_ne_tests() -> error::Result<()> {
    let mut model = make_model();
    let opt = Opt::from_model(&mut model)?;
    let trait_map = make_factory_map();
    let store = Store::new(opt, trait_map)?;

    let mut box3 = store.get_box::<Entity3>()?;
    box3.remove_all()?;

    let mut first = Entity3 {
        id: 0,
        hello: "world".to_string(),
    };
    let mut second = Entity3 {
        id: 0,
        hello: "real world".to_string(),
    };
    let mut third = Entity3 {
        id: 0,
        hello: "REAL world".to_string(),
    };
    box3.put(&mut first)?;
    box3.put(&mut second)?;
    box3.put(&mut third)?;

    let Entity3ConditionFactory { hello, .. } = new_entity3_condition_factory();

    // eq "world" - exact match (case insensitive by default)
    let mut c = hello.eq("world".to_string());
    box3.given_condition_count(&mut c, 1, "eq(world)")?;

    // eq "real world" - case insensitive matches both cases
    let mut c = hello.eq("real world".to_string());
    box3.given_condition_count(&mut c, 2, "eq(real world)")?;

    // ne "world" - not "world" -> 2 results
    let mut c = hello.ne("world".to_string());
    box3.given_condition_count(&mut c, 2, "ne(world)")?;

    // ne "a" - none equal "a", so all 3
    let mut c = hello.ne("a".to_string());
    box3.given_condition_count(&mut c, 3, "ne(a)")?;

    // eq "nonexistent" - 0
    let mut c = hello.eq("nonexistent".to_string());
    box3.given_condition_count(&mut c, 0, "eq(nonexistent)")?;

    Ok(())
}

#[test]
#[serial]
fn string_comparison_tests() -> error::Result<()> {
    let mut model = make_model();
    let opt = Opt::from_model(&mut model)?;
    let trait_map = make_factory_map();
    let store = Store::new(opt, trait_map)?;

    let mut box3 = store.get_box::<Entity3>()?;
    box3.remove_all()?;

    let mut first = Entity3 {
        id: 0,
        hello: "apple".to_string(),
    };
    let mut second = Entity3 {
        id: 0,
        hello: "banana".to_string(),
    };
    let mut third = Entity3 {
        id: 0,
        hello: "cherry".to_string(),
    };
    box3.put(&mut first)?;
    box3.put(&mut second)?;
    box3.put(&mut third)?;

    let Entity3ConditionFactory { hello, .. } = new_entity3_condition_factory();

    // lt "banana" - only "apple"
    let mut c = hello.lt("banana".to_string());
    box3.given_condition_count(&mut c, 1, "lt(banana)")?;

    // le "banana" - "apple" and "banana"
    let mut c = hello.le("banana".to_string());
    box3.given_condition_count(&mut c, 2, "le(banana)")?;

    // gt "banana" - only "cherry"
    let mut c = hello.gt("banana".to_string());
    box3.given_condition_count(&mut c, 1, "gt(banana)")?;

    // ge "banana" - "banana" and "cherry"
    let mut c = hello.ge("banana".to_string());
    box3.given_condition_count(&mut c, 2, "ge(banana)")?;

    // lt "a" - nothing less than "a"
    let mut c = hello.lt("a".to_string());
    box3.given_condition_count(&mut c, 0, "lt(a)")?;

    // gt "z" - nothing greater than "z"
    let mut c = hello.gt("z".to_string());
    box3.given_condition_count(&mut c, 0, "gt(z)")?;

    Ok(())
}

#[test]
#[serial]
fn string_in_strings_tests() -> error::Result<()> {
    let mut model = make_model();
    let opt = Opt::from_model(&mut model)?;
    let trait_map = make_factory_map();
    let store = Store::new(opt, trait_map)?;

    let mut box3 = store.get_box::<Entity3>()?;
    box3.remove_all()?;

    let mut first = Entity3 {
        id: 0,
        hello: "world".to_string(),
    };
    let mut second = Entity3 {
        id: 0,
        hello: "real world".to_string(),
    };
    let mut third = Entity3 {
        id: 0,
        hello: "REAL world".to_string(),
    };
    box3.put(&mut first)?;
    box3.put(&mut second)?;
    box3.put(&mut third)?;

    let Entity3ConditionFactory { hello, .. } = new_entity3_condition_factory();

    // in_strings with one match
    let mut c = hello.in_strings(&vec![
        "world".to_string(),
        "does not exist".to_string(),
    ]);
    box3.given_condition_count(&mut c, 1, "in_strings(world, dne)")?;

    // in_strings with two matches (case insensitive: "real world" matches both)
    let mut c = hello.in_strings(&vec![
        "real world".to_string(),
    ]);
    box3.given_condition_count(&mut c, 2, "in_strings(real world)")?;

    // in_strings with no matches
    let mut c = hello.in_strings(&vec![
        "nonexistent".to_string(),
    ]);
    box3.given_condition_count(&mut c, 0, "in_strings(nonexistent)")?;

    Ok(())
}

#[test]
#[serial]
fn string_case_sensitive_tests() -> error::Result<()> {
    let mut model = make_model();
    let opt = Opt::from_model(&mut model)?;
    let trait_map = make_factory_map();
    let store = Store::new(opt, trait_map)?;

    let mut box3 = store.get_box::<Entity3>()?;
    box3.remove_all()?;

    let mut first = Entity3 {
        id: 0,
        hello: "world".to_string(),
    };
    let mut second = Entity3 {
        id: 0,
        hello: "real world".to_string(),
    };
    let mut third = Entity3 {
        id: 0,
        hello: "REAL world".to_string(),
    };
    box3.put(&mut first)?;
    box3.put(&mut second)?;
    box3.put(&mut third)?;

    let Entity3ConditionFactory { hello, .. } = new_entity3_condition_factory();

    // case_sensitive(true) + contains("real") should match only "real world" (not "REAL world")
    let mut c = hello.case_sensitive(true).and(hello.contains("real"));
    box3.given_condition_count(&mut c, 1, "case_sensitive+contains(real)")?;

    // case_sensitive(true) + contains("REAL") should match only "REAL world"
    let mut c = hello.case_sensitive(true).and(hello.contains("REAL"));
    box3.given_condition_count(&mut c, 1, "case_sensitive+contains(REAL)")?;

    // case_sensitive(true) + starts_with("REAL") - only "REAL world"
    let mut c = hello.case_sensitive(true).and(hello.starts_with("REAL"));
    box3.given_condition_count(&mut c, 1, "case_sensitive+starts_with(REAL)")?;

    // case_sensitive(true) + eq("world") - exact case match
    let mut c = hello.case_sensitive(true).and(hello.eq("world".to_string()));
    box3.given_condition_count(&mut c, 1, "case_sensitive+eq(world)")?;

    // case_sensitive(true) + in_strings with "world" - exact case match
    let mut c = hello.case_sensitive(true).and(hello.in_strings(&vec![
        "world".to_string(),
        "does not exist".to_string(),
    ]));
    box3.given_condition_count(&mut c, 1, "case_sensitive+in_strings(world)")?;

    Ok(())
}
