// File: lexer/state.rs

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum State {
	Start,
	Whitespace,
	Literal,
	Keyword,
	Digit,
	Done,
}
