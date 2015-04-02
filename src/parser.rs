use ast;
use token::Tok;
use token::Tok::*;
use token::OpType::*;

pub fn parse(toks: &[Tok]) -> i64 {
	parse_expr(toks).codegen()
}

fn parse_expr(toks: &[Tok]) -> Box<ast::Node> {
	match toks.first() {
		Some(&Num(num)) => parse_num(box ast::NumNode(num)), toks.tail(),
		_      => panic!(),
	}
}

fn parse_num(first: Box<ast::Node>, others: &[Tok]) -> Box<ast::Node> {
	match others.first() {
		Some(&Op(Add))         => box ast::AddNode(first, parse_expr(others.tail()), false),
		Some(&Op(Sub))         => box ast::AddNode(first, parse_expr(others.tail()), true),
		_ if others.is_empty() => first,
		_                      => panic!(),
	}
}


#[test]
fn test() {
	let input = [Num(7i64), Op(Add), Num(40i64), Op(Sub), Num(5i64)];
	let output = parse(&input);
	print!("{}", output);
	assert!(output==42i64);
} 
