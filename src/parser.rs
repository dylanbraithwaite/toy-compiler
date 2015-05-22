use sym;
use ast;

use lexer::Token;
use lexer::TokenType::*;
use lexer::TokenSlice;


struct Parser {
	pub syms: sym::Table,
}


pub fn parse(toks: TokenSlice) -> i64 {
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


	fn parse_block(&mut self, tokens: TokenSlice) -> Box<ast::Node> {
		let raw_lines = tokens.split(|t| t.ttype == LineEnd).filter(|line| line.len() != 0);

		let lines: Vec<Box<ast::Node>> = raw_lines.map(|line| self.parse_line(line)).collect();

		box ast::ScopeNode(lines)
	}



	fn parse_line(&mut self, toks: TokenSlice) -> Box<ast::Node> {
		match toks {
			[Token{ttype: Id, data: ref id}, Token{ttype: Assign, data: _}, x..] =>  box ast::AssignNode(id.clone(), self.parse_expr(x)),
			_                                          =>  self.parse_expr(toks),
		}
	}


	fn parse_expr(&mut self, toks: TokenSlice) -> Box<ast::Node> {
		match toks {
			[Token{ttype: Id, data: ref id}, ..]     =>  self.parse_expr_aux(box ast::IdNode(id.clone()), toks.tail()),
			[Token{ttype: Num, data: ref num}, ..]   =>  self.parse_expr_aux(box ast::NumNode(num.parse().ok().expect("Invalid num")), toks.tail()),
			_                       =>  panic!(),
		}
	}


	fn parse_expr_aux(&mut self, first: Box<ast::Node>, others: TokenSlice) -> Box<ast::Node> {
		match others {
			[Token{ttype: Add, data: _}, ..] =>  box ast::AddNode(first, self.parse_expr(others.tail()), false),
			[Token{ttype: Sub, data: _}, ..] =>  box ast::AddNode(first, self.parse_expr(others.tail()), true),
			[]                  =>  first,
			_                   =>  panic!(),
		}
	}
}
