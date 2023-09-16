use serde_json::{Value, json, from_str};
use std::fs::File;
use std::io::{Read, Write};
use std::error::Error;

#[derive(Debug)]
pub struct JsonEditor {
    pub json_data: Value,
    pub(crate) path: &'static str,
}

impl JsonEditor {
    pub fn new(path: &'static str) -> JsonEditor {
        JsonEditor { json_data: json!({}), path }
    }

    pub fn add_key<T>(&mut self, key: &str, value: T) -> Result<(), &'static str>
    where
        T: serde::Serialize,
    {
        if self.json_data.get(key).is_some() {
            return Err("Key already exists");
        }
        self.json_data[key] = json!(value);
        Ok(())
    }

    pub fn update_key<T>(&mut self, key: &str, value: T)
    where
        T: serde::Serialize,
    {
        let data = &mut self.json_data;
        if !data.is_object() {
            *data = json!({});
        }
        self.json_data[key] = json!(value);
    }

    pub fn remove_key(&mut self, key: &str) -> Result<(), &'static str> {
        if !self.json_data.get(key).is_some() {
            return Err("Key not found");
        }
    
        self.json_data.as_object_mut().expect("Failed to get from mut").remove(key).expect("Failed to delete key");
    
        let mut file = match File::create(&self.path) {
            Ok(file) => file,
            Err(_) => return Err("Failed to open file"),
        };
    
        match file.write_all(self.json_data.to_string().as_bytes()) {
            Ok(_) => Ok(()),
            Err(_) => return Err("Failed to write to file"),
        }
    }

    pub fn open_from_file(path: &'static str) -> Result<JsonEditor, Box<dyn Error>> {
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let json_data = serde_json::from_str(&content)?;
        Ok(JsonEditor { json_data, path })
    }

    /// Overriding the save_to_file method to prevent the existing file from being overwritten.
    /// Instead, read the existing data from the file, merge the new data with it,
    /// and then write the updated data back to the file.
    pub fn save_to_file(&self, path: &str) -> std::io::Result<()> {
        let file = match File::open(path) {
            Ok(file) => file,
            Err(_) => {
                return self.save_to_file_new(path);
            }
        };
        let existing_data: Value = serde_json::from_reader(file)?;
        let mut updated_data = existing_data.clone();
        for (key, value) in self.json_data.as_object().unwrap() {
            updated_data[key] = value.clone();
        }
        let mut file = File::create(path)?;
        file.write_all(serde_json::to_string_pretty(&updated_data).unwrap().as_bytes())
    }

    pub fn save_to_file_new(&self, path: &str) -> std::io::Result<()> {
        let mut file = File::create(path)?;
        file.write_all(serde_json::to_string_pretty(&self.json_data).unwrap().as_bytes())
    }

    pub fn open_file(&mut self, path: &str) -> std::io::Result<()> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        self.json_data = from_str(&contents).unwrap();
        Ok(())
    }
}