use ropey::Rope;
use std::fs::File;
use std::io::{BufReader, BufWriter};

struct Text {
    data: Rope,
    path: String,
    last_save: u64,
}

impl Text {
    pub fn open_file(&mut self, path: &str) -> Result<bool, std::io::Error> {
        self.data = Rope::from_reader(BufReader::new(File::open(path)?))?;
        self.path = path.to_string();
        Result::Ok(true)
    }

    pub fn from_str(&mut self, text: &str) {
        self.data = Rope::from_str(text);
    }

    pub fn save_file(&self) -> Result<bool, std::io::Error> {
        let file = File::create(&self.path)?;
        let mut writer = BufWriter::new(file);
        self.data.write_to(&mut writer)?;
        Result::Ok(true)
    }

    pub fn save_as(&self, path: &str) -> Result<bool, std::io::Error> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        self.data.write_to(&mut writer)?;
        Result::Ok(true)
    }

    pub fn get_rope(&mut self) -> &mut Rope {
        &mut self.data
    }

    //https://docs.rs/ropey/latest/ropey/index.html
}
