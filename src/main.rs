use std::io::Read;
use std::{env, io};
use rust_grep::input::Input;

fn main() {
    let regex_input = env::args().nth(1).expect("Requires 2 arguments.");
    let text_input: Input = env::args().nth(2)
        .map(|x| Input::from(x))
        .unwrap_or_else(|| {
            let mut input = String::new();
            io::stdin().read_to_string(&mut input).unwrap();
            Input::Text(input)
        });
    dbg!(regex_input, text_input);
}