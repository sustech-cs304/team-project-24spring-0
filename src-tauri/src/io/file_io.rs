use std::fs::File;
use std::io::prelude::*;

pub fn read_file(file_path_str: &str) -> Result<String, String> {
    let file_path = std::path::Path::new(file_path_str);
    let file = File::open(file_path);
    match file {
        Ok(mut file) => {
            let mut contents = String::new();
            match file.read_to_string(&mut contents) {
                Ok(_) => Ok(contents),
                Err(e) => Err(format!("Error: {}", e)),
            }
        }
        Err(e) => Err(format!("Error: {}", e)),
    }
}

pub fn write_file(file_path_str: &str, data: &str) -> Result<bool, String> {
    let file_path = std::path::Path::new(file_path_str);
    let file = File::create(file_path);
    match file {
        Ok(mut file) => match file.write_all(data.as_bytes()) {
            Ok(_) => Ok(true),
            Err(e) => Err(format!("Error: {}", e)),
        },
        Err(e) => Err(format!("Error: {}", e)),
    }
}
