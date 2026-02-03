use objectbox_generator::{id, model_json};
use syn::{punctuated::Pair, DeriveInput};

use crate::property::Property;

// TODO see if uid type = u64 can be parameterized with generics e.g. 0x... 0b... etc.
// TODO see how generics work with this e.g. "struct Gen<T> { field: T }"
// TODO see how fields with Option<T> type, that default to None, and how store deals with this
// TODO check if another attribute macro can mess with our attribute, otherwise panic if another attribute is present
#[derive(Debug)]
pub(crate) struct Entity {
    name: String,
    id: id::IdUid,
    fields: Vec<Property>,
}

fn warn_transient(entity_name: &str, field_name: &str) {
    panic!(
        "Error: There is a field {}::{} with an unsupported type.",
        entity_name, field_name
    );
    // println!("Warning: {}.{} will be considered as a transient", entity_name, field_name);
}

impl Entity {
    /// Unnamed fields are ignored, e.g. nested anonymous unions / structs, like in C.
    pub(crate) fn from_entity_name_and_fields(id: id::IdUid, derive_input: DeriveInput) -> Entity {
        let mut entity = Entity {
            name: derive_input.ident.to_string(),
            id: id,
            fields: Vec::<Property>::new(),
        };
        let Entity {
            name: entity_name,
            id: _,
            fields,
        } = &mut entity;
        if let syn::Data::Struct(ds) = derive_input.data {
            match ds.fields {
                syn::Fields::Named(fields_named) => {
                    fields_named.named.pairs().for_each(|p| {
                        match p {
                            Pair::Punctuated(t, _) => {
                                // TODO check for attribute: #[transient]
                                if let Some(f) = Property::from_syn_field(t) {
                                    if f.field_type == 0 {
                                        warn_transient(&entity_name, &f.name);
                                    } else {
                                        fields.push(f);
                                    }
                                }
                            }
                            Pair::End(t) => {
                                // TODO check for attribute: #[transient]
                                if let Some(f) = Property::from_syn_field(t) {
                                    if f.field_type == 0 {
                                        warn_transient(&entity_name, &f.name);
                                    } else {
                                        fields.push(f);
                                    }
                                }
                            }
                        }
                    });
                }
                _ => {}
            }
        } else {
            panic!("This macro attribute is only applicable on structs");
        }
        if fields.is_empty() {
            panic!("Structs must have at least one attribute / property!");
        }
        entity
    }

    fn get_last_property_id(&self) -> id::IdUid {
        if let Some(field) = self.fields.last() {
            return field.id.clone();
        }
        // TODO throw an error down the road, this should never happen
        // TODO write test with an Entity without properties
        id::IdUid::zero()
    }

    fn get_properties(&self) -> Vec<model_json::ModelProperty> {
        let mut v: Vec<model_json::ModelProperty> = Vec::new();
        for f in self.fields.iter() {
            let flags = if f.flags == 0 { None } else { Some(f.flags) };
            let index_id = if let Some(x) = &f.index_id {
                Some(x.to_string())
            } else {
                None
            };
            let p = model_json::ModelProperty {
                id: f.id.to_string(),
                name: f.name.clone(),
                type_field: f.field_type,
                flags,
                index_id,
                rust_type: f.rust_type.clone(), // Передача rust_type для генерації коду
            };
            v.push(p);
        }
        v
    }

    pub(crate) fn serialize(&self) -> model_json::ModelEntity {
        model_json::ModelEntity {
            id: self.id.to_string(),
            last_property_id: self.get_last_property_id().to_string(),
            name: self.name.clone(),
            properties: self.get_properties(),
            relations: Vec::new(), // TODO
                                   // path: None,
                                   // TODO see flags
        }
    }
}
