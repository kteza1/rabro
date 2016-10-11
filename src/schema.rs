use error::Result;
use serde_json::{self, Value};

pub enum SchemaType {
    Null,
    Boolean,
    Int,
    Long,
    Float,
    Double,
    String,
    Bytes,
    Record,
    Enum,
    Array,
    Map,
    Union,
    Fixed,
    Recursive,
}

// pub trait Schema {
//     fn avro_type() -> Result<SchemaType>;
//     // If this is a record, enum or fixed, returns its name,
//     // otherwise the name of the primitive type.
//     fn name() -> String;
//     // Gets a custom non-reserved property from this schema
//     // and a bool representing if it exists.
//     fn property(key: &str) -> Option<String>;
//     // Converts this schema to its JSON representation.
//     fn to_string() -> String;

//     fn parse(json: &str) -> Value {
//         let j = serde_json::from_str(json).unwrap();
//         j
//     };
// }

#[derive(Debug)]
pub struct Schema {
    pub schema: serde_json::Value,
}

impl Schema {
    pub fn schema_from_json_object(s: serde_json::Value) {
        
    }

    pub fn parse(json: &str) -> Self {
        let j = serde_json::from_str(json).unwrap();
        Schema {
            schema: j
        }
    }
}
