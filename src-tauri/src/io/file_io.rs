use std::{error::Error, fs::File, io::prelude::*, path::Path, time::SystemTime};

use crate::types::ResultVoid;

/// Read file with  std::Path.
pub fn read_file(file_path: &Path) -> Result<String, Box<dyn Error + Send + Sync>> {
    let file = File::open(file_path);
    match file {
        Ok(mut file) => {
            let mut contents = String::new();
            match file.read_to_string(&mut contents) {
                Ok(_) => Ok(contents),
                Err(e) => Err(Box::new(e)),
            }
        }
        Err(e) => Err(Box::new(e)),
    }
}

/// get file last modified time.
pub fn get_last_modified(file_path: &Path) -> Result<SystemTime, Box<dyn Error + Send + Sync>> {
    let metadata = std::fs::metadata(file_path);
    match metadata {
        Ok(metadata) => Ok(metadata.modified().unwrap()),
        Err(e) => Err(Box::new(e)),
    }
}

/// Write file with  std::Path.
pub fn write_file(file_path: &Path, data: &str) -> ResultVoid {
    let mut file = File::create(file_path)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}

/// Read file with string path.
pub fn read_file_str(file_path_str: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
    let file_path = Path::new(file_path_str);
    read_file(file_path)
}

/// write file with string path
pub fn write_file_str(file_path_str: &str, data: &str) -> ResultVoid {
    let file_path = Path::new(file_path_str);
    write_file(file_path, data)
}

/// get file last modified time with string path.
pub fn get_last_modified_str(file_path: &str) -> Result<SystemTime, Box<dyn Error + Send + Sync>> {
    let file_path = Path::new(file_path);
    get_last_modified(file_path)
}
