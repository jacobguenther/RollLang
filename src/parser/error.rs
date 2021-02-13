// File: parser/error.rs

use crate::lexer::token::Token;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum ParseError {
	UnexpectedToken(Token),

	ExpectedPunctuation(String),
	ExpectedLiteral(String),
	ExpectedOperator(String),

	MultipleTypesOfExpandingModifiersNotSupported,

	ExpectedInteger,
	DoesNotMatch,

	OutOfBounds,
	UnexpectedTooltip(String),
	Unknown,
}
