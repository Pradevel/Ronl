pub enum Token {
    Letter(char),
    Digit(char),
    SpecialChar(char),
    OpenBracket,
    CloseBracket,
    Plus,
    Minus,
    Multiply,
    Divide,
}