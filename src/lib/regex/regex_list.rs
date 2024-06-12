use super::regex_item::{Regex};

#[derive(Debug, PartialEq, Eq)]
pub struct RegexList {
    list: Vec<Regex>
}

impl RegexList {
    pub fn new() -> Self {
        RegexList{list: Vec::new()}
    }
}


impl TryFrom<&str> for RegexList {
    type Error = ();

    fn try_from(mut value: &str) -> Result<Self, Self::Error> {
        let mut reg_list = RegexList::new();

        while let Ok(regex) = Regex::try_from(value) {
            let len = regex.len();
            reg_list.list.push(regex);
            value = &value[len..];
        }

        Ok(reg_list)
    }
}