use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Debug)]
pub enum Input {
    Text(String),
    File(BufReader<File>)
}

impl From<String> for Input {
    fn from(value: String) -> Self {
        match File::open(Path::new(&value)) {
            Ok(file) => Input::File(BufReader::new(file)),
            Err(_) => Input::Text(value)
        }
    }
}