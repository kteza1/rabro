#![allow(unused_variables)] 
extern crate rabro;

use rabro::Schema;

#[test]
fn basic() {
    let schema = r#"{
        "namespace": "example.avro",
        "type": "record",
        "name": "User",
        "fields": [
            {"name": "name", "type": "string"}
        ]
    }"#;
    let schema = Schema::parse(schema);
    let schema = schema.schema.as_object().unwrap();
    println!("{:#?}", schema.get("type"));
}