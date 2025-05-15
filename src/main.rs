mod tokenizer;

use crate::tokenizer::{Token, TokenType, Tokenizer};

fn main() {
    let sentence: &str = "Hello, my name is Yami-no-karuro and i'm a software developer.";
    let input: String = sentence.to_string();
    let mut tokenizer: Tokenizer = Tokenizer::new(input);
    dbg!(tokenizer);
}
