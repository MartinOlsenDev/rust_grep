use std::io::Read;
use std::{env, io};
use rust_grep::input::Input;
use rust_grep::regex::regex_list::RegexList;

fn main() {
    let regex_input = env::args().nth(1).expect("Requires 2 arguments.");
    let regex_input = RegexList::try_from(regex_input.as_str()).expect("Error parsing regex input.");

    let text_input: Input = env::args().nth(2)
        .map(|x| Input::from(x))
        .unwrap_or_else(|| {
            let mut input = String::new();
            io::stdin().read_to_string(&mut input).unwrap();
            Input::Text(input)
        });


    dbg!(regex_input, text_input);
}