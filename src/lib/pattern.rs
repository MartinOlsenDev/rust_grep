use unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation as US;

const DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
const UPPERS: [char; 26] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];
const LOWERS: [char; 26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];

#[derive(Debug, PartialEq, Eq)]
pub struct RegexList {
    list: Vec<Regex>
}

impl RegexList {
    pub fn new() -> Self {
        RegexList{list: Vec::new()}
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Regex {
    pattern: Pattern,
    modifier: Option<Modifier> // ex '*', '?', "{2, 5}"
}

impl Regex {
    pub fn len(&self) -> usize {
        &self.pattern.len() + &self.modifier.as_ref().map(|x| x.len()).unwrap_or(0)
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Pattern {
    Literal(String)
}

impl Pattern {
    pub fn len(&self) -> usize {
        match &self {
            Pattern::Literal(s) => s.len()
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Modifier {

}

impl Modifier {
    fn len(&self) -> usize {
        0
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

#[derive(Debug)]
pub enum RegexError {
    Empty,
}

impl TryFrom<&str> for Regex {
    type Error = RegexError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let next_character = US::graphemes(value, true).nth(0);

        let pattern: Option<Pattern> = match next_character {
            Some(ch) => Some(Pattern::Literal(ch.into())),
            None => None
        };

        let modifier = None; // change when needed

        match (pattern, modifier) {
            (Some(pattern), Some(m)) => {
                Ok(Regex {
                    pattern,
                    modifier: m
                })
            },
            (Some(pattern), None) => {
                Ok(Regex {
                    pattern,
                    modifier: None
                })
            }
            (None, _) => Err(Self::Error::Empty),
        }
    }
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let input = "test";
        let result = RegexList::try_from(input).unwrap();
        let correct =  RegexList {
            list: vec![
                Regex {
                    pattern: Pattern::Literal("t".to_string()),
                    modifier: None,
                },
                Regex {
                    pattern: Pattern::Literal(
                        "e".to_string()
                    ),
                    modifier: None,
                },
                Regex {
                    pattern: Pattern::Literal(
                        "s".to_string(),
                    ),
                    modifier: None,
                },
                Regex {
                    pattern: Pattern::Literal(
                        "t".to_string(),
                    ),
                    modifier: None,
                },
            ],
        };
        assert_eq!(result, correct);
    }
}