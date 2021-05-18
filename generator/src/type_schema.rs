use std::io::Stderr;

use OASchemaProp::*;
use Schema::*;

use crate::schema;
use crate::schema::{MayRef, Schema as OASchema};
use serde_json::Value;

enum Schema {
    // extra
    Any,

    // primitives
    String,
    Number,
    // became f64
    Integer,
    // became i64
    Boolean,
    Null,
    // became ()
    Enum(Vec<String>),

    // formatteds
    DateTime,
    // RFC-3339 date-time in string
    Time,
    // RFC-3339 time(full-time) in string
    Date,
    // RFC-3339 date(full-date) in string
    Ipv4,
    // RFC 2673 IPv4 (dotted-quad) in string
    Ipv6,
    // RFC 2373 IPv6 in string
    Uri,
    // RFC 3986 URI in string, including relative
    Regex,// ECMA 262 Regular Expression

    // littile complex
    Array(Box<Schema>),
    //Object
    //Array
    Ref { name: String },
    Nullable(Box<Schema>),
}

impl From<MayRef<Box<OASchema>>> for Schema {
    fn from(from: MayRef<Box<OASchema>>) -> Self {
        match from {
            MayRef::Ref(schema::Reference { ref_ }) => Ref { name: ref_ },
            MayRef::Value(schema) => Schema::from(*schema),
        }
    }
}

impl From<MayRef<OASchema>> for Schema {
    fn from(from: MayRef<OASchema>) -> Self {
        match from {
            MayRef::Ref(schema::Reference { ref_ }) => Ref { name: ref_ },
            MayRef::Value(schema) => Schema::from(schema),
        }
    }
}

impl From<OASchema> for Schema {
    fn from(mut from: OASchema) -> Self {
        if let Some(format) = &from.format {
            match format.as_str() {
                "date-time" => return DateTime,
                "time" => return Time,
                "date" => return Date,
                "ipv4" => return Ipv4,
                "ipv6" => return Ipv6,
                "regex" => return Regex,
                _ => {} // unsupported format: ignore
            }
        }

        if let Some(enum_) = from.enum_ {
            if enum_.iter().all(|x| x.is_null()) {
                return Null;
            }
            let strings: Vec<_> = enum_.into_iter()
                .map(|x| if let Value::String(str) = x {
                    str
                } else {
                    panic!("enum value must be string")
                })
                .collect();
            return Enum(strings);
        }

        // primitives
        if let Some(type_) = &from.type_ {
            match type_.as_str() {
                "string" => return String,
                "number" => return Number,
                "integer" => return Integer,
                "boolean" => return Boolean,
                "null" => return Null,
                "array" => {
                    return Array(Box::new(from.items.map(|x| Schema::from(x)).unwrap_or(Any)));
                }
                "object" => {}
                _ => panic!("invalid type: {}", type_),
            }
        }

        // check disallows
        match (
            from.one_of.is_empty(),
            from.any_of.is_empty(),
            from.all_of.is_empty(),
        ) {
            (true, true, true) => {}
            (false, true, true) => {}
            (true, false, true) => {}
            (true, true, false) => {}
            _ => panic!("invalid schema: found multiple of oneOf, anyOf, and allOf"),
        }

        if !from.all_of.is_empty() {
            if !is_empty_schema(&schema, &[PAllOf, PNullable]) {
                let mut all_of = std::mem::take(&mut from.all_of);
                all_of.push(MayRef::Value(from));
                from = OASchema::default();
                from.all_of = all_of;
            }
        }

        if from.nullable == Some(true) && from.all_of.len() == 1 {
            return Nullable(Box::new(from.all_of.remove(0).into()));
        }

        ;
        ;
    }
}

enum OASchemaProp {
    PAdditionalProperties,
    PDefault,
    PDeprecated,
    PDescription,
    PEnum_,
    PExample,
    PExternalDocs,
    PFormat,
    PNot,
    PNullable,
    PProperties,
    PReadOnly,
    PTitle,
    PType_,
    PUniqueItems,
    PWriteOnly,
    PAllOf,
    PAnyOf,
    POneOf,
    PRequired,
}

fn is_empty_schema(schema: &OASchema, excludes: &[OASchemaProp]) -> bool {
    if !excludes.contains(&PAdditionalProperties) && !schema.additional_properties.is_none() {
        return false;
    }
    if !excludes.contains(&PDefault) && !schema.default.is_none() {
        return false;
    }
    if !excludes.contains(&PDeprecated) && !schema.deprecated.is_none() {
        return false;
    }
    if !excludes.contains(&PDescription) && !schema.description.is_none() {
        return false;
    }
    if !excludes.contains(&PEnum_) && !schema.enum_.is_none() {
        return false;
    }
    if !excludes.contains(&PExample) && !schema.example.is_none() {
        return false;
    }
    if !excludes.contains(&PExternalDocs) && !schema.external_docs.is_none() {
        return false;
    }
    if !excludes.contains(&PFormat) && !schema.format.is_none() {
        return false;
    }
    // items: not for object
    if !excludes.contains(&PNot) && !schema.not.is_none() {
        return false;
    }
    if !excludes.contains(&PNullable) && !schema.nullable.is_none() {
        return false;
    }
    if !excludes.contains(&PProperties) && !schema.properties.is_none() {
        return false;
    }
    if !excludes.contains(&PReadOnly) && !schema.read_only.is_none() {
        return false;
    }
    if !excludes.contains(&PTitle) && !schema.title.is_none() {
        return false;
    }
    if !excludes.contains(&PType_) && !schema.type_.is_none() {
        return false;
    }
    if !excludes.contains(&PUniqueItems) && !schema.unique_items.is_none() {
        return false;
    }
    if !excludes.contains(&PWriteOnly) && !schema.write_only.is_none() {
        return false;
    }
    if !excludes.contains(&PAllOf) && !schema.all_of.is_empty() {
        return false;
    }
    if !excludes.contains(&PAnyOf) && !schema.any_of.is_empty() {
        return false;
    }
    if !excludes.contains(&POneOf) && !schema.one_of.is_empty() {
        return false;
    }
    if !excludes.contains(&PRequired) && !schema.required.is_empty() {
        return false;
    }
    return true;
}

/*
allOf
- nullable + single -> Option<>
- multiple
oneOf: copy origin
 */
