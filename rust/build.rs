extern crate schemafy;
extern crate schemafy_lib;

use schemafy_lib::Expander;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use serde_json;

fn main() -> io::Result<()> {
    let schema_path = Path::new("../schemas/curated-listing.json");
    let json_schema = fs::read_to_string(schema_path).expect("Unable to read JSON schema file");
    let json_schema = serde_json::from_str(&json_schema).unwrap();
    let mut expander = Expander::new(None, "./src/models.rs", &json_schema);
    let code = expander.expand(&json_schema);
    let destination_path = Path::new("./src/models.rs");
    let mut file = File::create(&destination_path).expect("Could not create file");
    file.write_all(code.to_string().as_bytes()).expect("Could not write to file");
    println!("Done creating models");
    Ok(())
}
