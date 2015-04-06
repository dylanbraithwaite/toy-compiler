use lexer::LexState;
use self::Tok::*;
use self::OpType::*;

#[derive(Debug, PartialEq)]
pub enum Tok {
	Op(OpType),
	Num(i64),
	Id(String),
	LineEnd,
}

#[derive(Debug, PartialEq)]
pub enum OpType {
	Add,
	Sub,
}

impl Tok {
	pub fn new(data: String, state: LexState) -> Tok {
		match state {
			LexState::Num     =>  Num(data.parse().unwrap()),
			LexState::Id      =>  Id(data),
			LexState::Add     =>  Op(Add),
			LexState::Sub     =>  Op(Sub),
			LexState::LineEnd =>  LineEnd,
			_                 =>  panic!(),
		}
	}
}
