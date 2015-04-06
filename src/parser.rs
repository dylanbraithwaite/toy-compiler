use sym;
use ast;

use token::Tok;
use token::Tok::*;
use token::OpType::*;


struct Parser {
	pub syms: sym::Table,
}


pub fn parse(toks: &[Tok]) -> i64 {
	let mut p = Parser::new();
	p.parse_block(toks).codegen(&mut p.syms)
}


impl Parser {

	fn new () -> Parser {
		let mut a = Parser {
			syms: sym::Table::new(),
		};
		a
	}


	fn parse_block(&mut self, toks: &[Tok]) -> Box<ast::Node> {
		let raw_lines = toks.split(|tok| *tok == LineEnd);
		let lines : Vec<Box<ast::Node>> = raw_lines.map(|line| self.parse_line(line)).collect();
		box ast::ScopeNode(lines)
	}



	fn parse_line(&mut self, toks: &[Tok]) -> Box<ast::Node> {
		match toks {
			[Id(ref id), Op(Assign), x..] =>  box ast::AssignNode(id.clone(), self.parse_expr(x)),
			_                        =>  self.parse_expr(toks),
		}
	}


	fn parse_expr(&mut self, toks: &[Tok]) -> Box<ast::Node> {
		match toks.first() {
			Some(&Id(ref id))    =>  self.parse_expr_aux(box ast::IdNode(id.clone()), toks.tail()),
			Some(&Num(num))  =>  self.parse_expr_aux(box ast::NumNode(num), toks.tail()),
			_                =>  panic!(),
		}
	}


	fn parse_expr_aux(&mut self, first: Box<ast::Node>, others: &[Tok]) -> Box<ast::Node> {
		match others.first() {
			Some(&Op(Add))         =>  box ast::AddNode(first, self.parse_expr(others.tail()), false),
			Some(&Op(Sub))         =>  box ast::AddNode(first, self.parse_expr(others.tail()), true),
			None                   =>  first,
			_                      =>  panic!(),
		}
	}
}
