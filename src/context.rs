//use std::error::Error;

//use csv;

use crate::models::ToDo;
/*
pub const MONGODB_URL: &str = "mongodb://localhost:27017/";

pub struct DataContext {
    pub todos: Vec<ToDo>,
}


impl DataContext{
    pub fn init() -> Result<DataContext, Box<dyn Error>>{
        let todos = read_from_file("data/todos.csv")?;

        Ok(DataContext){
            todos,
        }
    }
}

fn read_from_file<T> (path: &str) -> Result<Vec<T>, Box<dyn Error>>
where
    T: serde::de::DeserializedOwned,
    {
        let mut reader = csv::Reader::from_path(path);
        let mut results = Vec::new();
        for result in reader.deserialize(){
            let record: T = result?;
            results.push(record);
        }
        Ok(results);
    }
    */
