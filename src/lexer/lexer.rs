pub(crate) mod types;

use types::token::Token;
pub fn lex(input: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    for c in input.chars() {
        let token = if c.is_alphabetic() {
            Token::Letter(c)
        } else if c.is_digit(10) {
            Token::Digit(c)
        } else {
            match c {
                '(' => Token::OpenBracket,
                ')' => Token::CloseBracket,
                '+' => Token::Plus,
                '-' => Token::Minus,
                '*' => Token::Multiply,
                '/' => Token::Divide,
                _ => Token::SpecialChar(c),
            }
        };

        tokens.push(token);
    }
    return tokens;
}