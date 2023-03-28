use chrono::{TimeZone, Utc};
use mongodb::bson::doc;

use std::fs::File;
use bson::Document;

pub fn bson_reader() {
    let mut f = File::open("todos.bson").unwrap();

    while let Ok(deserialized) = Document::from_reader(&mut f) {
        println!("{:?}", deserialized);
    }
}