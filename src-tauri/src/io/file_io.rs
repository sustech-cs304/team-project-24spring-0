use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::time::SystemTime;

/// Read file with  std::Path.
pub fn read_file(file_path: &Path) -> Result<String, String> {
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

/// Write file with  std::Path.
pub fn write_file(file_path: &Path, data: &str) -> Result<bool, String> {
    let file = File::create(file_path);
    match file {
        Ok(mut file) => match file.write_all(data.as_bytes()) {
            Ok(_) => Ok(true),
            Err(e) => Err(format!("Error: {}", e)),
        },
        Err(e) => Err(format!("Error: {}", e)),
    }
}

/// Read file with string path.
pub fn read_file_str(file_path_str: &str) -> Result<String, String> {
    let file_path = std::path::Path::new(file_path_str);
    read_file(file_path)
}

/// write file with string path
pub fn write_file_str(file_path_str: &str, data: &str) -> Result<bool, String> {
    let file_path = std::path::Path::new(file_path_str);
    write_file(file_path, data)
}

pub fn get_last_modified_str(file_path: &str) -> Result<SystemTime, String> {
    let file_path = std::path::Path::new(file_path);
    get_last_modified(file_path)
}

pub fn get_last_modified(file_path: &Path) -> Result<SystemTime, String> {
    let metadata = std::fs::metadata(file_path);
    match metadata {
        Ok(metadata) => Ok(metadata.modified().unwrap()),
        Err(e) => Err(format!("Error: {}", e)),
    }
}
