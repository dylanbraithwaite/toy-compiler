#![feature(box_syntax)]
#![feature(collections)]
#![feature(core)]
#![feature(slice_patterns)]
pub mod token;
pub mod lexer;
pub mod ast;
pub mod sym;
pub mod parser;

#[test]
fn test_assign () {
	let input =
	r"piTimes100 := 314
	  TWO := 2
	  piTimes100 + TWO  ";
	let tokens = lexer::lex(input);
	let output = parser::parse(tokens.as_slice());
	assert_eq!(316, output);
}
