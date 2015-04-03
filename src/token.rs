use lexer::LexState;
use self::Tok::*;
use self::OpType::*;

#[derive(Debug)]
pub enum Tok {
	Op(OpType),
	Num(i64),
	Id(String),
}

#[derive(Debug)]
pub enum OpType {
	Add,
	Sub,
}

impl Tok {
	pub fn new(data: String, state: LexState) -> Tok {
		match state {
			LexState::Num  => Num(data.parse().unwrap()),
			LexState::Add  => Op(Add),
			LexState::Sub  => Op(Sub),
			_              => panic!(),
		}
	}
}
