use unicode_segmentation::UnicodeSegmentation as US;

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
pub enum Pattern {
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
pub enum Modifier {

}

impl Modifier {
    fn len(&self) -> usize {
        0
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
    fn test_t() {
        let input = "test";
        let result = Regex::try_from(input).unwrap();
        let correct = Regex{pattern: Pattern::Literal("t".to_string()), modifier: None};
        assert_eq!(correct, result);
    }

    #[test]
    fn test_e() {
        let input = "est";
        let result = Regex::try_from(input).unwrap();
        let correct = Regex{pattern: Pattern::Literal("e".to_string()), modifier: None};
        assert_eq!(correct, result);
    }

    #[test]
    fn test_t2() {
        let input = "t";
        let result = Regex::try_from(input).unwrap();
        let correct = Regex{pattern: Pattern::Literal("t".to_string()), modifier: None};
        assert_eq!(correct, result);
    }
}