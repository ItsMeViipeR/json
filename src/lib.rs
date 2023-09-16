use serde_json::{json, from_str};
use std::fs::File;
use std::io::{Read, Write};
use std::error::Error;

pub type JsonValue = serde_json::Value;

#[derive(Debug)]
pub struct JsonEditor {
    pub(crate) json_data: JsonValue,
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

    pub fn remove_key(&mut self, key: &str) -> Result<(), Box<dyn Error>> {
        if !self.json_data.get(key).is_some() {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Key not found",
            )));
        }
    
        self.json_data.as_object_mut().expect("Failed to get from mut").remove(key).expect("Failed to delete key");
    
        let mut file = match File::create(&self.path) {
            Ok(file) => file,
            Err(_) => return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "File not found",
            )))
        };
    
        match file.write_all(self.json_data.to_string().as_bytes()) {
            Ok(_) => Ok(()),
            Err(_) => return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "File not found",
            )))
        }
    }

    pub fn open_from_file(path: &'static str) -> Result<JsonEditor, Box<dyn Error>> {
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(_) => {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "File not found",
                )));
            }
        };

        let mut content = String::new();
        
        match file.read_to_string(&mut content) {
            Ok(_) => (),
            Err(_) => {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Invalid JSON data",
                )));
            }
        };

        let json_data = match serde_json::from_str(&content) {
            Ok(u) => u,
            Err(_) => {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Invalid JSON data",
                )));
            }
        };

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

        let existing_data: JsonValue = match serde_json::from_reader(file) {
            Ok(u) => u,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Invalid JSON data",
                ));
            }
        };
        let mut updated_data = existing_data.clone();

        for (key, value) in self.json_data.as_object().unwrap() {
            updated_data[key] = value.clone();
        }

        let mut file = match File::create(path) {
            Ok(file) => file,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "File not found",
                ));
            }
        };

        file.write_all(serde_json::to_string_pretty(&updated_data).unwrap().as_bytes())
    }

    pub fn save_to_file_new(&self, path: &str) -> std::io::Result<()> {
        let mut file = match File::create(path) {
            Ok(file) => file,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "File not found",
                ));
            }
        };

        file.write_all(serde_json::to_string_pretty(&self.json_data).unwrap().as_bytes())
    }

    pub fn open_file(&mut self, path: &str) -> std::io::Result<()> {
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "File not found",
                ));
            }
        };

        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Ok(_) => (),
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Invalid JSON data",
                ));
            }
        };

        self.json_data = match from_str(&contents) {
            Ok(u) => u,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Invalid JSON data",
                ));
            }
        };

        Ok(())
    }

    pub fn get_value(&self, key: &str) -> Option<&JsonValue> {
        self.json_data.get(key)
    }

    pub fn read_file(&self) -> std::io::Result<JsonValue> {
        let file = match File::open(&self.path) {
            Ok(file) => file,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "File not found",
                ));
            }
        };

        let reader = std::io::BufReader::new(file);
        let u: JsonValue = match serde_json::from_reader(reader) {
            Ok(u) => u,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Invalid JSON data",
                ));
            }
        };
        
        Ok(u)
    }
}