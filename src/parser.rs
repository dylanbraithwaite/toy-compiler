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
	p.parse_block(toks).codegen()
}


impl Parser {
	fn new () -> Parser {
		let mut a = Parser {
			syms: sym::Table::new(),
		};
		a.syms.syms.insert("ONETHOUSAND".to_string(), sym::Info { val: 1000 });
		a
	}


	fn parse_block(&mut self, toks: &[Tok]) -> Box<ast::Node> {
		let raw_lines = toks.split(|tok| *tok == LineEnd);
		let lines : Vec<Box<ast::Node>> = raw_lines.map(|line| self.parse_expr(line)).collect();
		box ast::ScopeNode(lines)
	}


	fn parse_expr(&mut self, toks: &[Tok]) -> Box<ast::Node> {
		match toks.first() {
			Some(&Id(ref id))    => {
				let val = self.syms.get(id).val.clone();
				self.parse_num(box ast::NumNode(val), toks.tail()) 
			},
			Some(&Num(num))  =>  self.parse_num(box ast::NumNode(num), toks.tail()),
			_                =>  panic!(),
		}
	}


	fn parse_num(&mut self, first: Box<ast::Node>, others: &[Tok]) -> Box<ast::Node> {
		match others.first() {
			Some(&Op(Add))         =>  box ast::AddNode(first, self.parse_expr(others.tail()), false),
			Some(&Op(Sub))         =>  box ast::AddNode(first, self.parse_expr(others.tail()), true),
			None                   =>  first,
			_                      =>  panic!(),
		}
	}
}
