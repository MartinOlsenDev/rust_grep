use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug)]
pub enum Input {
    Text(String),
    File(String)
}

impl TryFrom<String> for Input {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut output = String::new();

        match File::open(Path::new(&value)) {
            Ok(mut file) => {
                file.read_to_string(&mut output).map_err(|_| ())?;
                Ok(Input::File(output))
            },
            Err(_) => Ok(Input::Text(value))
        }
    }
}