use std::collections::HashMap;
use std::rc::Rc;
use std::io::{Write, Read};

use serde_json;

use error::{Error, Result};
use encode::Encodable;
use decode::Decodable;
use primitives::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Property {
    pub name: String,
    pub value: serde_json::Value,
}

impl Property {
    pub fn new(name: String, value: serde_json::Value) -> Self {
        Property {
            name: name,
            value: value,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    pub name: String,
    pub doc: Option<String>,
    pub properties: Vec<Property>,
    pub ty: Schema,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RecordSchema {
    pub name: String,
    pub doc: Option<String>,
    pub properties: Vec<Property>,
    pub fields: Vec<Field>,
    pub field_indexes: HashMap<String, usize>,
}

fn create_field_indexes(fields: &Vec<Field>) -> HashMap<String, usize> {
    fields.iter().enumerate().fold(HashMap::new(), |mut map, (i, f)| {
        map.insert(f.name.clone(), i);
        map
    })
}

impl RecordSchema {
    pub fn new(name: String,
               doc: Option<String>,
               properties: Vec<Property>,
               fields: Vec<Field>)
               -> Self {
        let indexes = create_field_indexes(&fields);
        RecordSchema {
            name: name,
            doc: doc,
            properties: properties,
            fields: fields,
            field_indexes: indexes,
        }
    }
}

// impl Encodable for RecordSchema {
//     fn encode<W: Write>(self, writer: &mut W) -> Result<()> {
//         for field in self.fields {

//         }
//         Ok(())
//     }
// }

#[derive(Debug, Clone, PartialEq)]
pub struct EnumSymbol {
    pub name: String,
    pub doc: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumSchema {
    pub name: String,
    pub doc: Option<String>,
    pub properties: Vec<Property>,
    pub symbols: Vec<EnumSymbol>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FixedSchema {
    pub name: String,
    pub doc: Option<String>,
    pub properties: Vec<Property>,
    pub size: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Message {
    pub name: String,
    pub doc: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Protocol {
    pub name: String,
    pub doc: Option<String>,
    pub properties: Vec<Property>,
    pub tys: Vec<Schema>,
    pub messages: Vec<Message>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Schema {
    Null,
    Boolean,
    Int,
    Long,
    Float,
    Double,
    Bytes,
    String,
    Record(Rc<RecordSchema>),
    Error(Rc<RecordSchema>),
    Enum(Rc<EnumSchema>),
    Array {
        items: Box<Schema>,
    },
    Map {
        values: Box<Schema>,
    },
    Union {
        tys: Vec<Schema>,
    },
    Fixed(Rc<FixedSchema>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    Boolean(bool),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    Bytes(Vec<u8>),
    String(String),
    Record(Rc<RecordSchema>, Vec<Value>),
    Enum(Rc<EnumSchema>, i32),
    Array(Vec<Value>),
    Map(HashMap<String, Value>),
    Fixed(Rc<FixedSchema>, Vec<u8>),
}

impl Encodable for Value {
    fn encode<W: Write>(self, writer: &mut W) -> Result<()> {
        match self {
            Value::Null => Ok(()),
            Value::Boolean(b) => b.encode(writer),
            Value::Int(i) => i.encode(writer),
            Value::Long(l) => l.encode(writer),
            Value::Float(f) => f.encode(writer),
            Value::Double(d) => d.encode(writer),
            Value::Bytes(b) => b.encode(writer),
            Value::String(s) => s.encode(writer),
            Value::Record(schema, values) => {
                // TODO: Validate Value with Schema
                for field in schema.fields.iter() {

                }
                Ok(())
            }
            _ => Ok(()),
        }
    }
}
