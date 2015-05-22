#![feature(box_syntax)]
#![feature(collections)]
#![feature(slice_patterns)]
#![feature(plugin)]
#![plugin(regex_macros)]
extern crate regex;

pub mod lexer;
pub mod ast;
pub mod sym;
pub mod parser;

fn main() {
	loop {
		let mut input = String::new();
		std::io::stdin().read_line(&mut input).ok().expect("Couldn't read from stdin");
		
		let tokens = lexer::lex(&input);
		let output = parser::parse(&tokens);
	
		println!("{}", output);
	}
}
