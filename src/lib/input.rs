use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Debug)]
pub enum Input<'a> {
    Text(&'a str),
    File(BufReader<File>)
}

impl<'a> From<&'a str> for Input<'a> {
    fn from(value: &'a str) -> Self {
        match File::open(Path::new(&value)) {
            Ok(file) => Input::File(BufReader::new(file)),
            Err(_) => Input::Text(value)
        }
    }
}