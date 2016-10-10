use std::collections::HashMap;
use std::rc::Rc;

use serde_json;

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
