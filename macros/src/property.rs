use std::option::Option;

use objectbox_generator::id;
use objectbox_generator::ob_consts as consts;

use crate::path_visitor::get_idents_from_path;
use crate::IdUidMacroHelper;

// TODO implement flags, reference: https://github.com/objectbox/objectbox-dart/blob/main/generator/lib/src/entity_resolver.dart#L23-L30

/// OBXPropertyType for ToOne relations (same as Dart's OBXPropertyType.Relation)
pub const PROPERTY_TYPE_RELATION: consts::OBXPropertyType = 11;

/// Represents a parsed field from the entity struct
#[derive(Debug)]
pub enum ParsedField {
    /// Regular property (including ToOne, which becomes a property with type Relation)
    Property(Property),
    /// ToMany relation (standalone, not a property)
    Relation(Relation),
}

/// Represents a ToMany standalone relation
#[derive(Debug)]
pub struct Relation {
    pub name: String,           // Field name in the struct (e.g., "teachers")
    pub id: id::IdUid,          // Relation ID
    pub target_name: String,    // Target entity name (e.g., "Teacher")
    pub rust_type: String,      // Full Rust type (e.g., "ToMany<Teacher>")
}

impl Relation {
    pub fn new(name: String, target_name: String) -> Self {
        Relation {
            name: name.clone(),
            id: id::IdUid::zero(),
            target_name: target_name.clone(),
            rust_type: format!("ToMany<{}>", target_name),
        }
    }
}

#[derive(Debug)]
pub struct Property {
    pub name: String,
    /// Custom DB/model name from #[property(name = "...")].
    /// When set, `name` is the Rust field name and `db_name` is the ObjectBox model name.
    /// This allows snake_case in Rust but camelCase in DB, or Rust keyword avoidance.
    pub db_name: Option<String>,
    pub field_type: consts::OBXPropertyType,
    pub id: id::IdUid,
    pub flags: consts::OBXPropertyFlags,
    pub index_id: Option<String>,
    // Rust type string for code generation
    pub rust_type: String, // "String", "Option<String>", "i32", "ToOne<Customer>", etc.
    
    // ToOne relation fields
    pub relation_field: Option<String>,   // Original ToOne field name (e.g., "customer")
    pub relation_target: Option<String>,  // Target entity name (e.g., "Customer")
}

impl Property {
    pub(crate) fn new() -> Self {
        Property {
            name: String::new(),
            db_name: None,
            field_type: 0,
            id: id::IdUid::zero(),
            flags: 0,
            index_id: None,
            rust_type: String::new(),
            relation_field: None,
            relation_target: None,
        }
    }
    
    /// Check if this property is a ToOne relation
    pub fn is_to_one_relation(&self) -> bool {
        self.field_type == PROPERTY_TYPE_RELATION
    }

    pub(crate) fn scan_obx_property_type_and_flags(
        mnv: &syn::MetaNameValue,
    ) -> (consts::OBXPropertyType, consts::OBXPropertyFlags) {
        let mut obx_property_type: consts::OBXPropertyType = 0;
        let mut obx_property_flags: consts::OBXPropertyFlags = 0;

        if let syn::Lit::Int(li) = &mnv.lit {
            let result = li.base10_parse::<consts::OBXPropertyFlags>();
            if let Ok(value) = result {
                if let Some(ident) = mnv.path.get_ident() {
                    let param_name: &str = &ident.to_string();
                    match param_name {
                        "type" => obx_property_type = value,
                        "flags" => obx_property_flags = value,
                        _ => {}
                    }
                }
            }
        }
        (obx_property_type, obx_property_flags)
    }

    /// Parse a syn::Field and return either a Property, Relation, or None
    pub(crate) fn from_syn_field(field: &syn::Field) -> Option<ParsedField> {
        let mut property = Property::new();

        let Property {
            name,
            db_name,
            field_type: obx_property_type,
            id,
            flags: obx_property_flags,
            index_id,
            rust_type,
            relation_field,
            relation_target,
        } = &mut property;

        if let Some(ident) = &field.ident {
            let field_name = ident.to_string();
            name.push_str(&field_name);

            // Pre-detect if field type is String for Dart-compatible index flag selection.
            // Dart uses INDEX_HASH for String fields, INDEXED (value) for others.
            let pre_idents = get_idents_from_path(&field.ty);
            let is_string_type = !pre_idents.is_empty() && {
                let first = pre_idents[0].to_string();
                first == "String"
                    || (first == "Option"
                        && pre_idents.len() >= 2
                        && pre_idents[1].to_string() == "String")
            };

            // Track explicit index type from #[index(type = "hash"/"value"/"hash64")]
            let mut explicit_index_type: Option<String> = None;
            // Track manual index id/uid from #[index(id = X, uid = Y)] or #[unique(id = X, uid = Y)]
            let mut index_id_uid = id::IdUid::zero();

            // Attribute parsing
            for a in field.attrs.iter() {
                // Track which attribute we're processing (for context-sensitive params)
                let mut is_id_attr = false;
                let mut is_index_or_unique_attr = false;

                if let Some(attr_path_ident) = a.path.get_ident() {
                    let attr_name: &str = &attr_path_ident.to_string();
                    match attr_name {
                        "id" => {
                            is_id_attr = true;
                            *obx_property_type = consts::OBXPropertyType_Long;
                            // Match Dart: just ID flag by default.
                            // Use #[id(assignable)] to also set ID_SELF_ASSIGNABLE.
                            *obx_property_flags |= consts::OBXPropertyFlags_ID;
                            *rust_type = "u64".to_string();
                            // Don't return early: allow further attributes (e.g.
                            // #[property(id = X, uid = Y)]) to set schema IDs,
                            // and allow #[id(uid = Y)] shorthand.
                        }
                        "index" => {
                            is_index_or_unique_attr = true;
                            // Just mark as indexed; actual index strategy flag (INDEXED vs
                            // INDEX_HASH) is applied after the loop based on field type.
                            // NOTE: #[index] does NOT imply UNIQUE (matches Dart behavior).
                            *index_id = Some("0:0".to_owned());
                        }
                        "unique" => {
                            is_index_or_unique_attr = true;
                            // UNIQUE flag; index strategy applied after the loop.
                            *obx_property_flags |= consts::OBXPropertyFlags_UNIQUE;
                            *index_id = Some("0:0".to_owned());
                        }
                        "backlink" => {} // TODO: implement backlinks
                        "property" => {}
                        _ => {
                            continue;
                        }
                    }
                }

                if let syn::parse::Result::Ok(m) = a.parse_meta() {
                    match m {
                        syn::Meta::NameValue(mnv) => {
                            id.update_from_scan(&mnv);
                            let (pt, pf) = Self::scan_obx_property_type_and_flags(&mnv);
                            if pt != 0 { *obx_property_type = pt; }
                            *obx_property_flags |= pf;
                        }
                        syn::Meta::List(meta_list) => {
                            meta_list.nested.into_iter().for_each(|nm| {
                                match nm {
                                    syn::NestedMeta::Meta(syn::Meta::NameValue(mnv)) => {
                                        // Route id/uid to index_id_uid when inside #[index] or #[unique],
                                        // otherwise to the property id.
                                        if is_index_or_unique_attr {
                                            index_id_uid.update_from_scan(&mnv);
                                        } else {
                                            id.update_from_scan(&mnv);
                                        }
                                        let (pt, pf) = Self::scan_obx_property_type_and_flags(&mnv);
                                        if pt != 0 { *obx_property_type = pt; }
                                        *obx_property_flags |= pf;

                                        // Parse string-valued parameters:
                                        //   #[property(name = "camelCaseName")]
                                        //   #[index(type = "hash"/"hash64"/"value")]
                                        //   #[unique(on_conflict = "replace")]
                                        if let Some(key_ident) = mnv.path.get_ident() {
                                            let key = key_ident.to_string();
                                            if key == "name" {
                                                if let syn::Lit::Str(ls) = &mnv.lit {
                                                    *db_name = Some(ls.value());
                                                }
                                            } else if key == "type" {
                                                if let syn::Lit::Str(ls) = &mnv.lit {
                                                    explicit_index_type = Some(ls.value());
                                                }
                                            } else if key == "on_conflict" {
                                                if let syn::Lit::Str(ls) = &mnv.lit {
                                                    if ls.value() == "replace" {
                                                        *obx_property_flags |= consts::OBXPropertyFlags_UNIQUE_ON_CONFLICT_REPLACE;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    syn::NestedMeta::Meta(syn::Meta::Path(path)) => {
                                        // Handle bare ident params: #[id(assignable)]
                                        if is_id_attr {
                                            if let Some(path_ident) = path.get_ident() {
                                                if path_ident == "assignable" {
                                                    *obx_property_flags |= consts::OBXPropertyFlags_ID_SELF_ASSIGNABLE;
                                                }
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                            });
                        }
                        _ => {}
                    }
                }
            }

            // Apply manually specified index id/uid if provided
            if index_id_uid.id != 0 || index_id_uid.uid != 0 {
                *index_id = Some(index_id_uid.to_string());
            }

            // Apply index strategy flags based on field type (Dart-compatible behavior):
            //   String fields  → INDEX_HASH  (2048) by default
            //   Other fields   → INDEXED     (8)    by default
            // Can be overridden with #[index(type = "hash"/"hash64"/"value")]
            if index_id.is_some() {
                let index_flag = match explicit_index_type.as_deref() {
                    Some("hash") => consts::OBXPropertyFlags_INDEX_HASH,
                    Some("hash64") => consts::OBXPropertyFlags_INDEX_HASH64,
                    Some("value") => consts::OBXPropertyFlags_INDEXED,
                    None => {
                        if is_string_type {
                            consts::OBXPropertyFlags_INDEX_HASH
                        } else {
                            consts::OBXPropertyFlags_INDEXED
                        }
                    }
                    Some(other) => {
                        panic!(
                            "Unknown index type: '{}'. Use 'hash', 'hash64', or 'value'.",
                            other
                        );
                    }
                };
                *obx_property_flags |= index_flag;
            }

            // If type was already determined by an attribute (e.g. #[id]),
            // skip type detection from the Rust type path
            if *obx_property_type != 0 {
                return Some(ParsedField::Property(property));
            }

            // Parse the type from the Rust type path
            let idents = get_idents_from_path(&field.ty);
            if idents.is_empty() {
                return None;
            }
            
            let first_ident = idents[0].to_string();
            
            // Check for ToOne<T> relation
            if first_ident == "ToOne" && idents.len() >= 2 {
                let target_entity = idents[1..].iter().map(|i| i.to_string()).collect::<String>();
                
                // ToOne creates a property with type Relation
                // Property name is fieldName + "Id" (e.g., "customerId")
                *name = format!("{}Id", field_name);
                *obx_property_type = PROPERTY_TYPE_RELATION;
                *rust_type = format!("ToOne<{}>", target_entity);
                *relation_field = Some(field_name.clone());
                *relation_target = Some(target_entity.clone());
                
                // ToOne relations are automatically indexed
                *obx_property_flags |= consts::OBXPropertyFlags_INDEXED 
                    | consts::OBXPropertyFlags_INDEX_PARTIAL_SKIP_ZERO;
                *index_id = Some("0:0".to_owned());
                
                return Some(ParsedField::Property(property));
            }
            
            // Check for ToMany<T> relation
            if first_ident == "ToMany" && idents.len() >= 2 {
                let target_entity = idents[1..].iter().map(|i| i.to_string()).collect::<String>();
                
                // ToMany creates a standalone relation, not a property
                let relation = Relation::new(field_name, target_entity);
                return Some(ParsedField::Relation(relation));
            }
            
            // Check for Option<T>
            let is_option = first_ident == "Option" && idents.len() >= 2;
            
            if is_option {
                let inner_idents = &idents[1..];
                let inner_type_str = inner_idents.iter().map(|i| i.to_string()).collect::<String>();
                *rust_type = format!("Option<{}>", inner_type_str);
                
                let inner_ident_joined = inner_type_str.as_str();
                *obx_property_type = Self::type_str_to_obx_type(inner_ident_joined);
                *obx_property_flags |= Self::type_str_to_unsigned_flag(inner_ident_joined);
            } else {
                let ident_joined = idents.iter().map(|i| i.to_string()).collect::<String>();
                *rust_type = ident_joined.clone();
                *obx_property_type = Self::type_str_to_obx_type(&ident_joined);
                *obx_property_flags |= Self::type_str_to_unsigned_flag(&ident_joined);
            }

            return Some(ParsedField::Property(property));
        }
        None
    }
    
    /// Convert Rust type string to OBXPropertyType
    fn type_str_to_obx_type(type_str: &str) -> consts::OBXPropertyType {
        match type_str {
            "bool" => consts::OBXPropertyType_Bool,
            "i8" => consts::OBXPropertyType_Byte,
            "i16" => consts::OBXPropertyType_Short,
            "u16" => consts::OBXPropertyType_Short,
            "char" => consts::OBXPropertyType_Char,
            "u32" => consts::OBXPropertyType_Int,
            "i32" => consts::OBXPropertyType_Int,
            "u64" => consts::OBXPropertyType_Long,
            "i64" => consts::OBXPropertyType_Long,
            "f32" => consts::OBXPropertyType_Float,
            "f64" => consts::OBXPropertyType_Double,
            "u8" => consts::OBXPropertyType_Byte,
            "String" => consts::OBXPropertyType_String,
            "VecString" => consts::OBXPropertyType_StringVector,
            "Vecu8" => consts::OBXPropertyType_ByteVector,
            _ => 0,
        }
    }
    
    /// Get UNSIGNED flag for unsigned types
    fn type_str_to_unsigned_flag(type_str: &str) -> consts::OBXPropertyFlags {
        match type_str {
            "u8" | "u16" | "u32" | "u64" => consts::OBXPropertyFlags_UNSIGNED,
            _ => 0,
        }
    }
}
