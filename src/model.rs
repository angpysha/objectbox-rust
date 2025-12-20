#![allow(dead_code)]
use crate::{c, error::Error};
use std::{ffi, ptr};

/// Model is used to define a database model. Use as a fluent interface (builder pattern)
pub struct Model {
    pub(crate) obx_model: *mut c::OBX_model,
    pub(crate) error: Option<Error>,
    pub(crate) ptr_consumed: bool,
}

impl Drop for Model {
    fn drop(&mut self) {
        if !self.ptr_consumed {
            self.error = c::call(unsafe { c::obx_model_free(self.obx_model) }).err();
            self.obx_model = std::ptr::null_mut();
        }

        if let Some(err) = &self.error {
            eprintln!("Error: {err}");
        }
    }
}

impl Model {
    pub fn new() -> Self {
        match c::new_mut(unsafe { c::obx_model() }) {
            // TODO map on Ok
            Ok(c_ptr) => Model {
                obx_model: c_ptr,
                error: None,
                ptr_consumed: false,
            },
            Err(e) => Model {
                obx_model: ptr::null_mut(),
                error: Some(e),
                ptr_consumed: false,
            },
        }
    }

    /// Create an entity.
    pub fn entity(mut self, name: &str, id: c::obx_schema_id, uid: c::obx_uid) -> Self {
        if self.error.is_none() {
            let c_name = ffi::CString::new(name).unwrap();
            self.error =
                c::call(unsafe { c::obx_model_entity(self.obx_model, c_name.as_ptr(), id, uid) })
                    .err();
        }
        self
    }

    /// Inform the model about the last entity that was ever defined in the model.
    pub fn last_entity_id(self, id: c::obx_schema_id, uid: c::obx_uid) -> Self {
        if self.error.is_none() {
            unsafe { c::obx_model_last_entity_id(self.obx_model, id, uid) }
        }
        self
    }

    /// Inform the model about the last index that was ever defined in the model.
    pub fn last_index_id(self, id: c::obx_schema_id, uid: c::obx_uid) -> Self {
        if self.error.is_none() {
            unsafe { c::obx_model_last_index_id(self.obx_model, id, uid) }
        }
        self
    }

    /// Inform the model about the last relation that was ever defined in the model.
    pub fn last_relation_id(self, id: c::obx_schema_id, uid: c::obx_uid) -> Self {
        if self.error.is_none() {
            unsafe { c::obx_model_last_relation_id(self.obx_model, id, uid) }
        }
        self
    }

    /// Inform the model about the last property that was ever defined on the entity.
    /// Finishes building the entity, returning the parent Model.
    pub fn last_property_id(mut self, id: c::obx_schema_id, uid: c::obx_uid) -> Self {
        if self.error.is_none() {
            self.error =
                c::call(unsafe { c::obx_model_entity_last_property_id(self.obx_model, id, uid) })
                    .err();
        }

        self
    }

    /// Create a property.
    pub fn property(
        mut self,
        name: &str,
        id: c::obx_schema_id,
        uid: c::obx_uid,
        // type === typedef, is a reserved keyword, intentional
        typ: c::OBXPropertyType,
        flags: u32,
    ) -> Self {
        if self.error.is_none() {
            let c_name = ffi::CString::new(name).unwrap();

            self.error = c::call(unsafe {
                c::obx_model_property(self.obx_model, c_name.as_ptr(), typ, id, uid)
            })
            .err();

            if let Some(err) = &self.error {
                eprintln!("{err}")
            }

            self.error =
                c::call(unsafe { c::obx_model_property_flags(self.obx_model, flags) }).err();

            if let Some(err) = &self.error {
                eprintln!("{err}")
            }
        }

        self
    }

    /// Declare an index on the last created property.
    pub fn property_index(mut self, id: c::obx_schema_id, uid: c::obx_uid) -> Self {
        if self.error.is_none() {
            self.error =
                c::call(unsafe { c::obx_model_property_index_id(self.obx_model, id, uid) }).err();
        }
        self
    }

    /// Declare a to-one relation on the last created property.
    /// No need to declare the index separately using property_index(), it's created automatically.
    pub fn property_relation(
        mut self,
        target_entity_name: &str,
        index_id: c::obx_schema_id,
        index_uid: c::obx_uid,
    ) -> Self {
        if self.error.is_none() {
            let c_name = ffi::CString::new(target_entity_name).unwrap();
            self.error = c::call(unsafe {
                c::obx_model_property_relation(self.obx_model, c_name.as_ptr(), index_id, index_uid)
            })
            .err();
        }
        self
    }

    /// Declare a standalone to-many relation between this entity and another one
    pub fn relation(
        mut self,
        relation_id: c::obx_schema_id,
        relation_uid: c::obx_uid,
        target_entity_id: c::obx_schema_id,
        target_entity_uid: c::obx_uid,
    ) -> Self {
        if self.error.is_none() {
            self.error = c::call(unsafe {
                c::obx_model_relation(
                    self.obx_model,
                    relation_id,
                    relation_uid,
                    target_entity_id,
                    target_entity_uid,
                )
            })
            .err();
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn model_builder_positive() {
        let model = Model::new()
            .entity("A", 1, 1)
            .property(
                "id",
                1,
                101,
                c::OBXPropertyType_Long,
                c::OBXPropertyFlags_ID,
            )
            .property("text", 2, 202, c::OBXPropertyType_String, 102)
            .property_index(1, 101)
            .last_property_id(2, 202)
            .entity("B", 2, 2)
            .property(
                "id",
                1,
                301,
                c::OBXPropertyType_Long,
                c::OBXPropertyFlags_ID,
            )
            .property("number", 2, 202, c::OBXPropertyType_Int, 0)
            .last_property_id(2, 202)
            .last_entity_id(2, 2)
            .last_index_id(1, 301);

        assert!(model.error.is_none());
    }

    #[test]
    fn big_model_test() {
        let model = Model::new()
            .entity("Entity", 1, 12802433372377933144)
            .property("id", 1, 16303625144254194803, 6, 129)
            .property("index_u32", 2, 17348581232598351063, 5, 8232)
            .property_index(1, 15966762507251846644)
            .property("t_bool", 3, 1463975161829178694, 1, 0)
            .property("t_u8", 4, 8643677704739194959, 2, 8192)
            .property("t_i8", 5, 16142315373739492817, 2, 0)
            .property("t_i16", 6, 8726370263402291511, 3, 0)
            .property("t_u16", 7, 4525767685591106924, 3, 8192)
            .property("unique_i32", 8, 12320118081678770411, 5, 32)
            .property_index(2, 13718990065992865290)
            .property("t_i32", 9, 2724625488925209408, 5, 0)
            .property("t_u32", 10, 1997082525214322396, 5, 8192)
            .property("t_u64", 11, 18050249220377943096, 6, 8192)
            .property("t_i64", 12, 4771075407746354871, 6, 0)
            .property("t_f32", 13, 7496023529852242558, 7, 0)
            .property("t_f64", 14, 6428146482089461088, 8, 0)
            .property("t_string", 15, 15905456625202323974, 9, 0)
            .property("t_char", 16, 17061890276107621552, 4, 0)
            .property("t_vec_string", 17, 3460829531832782193, 30, 0)
            .property("t_vec_bytes", 18, 1384275525893232918, 23, 0)
            .last_property_id(18, 1384275525893232918)
            .entity("Entity2", 2, 2058930340149009603)
            .property("id", 1, 2084036648998826750, 6, 129)
            .property("index_u64", 2, 14743283183353881578, 6, 8232)
            .property_index(3, 10365057831981851219)
            .last_property_id(2, 14743283183353881578)
            .entity("Entity3", 3, 10267361146166351390)
            .property("id", 1, 8063586118144354481, 6, 129)
            .last_property_id(1, 8063586118144354481)
            .last_entity_id(3, 10267361146166351390)
            .last_index_id(3, 10365057831981851219);

        assert!(model.error.is_none());
    }

    #[test]
    fn model_builder_negative() {
        let model = Model::new().entity("A", 1, 1).last_property_id(0, 0);

        let expected_err = format!(
            "{} {} Argument condition \"property_id\" not met",
            c::OBX_ERROR_ILLEGAL_ARGUMENT,
            0
        );

        let actual_err = format!("{}", model.error.as_ref().unwrap());
        println!("expected: {}", &expected_err);
        println!("actual: {}", &actual_err);
        assert!(actual_err.starts_with(&expected_err));
    }
}
