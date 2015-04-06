#![feature(box_syntax)]
#![feature(collections)]
#![feature(core)]
pub mod token;
pub mod lexer;
pub mod ast;
pub mod sym;
pub mod parser;

#[test]
fn test1() {
	let input = "5 + 3 \n  3 -2";
	let tokens = lexer::lex(input);
	let output = parser::parse(tokens.as_slice());
	assert_eq!(output, 1i64);
}
